# Kura firewall provider and local transaction contract

Round a44da9fc builds on the V3 rule semantics and writer ownership evidence.
The opt-in provider and public API can now be installed by the standard
Networking package. The original GWT transaction editing and confirmation
workflow and end-to-end installed ownership transition remain pending.

## Activation and independent startup

- The standard Networking deb installs the Root-owned
  `org.eclipse.kura.firewall.agent` bundle beside the legacy facade.
- Activation requires `kura.firewall.provider=agent`, reservation of both original
  IPv4/IPv6 firewall PIDs in `kura.configuration.external.pids`, and the local
  NM writer observer. With the legacy bundle installed, the observer must
  attest both durable fences and drained writers. Both firewall facades must
  be free of competing registrations.
- NetworkManager is retained. This provider does not select Netplan, remove NM,
  change forwarding sysctls, or adopt an existing system firewall automatically.
- The OSGi package import for the Java bridge is optional. With the Agent package
  absent, default Core startup still resolves the bundle and selects the legacy
  facade. When the typed profile is selected, the paired Java bridge service is
  a required dynamic OSGi reference. The bridge can be present with the Agent
  daemon stopped. Calls over a disconnected bridge report unavailable.
- The package leaves the active profile selection to the existing startup and
  handover gates. See the [installed package checkpoint](checkpoints/20261116/packaged-firewall-provider-checkpoint.json)
  for the Debian ARM64 combinations and the refusal without original fences.

## Authorization and requests

`firewall.kura.*` is accepted only on the registered bridge connection of the
configured Kura OS UID. The ordinary root/admin `firewall.*` API is separate and
is never delegated to that connection.

Both `AGENT_FIREWALL_WRITE=1` and the new, default-off
`AGENT_KURA_FIREWALL_WRITE=1` are required for prepare and confirm. Read-only view
and original-outcome queries do not require these grants. Explicit rollback of
an owned, unconfirmed transaction remains available after grant revocation,
subject to the shared system-change admission boundary.

The public Core API is `org.eclipse.kura.net.firewall.transaction` version 1.0.
It uses detached JDK values and original typed firewall configurations, without
Gson or networking implementation classes at the API boundary. A typed snapshot
contains both IP families and metadata from one coherent confirmed revision.

- `firewall.kura.view`: schema 1, confirmed revision/source digest, initialized and
  projectable flags, write grant, V3 candidate or null, and a token-free transaction.
- `firewall.kura.prepare`: ownerKey (256-bit session secret), expectedRevision,
  sourceDigest, confirmSeconds (30–300), complete V3 candidate. The RPC correlation
  ID is the original request ID, limited to 64 approved ASCII characters.
- Prepare performs durable preparation and temporary application under one shared
  system-change lock. Its success is `WAIT_CONFIRM`, never an implicit commit.
- `firewall.kura.confirm` / `rollback`: transaction ID, ownerKey and secret token.
  Confirmation is a separate explicit caller action. The original UI/REST layer
  remains responsible for authenticating the user and verifying the affected path.
- `firewall.kura.outcome`: originalRequestId and the retained original change;
  use a new read RPC ID. UID, owner digest and canonical typed-input digest must
  match. The query returns original durable evidence and the owned token, with
  no nft invocation, write admission, repair, apply or confirmation.

Neither ownerKey nor confirmationToken belongs in browser URLs, logs, audit
payloads, Kura snapshots or the public view. The future GWT integration must keep
them in the authenticated HTTP session and freeze the draft's own baseline.

## Persistence, UNKNOWN and confirmed projections

New journals have an additive optional Kura binding. New confirmed records retain
the typed candidate as well as the compiled text and revision. Old v1/v2 journals
remain readable. Existing confirmed data without a matching typed V3 candidate
is reported unprojectable; it is not converted into an empty/default rule set.

Before any later prepare can replace a settled Kura journal, its complete outcome
is archived durably under a hashed UID/request key. Archives are retained. A
duplicate original prepare is rejected with `OUTCOME_UNKNOWN` and directs the
caller to the read-only outcome API; it never applies the candidate again.
Missing durable evidence also remains `OUTCOME_UNKNOWN`. Lost or malformed write
responses map to `UNKNOWN` in Java. No query or provider call retries a write.

Read observations serialize on the same OS lock without admitting a writer, so
retained `RECOVERY_REQUIRED` journals keep blocking writes while view/outcome
queries remain available. The original shared write admission is preserved.

The provider implements both original firewall service PIDs and exposes confirmed
legacy property names, including the IPv6 names. Generic configuration updates
must match confirmed values. Historical snapshot rollback does not rewind the
independent firewall owner. Legacy setters and raw flooding rules are closed;
they cannot supply the explicit confirmation context.

The Java mapper accepts omitted nullable NAT protocol/ruleType fields with the
same meaning as explicit null, matching Rust V3. Family, rule count, protocol,
range, automatic-rule constraints and unknown-field rejection remain strict.

## Remaining delivery boundaries

The GWT write UX/RPC, per-browser draft ownership and original-request UNKNOWN
reconciliation; NM automatic NAT ownership, other-manager adoption and retirement,
center task integration, affected management-path proof, the installed positive
ownership transition, and remaining package, Docker and release combinations
need their own acceptance. Physical-NM validation remains external deferred.
