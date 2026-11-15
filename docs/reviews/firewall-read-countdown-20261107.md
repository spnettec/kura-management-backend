# GWT initial-read contention and terminal countdown — 2026-11-07

Resumed from 6336f5c7. Overall goal is **active**, this round is progress. No commit,
push, reset/clean, package installation, old one-time acceptance replay or UNKNOWN
write retry occurred. Existing NM, identity, authorization and recovery boundaries
remain in force.

## Entry reconciliation

All 18 repository HEADs, branches and full porcelain statuses matched the latest
paused manifest. All 547 source, 79 artifact and 650 evidence pins matched. All
626 source/artifact contents were copied and verified before edits/tests; this
includes the three then-current Surefire XML reports. This preserves this round's
inputs without claiming to recover the earlier missing XML originals. The old
151 evidence pins, two historical evidence gaps and 6336f5c7 XML limitation remain
separate records.

## Read conflict mechanism and fix

The production path is GWT findFirewallEdit -> authenticated FirewallSession.read
-> AgentFirewall.getFirewallSnapshot -> firewall.kura.view -> Firewall.observe ->
Guard.observe. The last operation used nonblocking LOCK_EX even for a read. It
therefore rejected an overlapping coherent read with CONFLICT. Core confirmed-PID
reads and the agent's background recovery checks also use this system boundary.
The six GWT drafts retain their own immutable baselines and session serialization;
no authentication/session relaxation was needed.

A new deterministic regression holds a real OS shared lock and issues 32 parallel
views. With the previous exclusive read call it fails with the exact CONFLICT;
with the fix all views succeed and writers remain excluded. Firewall view/outcome
now use a shared observation lock with a one-second bounded wait, acquired before
performing the read. System writes retain the original nonblocking exclusive lock
and full persistent admission checks. This neither retries an action nor bypasses
RECOVERY_REQUIRED; readers still cannot observe a writer's partial state. A held
writer times out with CONFLICT; a short writer can release before the deadline.
The lock is advisory among cooperating code, not protection against arbitrary
privileged external mutation.

The retained native-v3/Parallels warning has no lock-holder trace. Its exact past
occurrence therefore cannot be forensically attributed to this mechanism. The
matching production defect and new actual first-load check are verified; historical
causation is explicitly unproven, rather than silently declaring the old evidence
complete. Long genuine write contention may still return CONFLICT by design.

## Countdown fix

FirewallSession projects remainingSeconds only for WAIT_CONFIRM. The GWT renderer
also only adds the countdown in that state; all other states show their translated
state label. Deadline persistence and transaction recovery semantics are unchanged.
New tests cover all non-wait states, fresh WAIT_CONFIRM, explicit confirm and
original-outcome projection. UNKNOWN handling and no-resubmission behavior remain.

## Validation and precise scope

- Host agent: 219 passed, 10 explicitly ignored; fmt and Clippy all targets with
  warnings denied passed. A new shared-lock and a bounded-wait guard regression
  exercise real OS locks. Ignored fixture/system entrypoints were not replayed.
- Java/GWT: 23 affected contracts passed; full login and denali compilation and
  server package/install passed. The new independent widget compiled all six
  permutations against the final client.
- New standalone browser fixture: two prepares, one confirm, one rollback, zero
  system writes. Chinese WAIT_CONFIRM counted down; Chinese COMMITTED and
  ROLLED_BACK had no countdown; English ROLLED_BACK had no countdown after reload.
  It uses real production widgets/RPC/server JAR and a fake provider, not Linux
  firewall transactions or proof of an affected management path.
- New complete isolated Kura was started through IDEA UI, PID56852, inventory284
  (283 non-framework bundles), fresh profile db6cd2c8. Original admin/admin login
  and hidden remote menu without installed agent passed. Production agent56872,
  all write grants off, exposed actual UDS reads. While an independent process
  held a real shared lock, the original GWT initial six-editor load and policy
  read showed UNINITIALIZED/disabled writes and no CONFLICT popup. No default
  rule or firewall journal was created. Only this fresh fixture bypassed first
  password change; production authentication gates are unchanged.
- Linux build: 115 copied input pins match current workspace; independent build
  root eeae6cc4, old target cache copied without modifying it. Offline release,
  firewall23 passed/1 ignored, guard2 passed and strict Clippy passed. First build
  attempt lacked CARGO_HOME and found no base64 index; its log is retained. The
  continuation only supplied the existing registry path; no system action ran.
- New Orb agent PID748073, unit yofc-fw-read-aad0cd77: actual registered UDS32
  reads under shared lock passed; short writer wait ~0.157s, held writer timeout
  ~1.006s, later read succeeded. CapEff0/NNP1. Actual readonly probe succeeded
  in writing its temporary test file because the old global override remains;
  this is explicitly not strict read-only sandbox proof.
- Existing Parallels, new agent PID2476, unit yofc-fw-read-e5f52d5a: same actual
  UDS32, ~0.152s short wait/~1.002s timeout, CapEff0/NNP1 and real EROFS30.
  Host nft/IPv4+IPv6 forwarding hashes matched before/after on both VMs; no
  firewall transaction journal, no firewall/system/network write request.
  These are agent/UDS checks, not another full Linux GWT/Core/traffic acceptance.

The initial Rust test fixture compile used TempDir incorrectly; its compiler
output was retained, then corrected before the valid red regression. Initial
cargo path lookup failures are not test outcomes. No production logic was changed
to hide these failures.

## State and remaining work

All new native JVMs/agent/browser helpers and both VM test units stopped; fixture,
state and build directories retained. Ubuntu shut down normally, Windows remained
stopped. Original Orb four services retain PIDs244516/244911/286620/268 and original
console26/agent20 packages. Full final state is in the manifest's evidence root.

Only six previous pinned source files changed for the two fixes/tests. Latest
source/artifact pins and full worktree status supersede prior-current hashes;
prior contents remain archived. No full workspace/Docker/deb/release acceptance
is claimed. Full firewall product and broad Kura recovery remain false.

Internal pending remains full components/0±1/metadata/GWT editing; production and
external PID Java recovery and empty/partial/inconsistent CM policies; NM automatic
NAT and formal other-writer adoption/retirement/install profiles; encrypted center
firewall tasks/bilingual remote and affected connection proof; confirmed-only
actual early boot and remaining Core install/upgrade/bootstrap/handoff; consistent
DB/artifact/key backup, rotation, migration and release. Historical warning trace
attribution remains an evidence limitation. External deferred stays real NM/cloud-init/
physical hardware, amd64, disk-full/hard-power; Java is available. Old F01–F19,
selected recovery/cancel/restart/reboot/NAT checks stay closed without replay.

Evidence: `../evidence/firewall-read-countdown-20261107.json`; private output:
`target/resume-gwt-initial-20261107`. Continue the original ledger without a new pause.
