# Original GWT firewall transactions v1

Implementation round 056e9101. Final affected contracts passed (38 tests), as did
full frontend compilation and final server packaging. Browser input is blocked
even on a separate static HTML button. Browser transactions, IDEA and subsequent
VM validation remain internal pending; prepared IDEA profiles have not launched.
See [the round review](reviews/firewall-gwt-transactions-20261107.md).
This document does not claim release or completed system acceptance.

The optional firewall owner profile uses the existing NETWORK_ADMIN permission,
existing authentication/password gate and XSRF checks. No agent installation is
required for the original Kura profile. The existing firewall-menu availability
setting remains authoritative. A configured agent provider adds six rule editors
and a default-policy tab, with explicit confirmation/rollback controls.

Each read produces an opaque edit ID backed by one immutable server baseline.
Another panel's read never advances that baseline. A valid prepare consumes all
older session edits before the one write call. A delayed old draft cannot apply
after a later transaction has rolled back to the same revision. A fresh edit is
required for further writes; reset only reads confirmed state.

The authenticated HTTP session generates and holds the owner secret, original
change/request, transaction token and bounded outcome records. Browser DTOs carry
only edit/correlation IDs, typed legacy rule fields, policies and safe projected
states. They do not expose owner secrets, tokens or provider candidate maps.

The six editors replace only their own family/category. Other fields and automatic
NAT remain intact. Original rule references preserve existing rule priority even
when a modal editor appends its replacement DTO; duplicate/foreign references are
rejected. Manual NAT preserves its original rule type and nullable protocol.
Prefix interfaces, named/numeric NAT protocols and both family defaults remain
subject to the strict provider/agent V3 contract. New manual NAT rows precede the
first automatic row without reordering surviving original rows.

Prepare is temporary application, not confirmation. The UI disables new edits
while a transaction is active, uncertain or observation is unavailable. The user
must explicitly acknowledge reconnecting through the affected management path
before confirming. Rollback is separate. The checkbox is a user acknowledgement;
a test loopback proxy is not proof of a changed management connection.

Lost prepare/confirm/rollback replies retain the original server intent. Polling
and the query button only call the original-outcome API; they never reissue a
write or refresh the intent to a newer baseline. Missing, unrelated or malformed
outcomes remain UNKNOWN and keep writes blocked. Explicit INVALID_ARGUMENT from
prepare is a known validation rejection before durable preparation; the draft
can be refreshed for a new user action. No other failure is treated as evidence
that a write did not occur.

Both original PID projections and Core ConfigurationService snapshot/update/
rollback integration are part of the actual full-Kura validation for this round.
The typed provider, independent UID grant and durable journal contracts remain
those of `firewall-kura-provider-contract-20261107.md`. NM/other-writer adoption,
center tasks/remote-path verification, actual early boot and formal release are
separate pending delivery work.
