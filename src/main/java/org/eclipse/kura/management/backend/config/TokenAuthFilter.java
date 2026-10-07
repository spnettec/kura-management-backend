package org.eclipse.kura.management.backend.config;

import java.io.IOException;

import org.eclipse.kura.management.backend.service.TokenService;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.web.util.matcher.AntPathRequestMatcher;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

/**
 * Extracts the Bearer token, and on success populates the security context with the
 * {@link TokenService.AuthContext} as principal. Requests without a valid token simply
 * leave the context empty; the downstream authorizeHttpRequests rules turn that into 401.
 */
@Component
public class TokenAuthFilter extends OncePerRequestFilter {

    private static final String BEARER_PREFIX = "Bearer ";
    private static final AntPathRequestMatcher LOGIN_MATCHER = new AntPathRequestMatcher("/api/auth/login");

    private final TokenService tokenService;

    public TokenAuthFilter(TokenService tokenService) {
        this.tokenService = tokenService;
    }

    @Override
    protected boolean shouldNotFilter(HttpServletRequest request) {
        return LOGIN_MATCHER.matches(request);
    }

    @Override
    protected void doFilterInternal(HttpServletRequest request, HttpServletResponse response, FilterChain chain)
            throws ServletException, IOException {
        String header = request.getHeader("Authorization");
        if (header != null && header.startsWith(BEARER_PREFIX)) {
            tokenService.validate(header.substring(BEARER_PREFIX.length())).ifPresent(context -> {
                UsernamePasswordAuthenticationToken authentication = new UsernamePasswordAuthenticationToken(
                        context, null, java.util.List.of());
                SecurityContextHolder.getContext().setAuthentication(authentication);
            });
        }
        chain.doFilter(request, response);
    }
}
