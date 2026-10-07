package org.eclipse.kura.management.backend.service;

import java.security.SecureRandom;
import java.time.Duration;
import java.time.Instant;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;

import org.eclipse.kura.management.backend.domain.ConsoleUserEntity;
import org.eclipse.kura.management.backend.repository.ConsoleUserRepository;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.stereotype.Service;

@Service
public class TokenService {

    public record AuthContext(String username, String tenantId, String displayName) {
    }

    public record LoginResult(String token, AuthContext context) {
    }

    private record TokenEntry(AuthContext context, Instant expiresAt) {
    }

    private static final Duration TOKEN_TTL = Duration.ofHours(12);
    private static final int TOKEN_BYTES = 32;

    private final ConsoleUserRepository userRepository;
    private final BCryptPasswordEncoder passwordEncoder = new BCryptPasswordEncoder();
    private final SecureRandom random = new SecureRandom();
    private final Map<String, TokenEntry> tokens = new ConcurrentHashMap<>();

    public TokenService(ConsoleUserRepository userRepository) {
        this.userRepository = userRepository;
    }

    public Optional<LoginResult> login(String username, String password) {
        return userRepository.findByUsernameAndEnabledTrue(username)
                .filter(user -> passwordEncoder.matches(password, user.getPasswordHash()))
                .map(user -> {
                    AuthContext context = toContext(user);
                    String token = newToken();
                    this.tokens.put(token, new TokenEntry(context, Instant.now().plus(TOKEN_TTL)));
                    return new LoginResult(token, context);
                });
    }

    public Optional<AuthContext> validate(String token) {
        if (token == null || token.isBlank()) {
            return Optional.empty();
        }
        TokenEntry entry = this.tokens.get(token);
        if (entry == null || entry.expiresAt().isBefore(Instant.now())) {
            this.tokens.remove(token);
            return Optional.empty();
        }
        return Optional.of(entry.context());
    }

    public void logout(String token) {
        if (token != null) {
            this.tokens.remove(token);
        }
    }

    private AuthContext toContext(ConsoleUserEntity user) {
        return new AuthContext(user.getUsername(), user.getTenantId(), user.getDisplayName());
    }

    private String newToken() {
        byte[] bytes = new byte[TOKEN_BYTES];
        this.random.nextBytes(bytes);
        StringBuilder hex = new StringBuilder(bytes.length * 2);
        for (byte b : bytes) {
            hex.append(Character.forDigit((b >> 4) & 0xF, 16)).append(Character.forDigit(b & 0xF, 16));
        }
        return hex.toString();
    }
}
