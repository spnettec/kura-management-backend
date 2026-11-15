# Core configuration operation receipts v1

2026-11-08, checkpoint 40141a77. This implements the Core protocol foundation. GWT, center/agent and other domain callers do not yet submit or reconcile these identified operations. Their existing UNKNOWN gates remain in force.

## Admission and compatibility

`ConfigurationRevisionService.applyConfigurationPlanIfUnchanged(operationId, ...)` accepts an `op-` prefix followed by a canonical lowercase UUID. The caller must create and retain its ID before sending a request. Both ordinary and deletion-closure overloads are available. There is no automatic ID generation, fallback to a legacy writer or retry. Every reuse of a reserved ID is refused, including identical arguments and previously committed/rejected/cancelled IDs. A legacy provider's default overload returns UNSUPPORTED.

The public configuration package is 1.8.0. Core imports `[1.8,2)` and provides `kura.configuration.operation.receipts` 1.0.0. The new receipt DTO is usable from Java 11 source compiled by the workspace JDK 21, without depending on `java.lang.Record`. Full distribution, installation, upgrade, hot refresh and downgrade enforcement remain pending. Older unidentified entrypoints retain their existing contract and do not gain client receipts.

Core detaches and validates a bounded plan, then forces a CREATE_NEW value-free request reservation before preflight. Before its first admitted writer it forces the existing journal intent and a dispatch record binding that exact intent and request. The intent UUID is the client ID's UUID. Only then can the existing plan writer run. A preflight exception may produce a forced REJECTED decision only before any intent attempt. After an intent attempt, failure cannot produce a reassuring pre-dispatch rejection. Existing unresolved-intent startup and ordinary-write gates remain authoritative.

The reservation records IDs, created PID/factory identities, updated/deleted PIDs and affected scope. It stores no property values, passwords, revision tokens or property-value digests. SHA bindings cover only these value-free records and the existing value-free intent. Reservations and cancellation share the same CREATE_NEW arbitration. The owner thread's single preflight permit cannot be consumed by a reentrant legacy writer.

## Query and cancellation

`ConfigurationOperationReceiptService` is a separate immediate DS service with only a SystemService reference. It does not require ConfigurationService/CM/Crypto/agent/business service activation, open snapshots, call a provider or replay a writer. This API supplies no remote ACL: any future transport must enforce authentication, authorization and XSRF independently.

| State | Meaning and next action |
| --- | --- |
| NOT_FOUND | No reservation is visible. An original request may still be arriving or prevalidating. Never resend on this result. |
| RESERVED | Reservation exists; preflight may be running or interrupted. Do not resend. |
| REJECTED | A matching pre-dispatch decision was forced. Obtain a fresh view before a separate explicit business action; never reuse this ID. |
| CORE_COMMITTED | Original intent/dispatch/completion match: the covered Core writer and its snapshot completed. This is not asynchronous factory activation, provider readiness or business health. |
| UNKNOWN | Dispatch may have occurred, or evidence is unsafe, corrupt, incomplete or contradictory. Preserve the ID and scene; no retry or compensation inferred from this query. |
| RECOVERED | A matching later recovery decision/exact published release resolved the intent. Original business success is still unknown. |
| CANCELLED | An entirely unreserved ID was durably sealed against a late identified submission. No CM mutation occurred. |

`cancelUnreservedOperation(id)` only seals an entirely unrecorded ID. It cannot cancel RESERVED/UNKNOWN/dispatched operations, delete reservations or release a journal gate. A racing request reservation or cancellation has exactly one winner. Orphan intent/decision/dispatch/rejection/release evidence refuses cancellation. Future UI may expose this only as an explicit action after a read, never as an automatic consequence of NOT_FOUND or a network timeout. It must not retain or resend an old property payload.

Queries are independent of the writer lock and may observe partial files; those observations become UNKNOWN rather than NOT_FOUND or success. Canonical IDs, private owner-only directories/files, NOFOLLOW checks, record bounds and exact SHA/EOF validation apply. A release link takes precedence over nominal completion just as startup does. Missing or changed bound publication cannot become original success. Legacy resolved decisions remain readable as RECOVERED. Ordinary startup does not infer that auxiliary receipt corruption reverses an already valid original completion; unresolved dispatch intents retain their existing global gate.

## Evidence and remaining work

API 24 and Core 253 tests pass/install; the unchanged offline recovery module passes 62 tests/install against the new pair. Fifteen new Core tests cover complete/rejected/unknown operations, duplicate IDs across restart, lost-response query without replay, fresh JVM startup gating, read during an active writer, reentrant preflight writer refusal, cancellation race/late arrival, unsafe/partial/oversized records, exact recovery publication, legacy-provider refusal and Java 11 source compatibility.

One new targeted Equinox/SCR integration test uses the actual packaged Core/API. It omits CM and Crypto providers and the old scenario bundle. ConfigurationService cannot activate, while the production receipt DS registers, reads an unfinished journal protocol fixture as UNKNOWN and seals an unrelated missing ID. That interrupted intent was deliberately constructed without a CM writer; it is not a real non-idempotent creation/deletion crash or a whole-business recovery acceptance. The separate fresh-JVM unit uses the actual Core journal reader/gate, not a deployed gateway.

GWT must still persist only correlation metadata before sending, carry the ID through create/delete and all required domain paths, distinguish Core commit from readiness, query the original receipt after lost replies/reload, and retain the write gate until explicit reconciliation. No existing UNKNOWN may be resent to obtain a receipt. Real creation/deletion lost replies, non-idempotent partial providers, business-result reconciliation, all server deadlines and legacy/direct-CA ownership remain internal pending. The full 14-item parent ledger and external deferred groups remain authoritative.


## 2026-11-08 5dc22468 selected GWT client integration

The earlier Core-only client status is superseded for generic New Component and default generic Services delete only. Retained value-free IDs, recorded RPCs, independent original receipt queries, separate current readiness/absence inspection and explicit unreserved sealing are now integrated and locally verified after lost response/reload/restart. See `gwt-client-receipts-v1.md` and `reviews/gwt-client-receipts-20261108.md`. Broader domain, real post-CM UNKNOWN/business recovery, server deadlines and formal deployment remain pending.
