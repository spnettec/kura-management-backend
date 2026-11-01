//! Live TLS contract tests; explicitly run after phase1-dev.py up.
use std::{fs, path::PathBuf, time::Duration};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    Code,
    transport::{Certificate, ClientTlsConfig, Endpoint, Identity},
};
use yofc_management_protocol::v1::{agent_frame, control_client::ControlClient, *};

fn dev() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.phase1-dev")
}
async fn probe(
    cert_name: &str,
    gateway: &str,
    version: u32,
) -> Result<tonic::Response<tonic::Streaming<ConsoleFrame>>, tonic::Status> {
    let d = dev();
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(fs::read(d.join("ca.pem")).unwrap()))
        .identity(Identity::from_pem(
            fs::read(d.join(format!("{cert_name}.pem"))).unwrap(),
            fs::read(d.join(format!("{cert_name}.key"))).unwrap(),
        ));
    let channel = Endpoint::from_static("https://localhost:18444")
        .tls_config(tls)
        .unwrap()
        .connect()
        .await
        .unwrap();
    let mut client = ControlClient::new(channel);
    let (tx, rx) = tokio::sync::mpsc::channel(2);
    tx.send(AgentFrame {
        payload: Some(agent_frame::Payload::Hello(Hello {
            gateway_id: gateway.into(),
            protocol_major: version,
            agent_version: "contract-test".into(),
            boot_id: "test".into(),
            capabilities: vec![],
        })),
    })
    .await
    .unwrap();
    client.connect(ReceiverStream::new(rx)).await
}
#[tokio::test]
#[ignore = "requires isolated phase1-dev stack"]
async fn registered_ca_is_insufficient_without_gateway_binding() {
    assert_eq!(
        probe("unregistered", "dev-gateway", 1)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    assert_eq!(
        probe("agent", "forged-gateway", 1)
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    assert_eq!(
        probe("agent", "dev-gateway", 99).await.unwrap_err().code(),
        Code::FailedPrecondition
    );
}
#[tokio::test]
#[ignore = "requires isolated phase1-dev stack"]
async fn no_client_certificate_and_wrong_hostname_are_rejected() {
    let ca = Certificate::from_pem(fs::read(dev().join("ca.pem")).unwrap());
    let result = Endpoint::from_static("https://localhost:18444")
        .tls_config(
            ClientTlsConfig::new()
                .ca_certificate(ca.clone())
                .domain_name("wrong.invalid"),
        )
        .unwrap()
        .connect()
        .await;
    assert!(result.is_err(), "hostname checking must remain enabled");
    let result = Endpoint::from_static("https://localhost:18444")
        .tls_config(ClientTlsConfig::new().ca_certificate(ca))
        .unwrap()
        .connect()
        .await;
    if let Ok(channel) = result {
        let mut client = ControlClient::new(channel);
        let (_tx, rx) = tokio::sync::mpsc::channel::<AgentFrame>(1);
        assert!(
            tokio::time::timeout(
                Duration::from_secs(5),
                client.connect(ReceiverStream::new(rx))
            )
            .await
            .unwrap()
            .is_err()
        );
    }
}

#[tokio::test]
#[ignore = "requires isolated phase1-dev stack"]
async fn expired_client_certificate_is_rejected() {
    let d = dev();
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(fs::read(d.join("ca.pem")).unwrap()))
        .identity(Identity::from_pem(
            fs::read(d.join("expired.pem")).unwrap(),
            fs::read(d.join("expired.key")).unwrap(),
        ));
    if let Ok(channel) = Endpoint::from_static("https://localhost:18444")
        .tls_config(tls)
        .unwrap()
        .connect()
        .await
    {
        let mut client = ControlClient::new(channel);
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        tx.send(AgentFrame {
            payload: Some(agent_frame::Payload::Hello(Hello {
                gateway_id: "dev-gateway".into(),
                protocol_major: 1,
                agent_version: "test".into(),
                boot_id: "test".into(),
                capabilities: vec![],
            })),
        })
        .await
        .unwrap();
        assert!(
            tokio::time::timeout(
                Duration::from_secs(5),
                client.connect(ReceiverStream::new(rx))
            )
            .await
            .unwrap()
            .is_err()
        );
    }
}
