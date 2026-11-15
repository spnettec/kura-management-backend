# GWT retained client operation receipts v1

2026-11-08 / checkpoint 5dc22468; extends Core contract `lifecycle-client-receipts-v1.md` (40141a77). Overall management redesign remains active.

## Selected integration

Generic New Component and the default generic Services delete backend now retain a client-generated canonical `op-UUID` before the original RPC. The new recorded RPCs use only API1.8 identified plans. Missing capability fails closed; there is no legacy fallback or resend. Web2 imports configuration `[1.8,2)` and requires `kura.configuration.operation.receipts`1.x. Agent is not a dependency. Existing specialized security/cloud/Wire/driver/asset/import/update flows are outside this selected integration and remain pending.

The browser stores only codec version, operation ID, CREATE/DELETE, fixed PID, factory ID and a cancellation-issued flag. No property values, revisions, tokens, passwords, callback or replayable payload are persisted. Version2 reads earlier version1 records. Secure crypto UUID, localStorage and Web Locks are required; reservation/removal/cancellation marking are serialized across tabs. Missing browser facilities or an invalid cache retain a write gate. Storage is per origin/module; this is not account-wide or cross-browser serialization.

Admin-only RPCs validate XSRF before lookup: `createFactoryComponentRecorded`, `deleteFactoryConfigurationRecorded`, `getConfigurationOperationReceipt`, `inspectConfigurationOperation`, `cancelUnreservedConfigurationOperation`. Receipt reads use the independent receipt service only. They do not acquire ConfigurationService, CM, Crypto or a business instance. The original request is never submitted automatically after refresh/re-login/restart.

## Receipt and observation are separate

Original CORE_COMMITTED is Core completion, not business health. An explicit current observation is allowed only for a simple single-PID generic create/delete with no updates or dependent mutations. Creation requires the current Core factory identity to match plus the existing SCR/registry readiness observer. Deletion requires current Core NOT_FOUND, exact PID registry absence and SCR component absence. These are current observations, not a proof of the original post-state or application health. An unrelated later change can affect them. Broader plan shapes fail closed for observation.

A matching REJECTED/CANCELLED can be explicitly acknowledged. CORE_COMMITTED requires the matching current READY/ABSENT observation. RESERVED/UNKNOWN/RECOVERED/NOT_FOUND do not clear the gate. An original successful callback bound to the exact ID may obtain current evidence and acknowledge; a failed/lost callback retains manual review. RECOVERED never becomes original success.

NOT_FOUND is an observation, never permission to retry. A distinct explicit seal action is offered only for an unreserved ID. After its XSRF token, cancellation-issued is persisted under the browser lock before one cancel RPC. Once marked, refresh or a lost reply cannot submit cancellation again; query the original ID. Core's create-only reservation/tombstone arbitrates a concurrent late request. This does not cancel an entered writer or clear unresolved Core journals.

Receipt/token/inspection/cancel callbacks have a 15-second client budget and generation checks that ignore late replies. This neither cancels the original server writer nor establishes a global server deadline. Query failures retain the ID and gate.

## Local proof and limits

226 Web2 JVM tests pass (210 prior +16 new); all GWT locale permutations build with the existing Java11 target. Actual owned no-agent IDEA Kura with loopback response-loss proxy: English create and Chinese delete each submitted once, successful upstream replies discarded, original IDs retained across reload, original CORE_COMMITTED and current instance evidence explicitly reviewed. Create also survived independent JVM restart and UI upgrade. A delayed receipt read exceeded the client budget; its late callback was ignored before a fresh manual read.

Distinct late-create case held the original request before Core, observed NOT_FOUND, explicitly sealed once, then released that original request once. The cancelled reservation survived reload and another independent JVM. No corresponding Core intent or business instance appeared. Proxy response bodies were not retained/decompressed; the exact rejection reason for the late original is not established. No UNKNOWN write was retried and no historical acceptance was replayed.

Four native JVM phases are stopped with all homes, journal records, snapshots, plugin inventory and metadata retained. This round does not prove real post-CM non-idempotent UNKNOWN recovery, independent GWT querying when ConfigurationService fails startup, dedicated domains, arbitrary business health, global deadlines, formal installation/upgrade, multi-tab browser races, every locale/provider or Linux/systemd behavior. The 14 internal pending ledger and three external deferred groups remain separate.


## 2026-11-08 936572c2 post-CM UNKNOWN and authenticated independent receipts

Selected actual single factory creation UNKNOWN/original-ID query/refresh/restart and Core-unavailable authenticated GWT child now verified after fixing persisted role-store availability and skipping normal service-list fetch while a retained ID is pending. Read-only role backend has fixed lowest priority, no defaults and rejects mutations; a distinct healthy native control verifies normal role-store selection/login/services. Useradmin17/Web2 226 pass. Original op-c4cca347-40ff-415b-bc0a-911754697f28 remains UNKNOWN in retained postcm-receipt-45ded12b; no recovery/release/inverse/replay. Full non-idempotent recovery/business handoff, dedicated domains and full14-item ledger remain pending. See `persisted-read-only-identities-v1.md`, `reviews/gwt-postcm-receipts-20261108.md`, and `evidence/gwt-postcm-receipts-20261108.json`.
