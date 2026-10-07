package org.eclipse.kura.management.backend.api;

import java.util.Map;

import org.eclipse.kura.management.backend.api.dto.LoginRequest;
import org.eclipse.kura.management.backend.service.TokenService;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/auth")
public class AuthController {

    private final TokenService tokenService;

    public AuthController(TokenService tokenService) {
        this.tokenService = tokenService;
    }

    @PostMapping("/login")
    public ResponseEntity<Map<String, Object>> login(@RequestBody LoginRequest request) {
        return tokenService.login(request.username(), request.password())
                .<ResponseEntity<Map<String, Object>>>map(result -> ResponseEntity.ok(Map.of(
                        "token", result.token(),
                        "username", result.context().username(),
                        "displayName", result.context().displayName() == null
                                ? result.context().username()
                                : result.context().displayName(),
                        "tenantId", result.context().tenantId())))
                .orElseGet(() -> ResponseEntity.status(401).body(Map.of("error", "invalid credentials")));
    }

    @PostMapping("/logout")
    public ResponseEntity<Void> logout(@RequestHeader(value = "Authorization", required = false) String header) {
        if (header != null && header.startsWith("Bearer ")) {
            tokenService.logout(header.substring("Bearer ".length()));
        }
        return ResponseEntity.noContent().build();
    }
}
