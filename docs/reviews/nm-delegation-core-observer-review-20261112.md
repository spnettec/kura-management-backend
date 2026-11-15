# R013 durable delegation and actual Core owner observation

This is progress in the existing unapplied candidate, not R013 completion or a
release. Continue `target/nm-role-authority-20261112/workspace/`; do not recreate
the shadow, apply its partial state, or replay old one-time acceptance. Original
management20 and upstream343 source pins remain authoritative for applied code.

## Changes in this candidate generation

The handover Store now reads and fsyncs the same no-follow descriptor, pins its
parent directory, and checks pathname/inode/metadata binding around synchronization.
Recovery does not reopen a previously checked pointer by following its pathname.
Symlinks, hardlinks, replaced parents, changed files and foreign confirmed pointers
are refused. Marker device/inode/content hash are bound into the fenced journal
and immutable lease; identical bytes published at a different inode revoke the
lease. An ambiguous fence publication is observed without another marker action.
Recovery re-syncs matching original lease/receipt/pointer evidence and never
recreates missing immutable lease/receipt files. No real system marker was made.

Coordinator now consumes a validated active delegation before consulting a live
Kura role source. Its two-sided source digest binds the delegation operation,
original plan hash and lease hash. The current allowlist and actual active profile
UUIDs must still match the delegated scope. A broken retained lease fails closed,
without falling back to live CM or inventing ownership. The initial role cache is
an explicitly confirmed delegation, not an operator-maintained role configuration.

New production preparation requires an active delegation before any new backend
action. Existing fake-backend transaction fixtures explicitly exercise the stage
after admission; the production/real-backend path does not have this exemption.
The new delegation binding is retained in both the immutable NM intent and journal
and checked at later effect/confirmation gates. Stripping it cannot impersonate
an old journal. Optional absent fields still serialize as absent. Legacy no-role
intents remain queryable, and original confirmation/compensation can proceed
without a live CM role source after original-intent admission.

Read-only pre-adoption role observations use the raw paired CM projection. The
distinct live fence and external-owner operations remain bounded authenticated
bridge reads. Normal reads, a suppression marker or a caller readiness Boolean
do not grant a new writer lease.

ConfigurationRevisionService gains a default-unsupported
`readExternalConfirmedConfiguration(pid)` observer. Core only implements it for
PIDs in its actual startup reservation set and requires one still-bound external
handler supplying a valid confirmed configuration. Changing a JVM property later
does not change that reservation. It returns detached property containers and
does not expose a generic revision/write fallback. The existing confirmed reader
also rejects a handler replacement during its call instead of returning the old
handler's result. Configuration export advances1.8→1.9, with matching Core/bridge
import minima. This is an additive minor contract, separate from the seven pending
API2 breaking migrations.

`kura.network.owner` calls this actual Core observer for the one fixed network PID
between two actual legacy-writer quiescence checks. Both service references must
remain bound. It exports only original operation/plan/mirror identities and strict
confirmed roles; arbitrary PIDs, `ready` claims, other properties and passwords
are excluded. Rust binds the response to the current authenticated agent instance
and bridge epoch, rejects malformed/extra/cross-PID data and never accepts an
owner identity supplied by caller JSON. This observer is not yet wired into the
Store activation callback, and no NM confirmed external handler is installed yet.

## Evidence and limits

- `handover-unit-v2`:10 durability/admission/identity cases passed before the
  later Active-binding wrapper. Current NM suite includes these tests.
- `native-current-v1`:354 passed,10 failed,15 ignored under default concurrency.
  Failures include short fixture-command timeouts and downstream state assertions
  in boot/enrollment. The unchanged Rust source pins in `native-current-v2`
  pass364/0/15 at two threads. No product deadline or expected behavior changed.
- After raw/owner bridge additions, `nm-current-v5`:121 passed,1 existing ignore;
  `local-all-v5`:33 passed. These include Kura-unavailable coordinator reads,
  UUID/allowlist/receipt drift, no-lease preparation refusal, immutable binding
  tamper rejection and legacy original confirmation/compensation controls.
- `native-current-v3`:the final current full native run passes367/0/15 at two
  threads. Its source pins match the sealed current candidate. Earlier native
  results retain their own source generations and limits.
- Core264 and API25 pass in `core-bridge-all-v3`; bridge76 pass in
  `bridge-owner-all-v4`. The final current paired reactor
  `core-bridge-final-v5` passes all three together. `core-owner-test-summary-v2.json`
  points to retained XML reports; `api-core-owner-classpath-v3.json` seals archived
  final private jars, matching1.9 exports/imports and the unchanged shared API.
  The initial bridge test used a valid opaque revision as an invalid value and
  nested Mockito stubbing; both fixture mistakes were corrected without changing
  production validation. Failed logs remain.
- `core-owner-before-v1`:the same two selected cases produce one assertion
  failure and one unsupported-read error against original Core source and the
  current private API. After source refuses stale-handler views and implements
  the actual observer, both pass in the full Core suite.
- API/Core/bridge jars are private package outputs, not shared-cache installs,
  deployed binaries or OSGi/live Kura acceptance. No Linux/GUI/current strict
  Clippy acceptance is claimed for this incomplete generation.

`candidate-source-pins-delegated-v2.json` seals28 changed existing +8 new files;
three changes are fixture-only, leaving33 production/test changes. Original
1206 inputs plus102 supplementary Core inputs all match. Original preimages are
retained under `before/`. `delegation-preservation-v2.json` verifies18 repository
HEAD/branch/staged and tracked outside the already applied UI delta,20 management
pins,343 upstream pins,141 unchanged credential inputs plus five existing
overlays,ca9 eight artifacts and f07. Runtime inventories remain preserved.

## Required continuation in the same main chain

1. Implement the actual NM confirmed configuration handler/mirror. Preserve every
   ordinary field and secret representation from the original paired full CM
   configuration; role-only projection is insufficient. A viable direction is a
   sealed encrypted original template on the Java side, bound to the original
   delegation plan, plus agent-confirmed typed deltas linked to terminal NM
   receipts. Verify existing crypto/snapshot/password-codec contracts before
   selecting an implementation. Do not create a manual file or environment-based
   role authority, or introduce center CA-private-key custody.
   Avoid a template/plan hash cycle: export the original encrypted template
   without its eventual plan hash, bind that ciphertext into the agent's immutable
   plan, and link the plan in a separate Java receipt. Retain the original export
   operation/pre-intent before that local write; an ambiguous export is queried,
   not re-encrypted or resubmitted under another operation.
   Initial source inspection confirms the bounded char-array CryptoService API
   uses AES/GCM/NoPadding with a12-byte random IV and128-bit tag, and verifies
   decryption with doFinal. Graph symbol names collide across these overloads;
   inspect actual implementations at CryptoServiceImpl lines149/337. Verify key
   provenance/default-key compatibility behavior before using it for new stored
   secrets. SnapshotPasswordSourcesTest documents distinct CM-cipher/SCC-logical
   password boundaries; preserve those explicitly instead of guessing by text.
2. Add the explicit adoption controller/GUI, cancellation and full retirement.
   The trusted activation callback must match actual Core owner proof against
   the original operation/plan/template and verify actual drained legacy writers,
   profile/CAS scope and complete firewall/NAT ownership. Merely parsing the new
   owner response is insufficient. Preserve foreign/legacy rules until an explicit
   verified transfer; remove suppression fences last on retirement.
3. Reserve the fixed external PID before Core startup after explicit adoption.
   Raw CM roles are read before delegation; after Core reservation use the actual
   owner/confirmed mirror rather than attempting the intentionally unsupported
   generic CM revision read. An authenticated new bridge registration after Core
   restart must be bound to original handover proof, not accepted by a loose epoch
   comparison. External handlers must never call ConfigurationService.
4. Avoid an activation→Core owner observer→external handler→agent lock cycle.
   The handler should read immutable confirmed evidence without a recursive live
   CM call or acquiring a lock already held by its original activation request.
   Any read spanning a transition must prove one coherent original/confirmed
   generation or refuse it; do not add a reentrant write bypass.
5. Joint role proposals must publish confirmed WAN/LAN/NAT intent only with the
   original NM terminal receipt. Current equality with unchanged CM roles still
   rejects disable/subset changes and is not shippable. Add generation/receipt
   settlement without backend replay, NAT-only checkpoint scope, multi-WAN and
   real adopted modem/interface mapping, ordinary retained fields and other-writer
   closure including Python/ordinary startup guards.
6. Separate new preparation availability from legacy original reconciliation in
   the GUI. Complete capability gating and explicit keep/disable/set flows, fresh
   matching API/Core/NM/bridge/UI builds and authorized new Linux/GUI/current
   strict checks before guarded source application. Record the one API-manifest
   overlap as an explicit overlay; do not rewrite the old342/343 seals.

Continue R017 GUI deployment/packaging, TLS reload, bulk enrollment, published
protocol crate, the full14 management groups and seven API2 migrations. External
deferred remain only real NM/cloud-init/physical hardware,amd64,disk-full/hard-power.
Weekly usage28%, ordinary usage allowed, credits unchanged; goal remains active.
