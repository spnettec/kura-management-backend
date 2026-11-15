# One self-owned view per logical Kura PID

2026-11-08,8c51aa2d. Multiple service registrations may share a Kura PID, including separate SelfConfiguringComponent and ManagedService registrations. Core must not let an unrelated registration erase a valid self-owned view or replace it with stored CM fallback because of enumeration order.

The private read result now distinguishes actual SCC output from the existing unavailable-reference CM fallback. A valid SCC view takes priority; non-SCC null results do not erase it. CM fallback remains available when no SCC can be acquired, and an invalid SCC still rejects the read rather than silently selecting fallback. Existing provider budgets, acquisition/release lifetime, logical-password normalization and multiple-actual-SCC behavior are unchanged. This does not certify arbitrary duplicate actual SCC owners, lifecycle races or all provider types.

Filtered enumeration still evaluates the filter against each registration first. Once a registration qualifies, Core reads one logical configuration for that PID; additional matching registrations do not duplicate it or execute the provider again. A nonmatching earlier registration does not consume the PID or hide a later matching registration. No CM update, factory/lifecycle action or new write authority is introduced.

Six regression cases and Core259 pass. Two fresh full IDEA scenes show both actual registration orders and original GWT reads; the intermediate scene exposed four menu items for two PIDs, and the final scene has exactly two. Native target CM count remains0, with zero non-null ManagedService callbacks and zero dispatch intents. Ordinary startup's two expected ManagedService.updated(null) initialization notifications are explicitly distinguished from business updates.
