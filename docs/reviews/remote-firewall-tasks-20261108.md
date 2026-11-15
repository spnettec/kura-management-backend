# f07d2c1a — remote firewall task implementation in progress

2026-11-08, after sealed cc91d577. Goal remains active. This batch is not sealed
as a completed remote firewall delivery.

All18 repository HEADs/branches and previous dirty entries were checked at entry.
The immediately preceding195628-pin seal is reused; no redundant historical
acceptance is run. Five edited source files have exact before copies. The first checkpoint
changes were additive protocol operations30–33, typed bounded V3 wire admission,
current center constructor compatibility, and agent cross-compiler tests. The
HTTP operation allowlist did not expose these new operations at that stage. At the
first checkpoint the agent dispatcher was also absent; the next stage below adds
its explicit default-off grant and conditional capability.

Protocol13 tests pass; strict protocol Clippy passes; current center cargo check
passes. Two agent tests pass against the actual existing V3 compiler, preserving
the complete retained candidate and null/all/numeric NAT protocol distinctions.
These are pure contracts, not nft, CM, browser, mTLS or system validation. The
first protocol test invocation produced an unused-import warning, subsequently
removed before the final test and strict check. Initial shell invocations lacked
cargo on PATH or used wrong working-directory-relative paths; they ran no tests.
The exact toolchain1.95 binary path is used for final commands.

Agent dependencies were APFS-cloned from the prior build into a new private
batch target; the previous test executable is preserved. No production JVM/daemon/VM/service
or real firewall transaction is started this round. No commit/push/reset/clean.

The next implementation stage delivered the agent remote dispatcher and a
default-off `AGENT_FIREWALL_REMOTE` grant, separate from the global write and
Kura UID grants. Value-free execution authority binds center/gateway generation
and full command hash before claim, including checks before terminal receipts
can disclose a token. Private domain journal/history bind prepare and action
identities. Locally minted post-handshake connection time/nonce must be new and
strictly after application completion before confirm. Old network sessions retain
their format. Entered writer errors stay UNKNOWN; read-only proof uses no nft or
repair. A failed, undecided confirmation followed by explicit rollback resolves
FAILED, never successful. Current source is not deployed to original services.

Full agent regression passed238/ignored12 before that final failed-confirmation
proof branch and stronger terminal-token test. Final affected scope passed39/
ignored1, and strict all-target agent Clippy passed. These new tests use fake nft
and private SQLite; they do not prove real mTLS reconnection or system traffic.
The initial remote build failed because the shared observation helper was
private to its sibling module; its visibility is now restricted to the firewall
parent. No behavioral read-lock change. Historical actual tests remain ignored.

Center HTTP admission/dispatch and strict original-result validation are now
implemented, with migration0020 and additive OpenAPI schemas. All four operations
are admin-only with fresh capability, existing tenant/password/CSRF and encryption
contracts. Queued writes retain connection epochs; reconnect expires only
unleased stale writes, preserves delivered UNKNOWN, and queries the original ID.
Read30 follows read leases. Exact typed receipts and full original wire hashes
gate proof resolution; non-admin reads remove prepare confirmation tokens.

A new private PostgreSQL17 cluster, Unix socket only, ran one new actual loopback
HTTP/database contract successfully. It covered role/tenant/password/CSRF and
capability/offline denial, encrypted input/result, original idempotency/no new
prepare, wrong proof rejection, UNKNOWN across deadline/reconnect, once-only
firewall audit, non-admin token filtering, stale queued-confirm expiry and
explicit rollback admission despite another UNKNOWN. Database metadata simulates
delivery/loss; no agent, nft or mTLS is involved. Its cluster is stopped, all
schema/data and the separate private key directory retained. It is a closed
selected contract and must not be replayed in that retained fixture.

Center workspace regression:27 center tests and13 protocol tests pass, with14
database/external center tests and3 historical mTLS tests ignored. The newly
selected PostgreSQL contract above ran separately and passed1. Agent affected
scope39/ignored1 passes after receipt witness/request-hash fields were added.
Strict center Clippy initially rejected one boolean form; equivalent simplification
passes final strict workspace/all-target Clippy. No semantic bypass was added.

Cargo tests updated the default center binary before a new before-copy had been
taken. Its exact prior SHA f8bd11b0309cc4de8504735dc4ac5df6515bd369fa70acc43646b10ab38f3a58
was recovered from an existing sealed old copy, copied into this batch's before
set and verified. The old sealed copy was not modified. The current new build is
retained. This preservation correction is recorded explicitly, not called an
unchanged target.

React UI is now implemented: six structured family/category editors, both
policies/ICMP, optional source ranges/MAC/interface exclusions, nullable numeric
NAT protocol and read-only automatic rules. Explicit initialization/review and
30–300s window are required; old unprojectable values cannot become defaults.
Frozen drafts use consumed baselines until a fresh read. Correlations alone are
stored; original pending candidate/token are fetched from the original task for
review after refresh. Agent prepare receipts carry the original candidate and
center validates its exact typed equivalence. Current connection readiness gates
confirm in addition to the agent's final check. Final countdowns disappear.

Integration exposed a new remote-only ABA gap: rollback could restore the same
confirmed revision/digest and admit an older draft. A deterministic new test
failed by actually accepting that stale draft. Remote view/check now atomically
hash confirmed data plus current journal;40 affected agent tests pass/1 ignored.
The existing Kura/GWT digest is unchanged. Four center receipt tests pass with
candidate equality/readiness checks. Both strict Clippy gates pass. The earlier
isolated PG/HTTP contract remains historical for the pre-candidate-receipt shape;
it was not replayed in its retained fixture after this extension.

Final browser contracts cover9 new firewall/proof cases and9 related translation,
system-proof and task-reconciliation cases,18 distinct checks. They use route
fixtures, not live center/agent. The six workflow cases include field preservation,
30–300s bounds, consumed baseline, refresh of the exact applied candidate, lost
prepare acknowledgement, UNKNOWN without resend, authoritative readiness gate,
terminal countdown removal, English, and role/capability hiding. Build/typecheck
pass. Authored Task operation enum omissions were caught by TypeScript generation
and corrected. An initial missing drawer mount and loader-generated accessible
name caused failed/interrupted browser attempts; their reports remain. A test
also tried to click the page refresh behind the modal drawer, which was corrected.
Read control has a stable accessible name/busy state. Initial fieldset min-content
overflow was found in screenshots and fixed; final Chinese/English captures were
visually inspected. This is not actual browser/mTLS/system acceptance.

All newly owned UI test processes and Vite19190 are stopped. PostgreSQL remains
stopped with data retained. No new production JVM/daemon, VM or real nft action.
Next perform fresh native/mTLS/Linux workflow proof with current paired artifacts.
Do not treat unit contracts as remote business completion. Full14 internal groups and3 external
deferred groups remain. Contract: `docs/remote-firewall-tasks-v1.md`. Current
source/checkpoint pins and before copies are in
`target/remote-firewall-tasks-20261108`, with cc91d577 still the last full seal.

## Isolated host transport evidence

2026-11-08: host-v2 production mTLS dispatch, actual HTTPS browser (1 passed), default-off prepare rejection, IDEA full isolated Kura startup/GWT login and remote configuration list succeeded. All three task inputs/results are encrypted in private PostgreSQL. No nft write grant; actual Linux transaction flow remains pending. Fresh certificate helper now emits SKI/AKI after strict TLS rejected V1; failed scene retained. All owned host processes stopped, fixtures retained. See `target/remote-firewall-tasks-20261108/host-v2` and the unsealed progress checkpoint; last full seal remains cc91d577. Never replay the original default-off probe.

## Actual Linux selected workflow

A fresh g/w namespace pair carried the production agent mTLS connection through a TLS-opaque relay to a new center. Actual browser review/apply changed nft traffic (8 distinct probes); a new authenticated epoch followed application and explicit confirmation succeeded. A different candidate expired after30s and reverted to the confirmed rules. A test-tool TLS trust failure prevented a later browser-loss probe from reaching the center; its original intent is NOT_FOUND, never resent. A new, distinct loss-only scene used process-scoped CA trust, lost the browser202 acknowledgement, restored the exact original task/candidate after refresh, and explicitly rolled back to identical kernel rules (1 browser test passed). The whole earlier attempt remains failed; its successful selected confirm/expiry are not rerun or called a whole-test pass.

Actual encrypted database rows: op30=8, op31=3, op32=1, op33=1, all SUCCEEDED and input/result encrypted. All new services/relay/echo/center/PG stopped; namespace/state/key/source artifacts retained. Four original Orb services and host nft/forwarding unchanged. No full sandbox/physical-NM/agent-stream-loss claim. See linux-workflow/outcome-summary.json in this batch target.

The live scene exposed idle network expiry erroneously requesting system-write admission while another firewall transaction was validly pending, logging false recovery errors each second. A new deterministic regression failed at the idle call before correction. Expiry now checks for its own due WAIT_CONFIRM work without holding the engine lock across shared admission, then rechecks under the existing write gate before rollback. Due inconsistent cross-domain states still fail closed. Network24 tests and strict all-target Clippy pass; a new Linux build/regression is recorded separately. The earlier runtime uses frozen pre-correction source and remains valid only for that pair.

## Original confirmation result lost at the agent boundary

A new explicit test binary ran the actual agent main path and real mTLS/nft confirmation, then exited93 after COMMITTED domain persistence and before SQLite execution receipt completion. The center original command `ce50ec4c-d514-4c27-a559-868c9620821e` became UNKNOWN; no confirmation was resent. The first whole browser attempt failed: the new ignored entry had omitted the real main entry hardening, creating SQLite0644, which the production reader correctly refused. Its failed service, logs, domain evidence and original source state are retained. The test entry now hardens before constructing workers; a new no-task startup under systemd UMask0022 proved DB/WAL/SHM/key0600 and empty nft, then stopped.

An independent complete offline state copy retained identical bytes and tightened permissions on the copy only. A paired production agent with global firewall writes disabled proved the exact original confirmation from `firewall.remote.journal.v1`, including original wire hash and locally proved new connection. The center resolved SUCCEEDED after the original task deadline, with exactly one firewall reconciliation audit. A loss-only read follow-up browser test passed1; repeated GETs were identical, the final UI showed COMMITTED with no countdown, and no additional prepare/confirm/rollback occurred. The original source SQLite retains RUNNING; the derived execution resolves SUCCEEDED. Domain journal/confirmed hashes and actual kernel handles/rules are unchanged across the production restart and proof. No idle-network false recovery error appeared, providing runtime evidence for that preceding fix.

The first inspection used a normal SQLite read before freeze, allowing ordinary WAL consolidation; only the later frozen set is asserted byte-identical. A base-file-only immutable inspection was incomplete for the derived WAL state and is explicitly superseded by an inspection of complete stopped DB/WAL/SHM copies. All new Linux processes, center and private PostgreSQL are stopped; all source/derived data, namespaces and failure evidence are retained; original four services/host nft/forwarding remain unchanged. This selected stream-loss closure does not establish all domain failure matrices, hard power loss, formal deployment or the overall parent ledger. See `stream-loss/outcome-summary.json`, `proof-integrity.json` and coordinated SQLite inspection in the batch target.

## Selected batch seal

Final preservation audit rechecked all195628 pins from cc91d577:23 expected differences, every prior byte set preserved by exact before copies,0 unexplained/unpreserved changes. All18 HEADs/branches and complete prior dirty entries remain; the first default Git untracked-directory aggregation comparison was superseded by matching --untracked-files=all, not interpreted as lost files. Original Orb four PID/package pairs and stopped Parallels states were checked again; official CLI Home SHA is unchanged.

The selected f07d2c1a delivery is recorded in `docs/evidence/remote-firewall-tasks-20261108.json`. Its source/artifact/evidence pins include preserved failure scenes and the precise original/derived proof boundary. The mutable continuation checkpoint and the manifest's own hash are excluded from self hashing. This closes the selected implementation/evidence batch; it is not overall management redesign or formal-release completion. All14 inherited requirement groups, their selected progress dispositions,3 external deferred groups and historical evidence limits remain explicit. Next delivery is NM automatic NAT and coordinated ownership/adoption/retirement/install profiles, retaining NM and existing rejection gates until that integration is safe.
