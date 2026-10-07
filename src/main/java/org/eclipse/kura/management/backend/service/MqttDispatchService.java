package org.eclipse.kura.management.backend.service;

import java.nio.charset.StandardCharsets;
import java.time.Instant;
import java.util.HashMap;
import java.util.Map;

import org.eclipse.kura.management.backend.domain.CommandEntity;
import org.eclipse.paho.client.mqttv3.IMqttDeliveryToken;
import org.eclipse.paho.client.mqttv3.MqttCallbackExtended;
import org.eclipse.paho.client.mqttv3.MqttClient;
import org.eclipse.paho.client.mqttv3.MqttConnectOptions;
import org.eclipse.paho.client.mqttv3.MqttException;
import org.eclipse.paho.client.mqttv3.MqttMessage;
import org.eclipse.paho.client.mqttv3.persist.MemoryPersistence;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import jakarta.annotation.PostConstruct;

@Service
public class MqttDispatchService {

    private static final Logger logger = LoggerFactory.getLogger(MqttDispatchService.class);

    private final CommandService commandService;
    private final GatewayService gatewayService;
    private final GatewayCredentialService gatewayCredentialService;
    private final ConfigHistoryService configHistoryService;
    private final ObjectMapper objectMapper;
    private final String brokerUri;
    private final String clientId;
    private final String username;
    private final String password;
    private MqttClient client;
    private MqttConnectOptions connectOptions;

    public MqttDispatchService(
            CommandService commandService,
            GatewayService gatewayService,
            GatewayCredentialService gatewayCredentialService,
            ConfigHistoryService configHistoryService,
            ObjectMapper objectMapper,
            @Value("${mqtt.broker-uri}") String brokerUri,
            @Value("${mqtt.client-id}") String clientId,
            @Value("${mqtt.username:}") String username,
            @Value("${mqtt.password:}") String password) {
        this.commandService = commandService;
        this.gatewayService = gatewayService;
        this.gatewayCredentialService = gatewayCredentialService;
        this.configHistoryService = configHistoryService;
        this.objectMapper = objectMapper;
        this.brokerUri = brokerUri;
        this.clientId = clientId;
        this.username = username;
        this.password = password;
    }

    @PostConstruct
    public void initClient() {
        try {
            this.client = new MqttClient(brokerUri, clientId, new MemoryPersistence());
            this.client.setCallback(new MqttCallbackExtended() {
                @Override
                public void connectComplete(boolean reconnect, String serverURI) {
                    onConnected(reconnect, serverURI);
                }

                @Override
                public void connectionLost(Throwable cause) {
                    logger.warn("MQTT connection lost: {}", cause == null ? "unknown" : cause.getMessage());
                }

                @Override
                public void messageArrived(String topic, MqttMessage message) {
                    handleInboundMessage(topic, new String(message.getPayload(), StandardCharsets.UTF_8));
                }

                @Override
                public void deliveryComplete(IMqttDeliveryToken token) {
                    // No-op.
                }
            });

            MqttConnectOptions options = new MqttConnectOptions();
            options.setAutomaticReconnect(true);
            options.setCleanSession(true);
            if (username != null && !username.isBlank()) {
                options.setUserName(username);
                options.setPassword(password == null ? new char[0] : password.toCharArray());
            }
            this.connectOptions = options;
        } catch (MqttException e) {
            logger.warn("Unable to create MQTT client for {}: {}", brokerUri, e.getMessage());
        }
    }

    /**
     * Paho automaticReconnect only engages after a first successful connect; if the broker is
     * down at startup this scheduled probe keeps retrying until the first connect succeeds.
     */
    @Scheduled(fixedDelay = 10_000)
    public void ensureConnection() {
        if (this.client == null || this.connectOptions == null || this.client.isConnected()) {
            return;
        }
        try {
            this.client.connect(this.connectOptions);
        } catch (MqttException e) {
            // 32110 = connect already in progress (e.g. automatic reconnect racing with this probe)
            logger.debug("MQTT connect retry pending: {}", e.getMessage());
        }
    }

    public void publishCommand(CommandEntity command) {
        ensureConnected();
        String topic = "mgmt/%s/%s/cmd".formatted(command.getTenantId(), command.getGatewayId());
        Map<String, Object> envelope = new HashMap<>();
        envelope.put("id", command.getCommandId());
        envelope.put("type", command.getType());
        envelope.put("ts", System.currentTimeMillis());
        envelope.put("version", 1);
        envelope.put("tenantId", command.getTenantId());
        envelope.put("gatewayId", command.getGatewayId());
        if (command.getCorrelationId() != null) {
            envelope.put("correlationId", command.getCorrelationId());
        }
        envelope.put("payload", command.getRequestPayload());
        envelope.put("meta", Map.of("source", "backend"));

        try {
            byte[] payload = objectMapper.writeValueAsBytes(envelope);
            this.client.publish(topic, payload, 1, false);
            commandService.markSent(command.getCommandId());
        } catch (Exception e) {
            throw new IllegalStateException("Unable to publish MQTT command", e);
        }
    }

    private void handleInboundMessage(String topic, String payload) {
        // Paho disconnects the client if messageArrived throws, so every inbound path
        // (unknown gateway, unknown command id, malformed json, credential store failure)
        // must be absorbed here.
        try {
            if (topic.endsWith("/reply")) {
                handleReply(payload);
                return;
            }
            if (topic.endsWith("/status")) {
                handleStatusReport(payload);
                return;
            }
            logger.debug("Ignoring MQTT message on topic {}", topic);
        } catch (Exception e) {
            logger.warn("Error handling MQTT message on topic {}: {}", topic, e.getMessage());
        }
    }

    private void handleReply(String messagePayload) throws Exception {
        Map<String, Object> reply = objectMapper.readValue(messagePayload, new TypeReference<>() {
        });
        String tenantId = String.valueOf(reply.get("tenantId"));
        String gatewayId = String.valueOf(reply.get("gatewayId"));
        if (!isAuthenticatedMessage(reply, tenantId, gatewayId)) {
            logger.warn("Rejecting unauthenticated MQTT reply from gateway {}", gatewayId);
            return;
        }
        gatewayCredentialService.markUsed(gatewayId);
        reply.remove("auth");
        String commandId = String.valueOf(reply.get("id"));
        String status = String.valueOf(reply.get("status"));
        if ("success".equals(status)) {
            commandService.markSuccess(commandId, reply);
            this.configHistoryService.recordFromReply(commandId);
            return;
        }

        Object replyPayload = reply.get("payload");
        String errorCode = "INTERNAL_ERROR";
        String errorMessage = "Command failed";
        if (replyPayload instanceof Map<?, ?> errorPayload) {
            Object code = errorPayload.get("errorCode");
            Object message = errorPayload.get("errorMessage");
            if (code != null) {
                errorCode = String.valueOf(code);
            }
            if (message != null) {
                errorMessage = String.valueOf(message);
            }
        }
        commandService.markFailed(commandId, errorCode, errorMessage, reply);
    }

    private void handleStatusReport(String messagePayload) throws Exception {
        Map<String, Object> report = objectMapper.readValue(messagePayload, new TypeReference<>() {
        });
        String tenantId = String.valueOf(report.get("tenantId"));
        String gatewayId = String.valueOf(report.get("gatewayId"));
        if (!isAuthenticatedMessage(report, tenantId, gatewayId)) {
            logger.warn("Rejecting unauthenticated MQTT status from gateway {}", gatewayId);
            return;
        }
        Instant ts = Instant.ofEpochMilli(((Number) report.getOrDefault("ts", System.currentTimeMillis())).longValue());

        Map<String, Object> payload = Map.of();
        Object reportPayload = report.get("payload");
        if (reportPayload instanceof Map<?, ?> values) {
            payload = values.entrySet().stream()
                    .collect(HashMap::new, (map, entry) -> map.put(String.valueOf(entry.getKey()), entry.getValue()),
                            HashMap::putAll);
        }

        Object onlineValue = payload.get("online");
        boolean online = onlineValue == null || Boolean.parseBoolean(String.valueOf(onlineValue));
        gatewayService.recordStatusReport(tenantId, gatewayId, ts, online, payload);
        gatewayCredentialService.markUsed(gatewayId);
    }

    private boolean isAuthenticatedMessage(Map<String, Object> message, String tenantId, String gatewayId) {
        Object auth = message.get("auth");
        if (!(auth instanceof Map<?, ?> authValues)) {
            return false;
        }
        Object secret = authValues.get("secret");
        return gatewayCredentialService.verifySecret(tenantId, gatewayId, secret == null ? null : String.valueOf(secret));
    }

    private void ensureConnected() {
        if (this.client == null || !this.client.isConnected()) {
            throw new IllegalStateException("MQTT client is not connected");
        }
    }

    private void onConnected(boolean reconnect, String serverURI) {
        try {
            this.client.subscribe("mgmt/+/+/reply", 1);
            this.client.subscribe("mgmt/+/+/status", 1);
            logger.info("MQTT {} to {} as {}", reconnect ? "reconnected" : "connected", serverURI, clientId);
        } catch (MqttException e) {
            throw new IllegalStateException("Unable to subscribe MQTT topics", e);
        }
    }
}
