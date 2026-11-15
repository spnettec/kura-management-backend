# ZCode checkpoint review and refactoring ledger

2026-11-11. User resumed the original full goal after quota reset (live usage0%).
This review incorporates the actual ZCode conversation, its subsequent user-approved
Scheme A/Compose/manual work and40 change batches. This is the first complete source-review pass across the checkpoint domains,
not approval of the implementation or completion of the original redesign. The initial review changed no production source. The later key/TLS batch below
contains scoped source refactoring. No deployment, UNKNOWN resend, old one-time
scene replay, commit/push/reset/clean has occurred.

## Preserved review baseline

Evidence root: `target/codex-zcode-review-20261111/`.104 current scoped file copies
were preserved before refactoring. The actual session has2121 messages;418 visible
dialogue entries and22 user instructions were read, with tool evidence exported
privately for subsequent per-claim checks. Historical reasoning parts were not
used as user instructions.18 HEAD/branches match the checkpoint. Aggregate vs
full untracked-status formats were normalized; no baseline paths are actually
missing. The protected ca9 eight artifacts and f07 manifest remain exact.

Composite paused baseline2026 entries:1979 unchanged,47 changed, including44
management-source/doc paths and3 derived NM target manifests. These3 build outputs
are not new production-source changes outside management repositories.
ZCode's user-authorized cleanup and extra VMs/Compose supersede the old VM/PID
preservation expectation for that historical interval. Current scene is preserved:
Orb gw-001 and Compose console/postgres were observed starting, both Parallels VMs
stopped. Do not recreate old deleted services or execute historical cleanup.

Tasks.md and Session Summaries.md still contained the entire old paused readback
without ZCode progress. Nine newer Experience notes were read in full and captured
through the official workspace Obsidian CLI. Their completion assertions require
code/evidence review; they do not supersede the original broad requirements.

## Original checkpoint findings and repair order

### R001 P0 — crates/console/src/http.rs:1425-1470

Online key rotation commits DB re-encryption before durable replacement key publication. Staging failure returns INTERNAL while the only new key is ephemeral, leaving neither active disk key nor live cipher usable for the new DB binding. A crash between commit and key staging has the same loss window.

Evidence: reproduced: v2 isolated HTTP regression.

Current disposition: implemented in the key/TLS batch below; new isolated contracts pass. Deployment acceptance remains open.

### R002 P1 — crates/console/src/grpc.rs:114 and http.rs:1425

The HTTP App and gRPC ControlService are created with different Arc<RwLock<TaskCipher>> instances. A successful HTTP rotation never updates the gRPC cipher, including future streams. Existing streams also retain clones. HTTP writers release cipher read locks before DB commit. No DB binding fence rejects stale-key writers after rotation.

Evidence: source-confirmed; main.rs uses separate HTTP and gRPC cipher locks.

Current disposition: implemented in the key/TLS batch below; new isolated contracts pass. Deployment acceptance remains open.

### R003 P1 — src/nm/coordinator.rs:832-865

CheckpointCreate is issued with an empty device list. This means all devices rather than the explicit affected/allowed device set, widening rollback scope beyond the proposal.

Evidence: source-confirmed.

Current disposition: source-repaired in `nm-checkpoint-scope-review-20261112.md`.
Fresh source-bound changed-device paths are persisted and actually dispatched;
empty legacy intents and device drift fail before creation. New before3 failures,
after NM44/full-agent286 passes and strict Clippy pass. Applied only to the agent
coordinator source; release/Linux integration remains internal pending. This does
not close R004/R005/R006 or broader NM/recovery requirements.

### R004 P1 — src/nm/coordinator.rs:944-1027

Update2/Reapply errors of any kind enter a bounded retry loop. Successful-but-lost replies/no-op edits can reissue the original mutation without proving non-dispatch or classifying a definitive CAS rejection. This conflicts with UNKNOWN no-resend.

Evidence: source-confirmed; unknown-write regression pending.

Current disposition: source-repaired in `nm-dispatch-review-20261112.md`. Five
original counterexamples fail before; a follow-up full no-op confirmation case
also exposed and fixed the first candidate's remaining version-increment check.
Final native293/NM51 cases and strict Clippy pass. Forward writes are once per
fresh intent; opaque failures remain UNKNOWN and retained intents cannot restart
forward dispatch. Seven unit functions include full no-op COMMITTED and negative
content/version controls. Only the coordinator source is applied; the R003 source
pin is explicitly superseded with its prior snapshot retained. Fresh release/Linux
integration remains internal pending. R005/R006 and broad NM scope remain open.

### R005 P1 — src/nm/coordinator.rs:1245-1284

CheckpointIntent recovery attributes every newly observed checkpoint to this operation based on an agent-only flock. Other NM clients do not share that lock. It may compensate a foreign checkpoint; the branch also calls CheckpointRollback on the checkpoint path with Manager interface and no path argument, unlike the corrected main rollback path.

Evidence: source-confirmed; recovery adversarial contract pending.

Current disposition: source-repaired with original bus/service binding, actual
authentication GUID pinning, acknowledged-path ownership, overlap-aware rollback,
fresh stored/applied restoration proof and durable compensation dispatch flags.
Unacknowledged create and unproven legacy ownership quarantine without writes.
See `nm-checkpoint-recovery-review-20261112.md`; release/deployment remains open.

### R006 P1 — src/nm/coordinator.rs:1324-1344

ConfirmDecided recovery publishes COMMITTED when the checkpoint is absent without rechecking the candidate/profile/applied/NAT state. An expired checkpoint may already have rolled back after the durable decision and before destroy; absence alone cannot distinguish that from successful confirmation.

Evidence: source-confirmed; expired-decision regression pending.

Current disposition: source-repaired. Confirmation and recovery reread the live
candidate, stored/applied fields, identities and NAT before terminal COMMITTED.
Expiry after the decision fails closed; a lost destroy reply can be adjudicated
without resend. Native312/14ignored, Linux NM70/2ignored and strict Clippy pass.
See `nm-checkpoint-recovery-review-20261112.md` for exact scope and limitations.

### R007 P2 — target/zcode-resume-20261108

Current source differs from latest v33 frozen inputs in five files. Two last recorded after SHAs differ, nine scoped last after SHAs are absent, and 38/79 recorded before SHAs have no matching file in the retained before tree. Additional new NM UI/backend paths were not listed in seq003. Preserve evidence; do not call current source acceptance identical to v33.

Evidence: verified provenance limitation.

### R008 P2 — src/nm/coordinator.rs:2558-2654

The ignored real-NM remote test constructs connection witness strings directly, including now()+5000; it uses a real NM backend but no actual mTLS reconnect. It cannot alone prove transport witness minting. Later manual browser/transport evidence must be evaluated separately.

Evidence: verified evidence limitation; manual transport trace still to inspect.

### R009 P1 — crates/console/src/task_crypto.rs:442-511

Rekeying skips encrypted mgmt_gateways.identity_bundle_ciphertext, so an undelivered identity or rotation bundle becomes unreadable after task-key rotation.

Evidence: reproduced: v2 isolated HTTP regression.

Current disposition: implemented in the key/TLS batch below; new isolated contracts pass. Deployment acceptance remains open.

### R010 P1 — crates/console/src/task_crypto.rs:452-454

rotate_storage accepts version=1 only but writes version=2. The second rotation always fails after the first success.

Evidence: reproduced: v2 isolated HTTP regression.

Current disposition: implemented in the key/TLS batch below; new isolated contracts pass. Deployment acceptance remains open.

### R011 P1 — crates/console/src/http.rs:1067-1101; src/provisioning.rs; docker/entrypoint.sh

TLS upload accepts a base64 PEM shell as a private key without parsing or checking the certificate pair; pair members are staged independently. Failure staging the second member leaves the first published. Entrypoint then copies/deletes members individually with no pair transaction or recovery.

Evidence: reproduced: v2 invalid-key and half-pair regressions; valid v3 TLS control passes.

Current disposition: implemented in the key/TLS batch below; new isolated contracts pass. Deployment acceptance remains open.

### R012 P1 — crates/console/src/http.rs:settings_task_key_backup/restore; docker/entrypoint.sh

Backup reads CONSOLE_DATA_KEY_FILE even after online rotation changes only pending/live memory, so it can back up the old key. Restore authenticates a backup but does not verify its key against current DB before staging; entrypoint blindly overwrites the only active key on restart.

Evidence: source-confirmed.

Current disposition: implemented in the key/TLS batch below; new isolated contracts pass. Deployment acceptance remains open.

### R013 P1 — kura-management-agent/src/nm/coordinator.rs:validate_nat/verify_nat; src/firewall.rs:foreign_writers

NM automatic NAT uses yofc_nm while the firewall writer treats other tables as foreign, so the two owned subsystems conflict. NAT candidates are caller-provided rather than derived from authoritative WAN/LAN roles; verification checks interface/comment pairs but not masquerade action, chain hook or full rule semantics.

Evidence: source-confirmed; kernel adversarial contract pending.

Partial disposition: the checkpoint recovery batch verifies full owned NAT
table/chain/rule semantics and preserves/restores the prior NAT baseline. The
follow-up owned-NAT batch adds committed provenance shared with the manual
firewall, a joint source lease, explicit keep/disable/set, and checkpoint expiry
fencing before NAT dispatch. Legacy queued/UNKNOWN null input retains its original
wire identity. Authoritative Kura WAN/LAN/NAT roles, Java adoption/retirement,
real kernel/release integration remain internal pending. The subsequent
forwarding/read-only batch now binds and checks actual IPv4 forwarding before a
NAT candidate, checkpoint/NAT dispatch and confirmation, without enabling it.
A valid nft proposal alone does not prove forwarding is enabled. This
finding remains open. See `nm-nat-ownership-review-20261112.md`.

### R014 P1 — frontend/react/src/features/gateways/NmPanel.tsx

Lost prepare acknowledgement persists the original intent before POST but saves the prepare lookup only after the response. Original-ID reconciliation can restore a plan, but a later view refresh clears it without a saved prepare ID, losing explicit confirm/rollback controls.

Current disposition: source-repaired. The exact original prepare lookup is saved
when its reconciled receipt is accepted; subsequent task reads validate original
IDs. The retained before browser case fails, and the final five mocked cases
include two successive view refreshes and a wrong-ID negative control. No token
is cached and no replacement prepare is issued. Real deployment remains pending.

### R015 P1 — kura-management-agent/src/transport.rs:312

Forced fresh-connection handling matches prepare operations 5,10,31 but omits NM prepare 35. A successful NM candidate that does not itself break the connection leaves confirm bound to the pre-apply witness and rejected; the normal UI has no explicit agent reconnect action.

Evidence: source-confirmed.

Current disposition: source-repaired. NM_PREPARE now requests a fresh connection
after delivering WAIT_CONFIRM. A new real local mTLS contract reproduces the old
failure and passes after repair, including read-only/UNKNOWN controls and proof
that no system task was claimed. See `nm-transport-csr-review-20261112.md`.

### R016 P1 — kura-management-agent/packaging/setup.py:602-631

CSR output checks absence before key generation, then shutil.copyfile follows the eventual destination. A new symlink created between check and copy can redirect the privileged write. Use exclusive no-follow creation at the actual write.

Evidence: source-confirmed.

Current disposition: source-repaired with a pinned directory descriptor and
exclusive no-follow output creation; file writes and chmod use that descriptor.
Four before-failing races and a valid CSR activation control pass natively and on
Linux ARM64; native packaging runs84 with2skip. Pending local key/CSR are retained
on export refusal. See `nm-transport-csr-review-20261112.md`.

### R017 P2 — desktop deployment skills; docs/provisioning-ui-design-v1.md; Tasks.md; Session Summaries.md

Standalone deployment steps omit stopping the postinst-started agent before import/CSR commands that require inactivity. Older provisioning design sections contradict selected Scheme A. ZCode Experience notes were not reconciled into Tasks/Session Summaries. TLS hot reload and batch enrollment are internal engineering pending, not external deferred.

Evidence: source/documentation-confirmed.

### R018 P2 — crates/console/src/nm_tasks.rs; frontend/react/src/features/gateways/nmModel.ts

Read-view validation requires writeEnabled=true, rejecting a legitimate read-only coordinator view. Keep read admission separate from write enablement.

Current disposition: source-repaired across agent, center and browser. An observer
can return a token-free original pending status without a write engine or nft
dependency. False Boolean enablement is admitted for reads; malformed flags and
contradictory readiness are refused. Pre-checkpoint views do not invent a deadline.
All mutation controls, including saved-plan rollback, require current write
enablement. Before counters fail and final native/Linux/browser checks pass.
See `nm-forwarding-readonly-review-20261112.md`. Deployment remains pending.

## New counterexamples and controls

A source-only shadow runs six new HTTP regressions against a fresh private
PostgreSQL17 database bound only to a private Unix socket. Five safety assertions
fail and the valid X.509v3 TLS pair control passes. The failing cases prove the
post-commit key loss, skipped bundle rekey, second-rotation failure, invalid DER
key acceptance and half-pair staging. The first fixture used an X.509v1 certificate
that was rejected before TLS staging; those two apparent passes are not evidence.
The corrected v2 log is authoritative. Every review-owned PG process stopped;
fixtures, logs and current before copies remain. No deployed service was changed.

## Scope disposition

The nine checkpoint review domains have a first source/doc pass. Protocol and
transport witnesses are minted locally after actual welcome, but the synthetic
real-NM test alone cannot prove a reconnect; manual transport evidence remains
separate. Current source is not the latest frozen Linux pair. Screenshots prove
visible states only, not fault recovery. Review findings are not a retrospective
claim that all1713 ZCode tool outputs were independently accepted.

Refactor the key lifecycle as a transaction spanning durable old/new key recovery,
DB binding, all encrypted domains, same-process and DB writer fencing, active-key
backup and DB-checked restore. Preserve the Scheme A external CA boundary. Then
repair NM per-action dispatch/UNKNOWN, checkpoint ownership and terminal readback,
shared authoritative NAT, original-ID UI restoration and packaging/docs.

The inherited14 groups remain authoritative. Core/editor semantics (1–6), direct
CA/non-DS ownership and lifecycle (7,12), broad configuration recovery (8,9), NM and
remote management (10,11), consistent backup/migration/release (13), and historical
evidence limits (14) are retained. Selected fixes do not close broad parents.
The read-lock/countdown, selected generic recovery, Driver/Asset receipt and frozen
remote-firewall scenes remain closed and must not be replayed.

External deferred remain exactly the original three groups: real NM/cloud-init/
physical hardware, amd64, and disk-full/hard-power. TLS hot reload and bulk enrollment
are internal implementation/design pending, not new external deferred. Scheme B
is outside the user's selected A. No overall completion percentage is inferred.

## Index resource limitation

Agent graph refresh succeeded. Console indexing stopped at its2267MB process
budget with3282MB RSS; its old graph remains. The Mac has18GiB RAM, so this is not
proof of machine-wide OOM. Retained build/runtime/PG directories may contribute,
but their actual inclusion has not been established. Stale/not-tracked console
paths were reviewed from source; no global limit increase or fixture deletion.

## Key/TLS refactoring batch — implemented, not deployed

R001/R002/R009/R010/R011/R012 have scoped implementation and new isolated contracts:

- `task_key_store.rs` persists both old/new key slots before DB mutation; startup
  chooses by the authenticated DB key-check. A prepared-but-uncommitted key does
  not replace the original. Configured base keys and recovery slots are retained.
- HTTP and gRPC share one lock. Each encrypted DB operation verifies the current
  key under a transaction-held DB binding lock before gateway/task locks. Rotation
  takes the exclusive process lock and DB lock; stale processes fail before writes.
- Rekeying includes tasks and undelivered identity bundles in one transaction and
  supports repeat rotation. Backups use the current runtime key. Restore validates
  against the DB before staging; the root entrypoint no longer overwrites keys.
- TLS validates parsed private keys and certificate pairing. Immutable pair
  generations and one durable pending/active pointer replace independent file
  copies. Activation is idempotent across restart; configured base files remain.
  Client-key uploads are paired before registration/certificate replacement too.

The final new private-PG run passes15 contracts, including all five original
counterexamples and a positive TLS activation/restart control. Native console
unit checks pass38 with30 ignored; three prior mTLS scenarios remain ignored.
Strict all-target Clippy, React typecheck and shell syntax pass. No live endpoint,
NM/nft state, prior acceptance fixture, container or existing DB was written.
All review-owned PG servers stopped; data/logs remain. Fresh Linux/container
acceptance for this source is internal pending, not one of the external deferred.

Fixture history is retained: a compile-only method ambiguity was corrected;
the first EC key fixture had implicit parameters unsupported by ring; the final
fixture uses explicit named-curve P-256/PKCS#8 like packaging. The positive control
pre-validates key pairing, so early rejection cannot be counted as a staging pass.
A missing `req -new` in one fixture attempt failed before PostgreSQL started.

### R019 P1 — Scheme A mount boundary still open

Both Compose definitions mount all of `./tls`, including demo `ca.key` and gateway
private keys. That puts CA private material inside the container despite selected
Scheme A. This batch has not fixed or redeployed that configuration. The next
batch must minimize mounts and adapt client-CA activation together; merely making
the directory read-only does not remove the private material. Desktop deployment
artifacts must be aligned as a coherent release, not partially copied over.

Final read-only preservation audit:18 HEAD/branches match; Git status changes are confined to console. The f07 manifest and ca9 eight artifact pins are exact. Existing Docker console/postgres are healthy and Orb gw-001 is running; this is observation of the existing stack, not deployment of the new source. Workspace Experience/Tasks/Session Summaries append text was verified by exact official-CLI readback.

## Credential mount stage and upstream checkpoint

R019 is now source-refactored and verified; new Linux release/deployment remains open. The follow-up also rejects CA material in ordinary identity forms and binds file review to the current form revision (R020). See `credential-mount-20261112.md` for exact scope, 146-input final seal,46 native/17 private-PG/5 browser checks and the entrypoint-only Linux contract limitation. The next user-required stage is logical upstream synchronization:11 forks and7 nonforks verified, with cross-repo consumer analysis retained for all18. No working-tree merge, commit or push has occurred.

### R021 P1 — coupled NM candidate compiler — source-repaired 2026-11-12

The R005/R006 review found that DetachedCandidate.applied held the old active
configuration although its digest described the computed edited configuration.
Reapply therefore sent the source. The compiler now carries the computed applied
candidate and separately retains sourceApplied for restoration proof. A contract
checks the actual Reapply payload. See `nm-checkpoint-recovery-review-20261112.md`.

### R022 P1 — NM terminal receipt published before final journal update

A valid receipt already on disk caused exclusive publication to fail again during
recovery, preventing the original confirmation from settling. The owned-NAT batch
reuses only byte-identical private single-link receipts, syncs file/directory,
and validates the receipt/ownership-pointer/journal prefixes. Conflicting receipts
remain uncertain, with no backend resend. New before/final contracts reproduce
and repair the prefix failures. Source-repaired; deployed crash acceptance remains
open. See `nm-nat-ownership-review-20261112.md`.

Current continuation is `continuation-active-20261112-nm-forwarding-readonly.md`.
The preceding upstream paragraph is historical: functional upstream cohorts have
since been source-integrated, with API2 compatibility migrations still explicitly
pending. R005/R006/R014/R015/R016/R018/R021/R022 are source-repaired; R013 is only partially
repaired. Fresh coherent deployment and all inherited14 parent groups remain open.
