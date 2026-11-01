package org.eclipse.kura.management.backend.api;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;

/** Serves the browser application at its client-side routes. */
@Controller
public class ConsolePageController {

    @GetMapping({"/", "/login", "/gateways", "/gateways/{gatewayId}", "/commands", "/commands/{commandId}"})
    public String index() {
        return "forward:/index.html";
    }
}
