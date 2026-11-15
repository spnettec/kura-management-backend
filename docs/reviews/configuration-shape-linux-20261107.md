# Configuration shape on real management transports — 2026-11-07

Round ID: fd837c85. Continues 1f7815c6 / b5c0e9c7 / 6336f5c7.
The entire management redesign remains active and incomplete.

## Reconciliation and preservation

Entry: all18 repositories' exact HEAD/branch/full porcelain matched 1f7815c6;
558 source,422 artifact,2064 evidence pins matched. Before changing/building bridge,
19 affected source/build inputs were copied and hash-verified; the continuation
prompt was also preserved before updating. All2064 previous evidence files remain
unchanged. No commit/push/reset/clean or historical one-time entrypoint replay.
UNKNOWN was never resent. New independent profiles, state, keys, journals, private
PostgreSQL and VM units are retained.

## Failure that changed the implementation

The first fresh Orb actual center/agent/UDS test passed typed tasks but its actual
Core snapshots exposed a further defect. ConfigurationPatch.convert always emitted
Vector for negative cardinalities. The snapshot after a native field update had
legacy oneList=LIST and nativeList=VECTOR; after a legacy field update these became
VECTOR/LIST. A successful typed response was therefore insufficient to prove stable
snapshot collection representation.

A strengthened pre-change regression fails with expected ArrayList, actual Vector.
The bridge now preserves the current public List/Vector representation while
converting elements; absent containers retain the former Vector fallback. New tests
cover capacity-one and larger lists/vectors, whitespace and empty collections.
No Java API, core, GWT, React, Rust or protobuf production code changed this round.
Bridge reactor install passed66 tests; diff check passed. Graph paths report
metadata_match/no recorded gap, a best-effort signal rather than completeness proof.

The old Orb failure is retained. Its tasks were not resent. Its unused Parallels
profile was stopped before remote tasks. New v2 identities and profiles were
created for the actual fix. Collectors initially made incorrect assumptions about
seed Vector versus normalized public List, and Orb NNP; retained actual snapshots
and /proc corrected these assumptions. No product rule was relaxed to obtain green.

## Selected verified scope

- New independent center on host: fresh PostgreSQL and CA, strict HTTPS19453 and
  mTLS gRPC19454. Three v2 gateways have distinct client certificates and generation1,
  only reviewed shape.legacy/shape.native permit configuration writes. Remote
  channel explicitly enabled; network/firewall/snapshot/recovery/system control
  grants remain off. Original services/identities were untouched.
- New complete Kura started only through IDEA UI, PID61487, profile f7f668b4;
  inventory283 includes launcher/framework, actual OSGi282 includes framework.
  Original admin/admin login passed with fixture-only password-gate setting.
  Production first-login/server gates unchanged. New agent61416 used actual UDS.
- Host, Orb and existing Parallels v2 each passed11 real HTTPS/PostgreSQL/mTLS/
  production-agent/authenticated-UDS/Core tasks: six reads, two successful typed
  writes, three invalid-shape/capacity rejections. Rejections retained both revisions.
  Legacy +/-1 scalars, true one-element arrays/collections, native OSGi +/-1
  defaults and whitespace traversed the full transport and read back correctly.
- Each actual DS observer passed; each latest real Core snapshot matched six fields.
  Both saves kept LIST for public lists. Agent execution metadata matches the
  original11 task IDs per v2 gateway; one row per command, no request replay.
  Unit contracts also preserve explicitly exposed Vector values. This is online
  editing/save evidence for two test PIDs, not full component or restore/recovery.
- Original GWT on host read the real remote result after reload. The initial open
  page still held its pre-update cached component values; automatic refresh and
  concurrent-editor behavior need a focused audit. No stale GWT form was submitted.
  This round has no live React-editor write acceptance; its client source and
  previous bilingual serialization tests were unchanged.
- Orb actual Kura748790/agent748784: CapEff0 both, agent hardens NNP1. The existing
  global service override results in ProtectSystem=no and Kura NNP0. These are
  limitations, not strict-sandbox proof. Global override was not changed.
- Parallels actual Kura3921/agent3918: CapEff0/NNP1 both, actual ProtectSystem=strict;
  separate mount-namespace probes both returned EROFS30. Existing verified private
  ARM64 JRE184 files used. Host nft stateless ruleset and IPv4/IPv6 forwarding
  matched before/after on both VMs; no network/firewall/system-control task ran.
- Center database includes44 tasks: first Orb11 plus v2 host/Orb/Parallels33.
  All44 input and44 result JSON values have encrypted envelopes,32 successes,
  12 failures,8 successful writes total,12 rejected writes,0 pending/UNKNOWN.
  Fixture plaintext field/value markers were absent from stored envelopes.
  This checks current task storage, not a DB/key backup/restore/rotation proof.

## New fixture failures and limitations

The freshly generated server certificate initially lacked Authority Key Identifier
required by the strict Python/OpenSSL verifier. The center was observed live; no
restart occurred just because readiness timed out. With zero admitted tasks, its
old certificate was retained, AKI/SKI added and only this new center restarted.
Strict certificate verification stayed enabled. New v2 client certificates include
AKI/SKI. An initial storage collector expected only successful results encrypted;
actual SQL showed all44, including failed results, encrypted. This stronger actual
result was recorded. No old runtime or test was replayed.

## Final state and full remaining ledger

All new native processes, center, private PostgreSQL and v1/v2 VM units stopped;
all data retained. New fixture gateway identities disabled in the new database.
Ubuntu Parallels shut down normally; Windows stayed stopped. Original Orb services
retain PIDs244516/244911/286620/268 and their installed packages.

Selected new Linux/center-agent configuration-shape validation is now closed.
Internal pending: full component/metadata/GWT editing and stale-cache/concurrent
editor audit; empty/add/remove fields and complete snapshot restore; formal API
upgrade/minimum-version packaging; production/external PID Java recovery and
empty/partial/inconsistent CM policies; NM automatic NAT and other-writer
ownership/adoption/retirement/install profiles; encrypted center firewall tasks
and bilingual remote editing with affected-connection proof; confirmed-only actual
early boot and Core install/upgrade/bootstrap/handoff; consistent DB/artifact/key
backup, restore, rotation, migration and release. Full firewall product and broad
Kura recovery remain false.

External deferred remains real NM/cloud-init/physical hardware, amd64 and disk-full/
hard-power. Java is available. Historical warning attribution, two historical gaps
and 6336f5c7 missing original mutable Surefire XML remain distinct limitations.
Already closed recovery/cancel/restart/reboot/NAT and F01-F19 evidence is not replayed.

Evidence: ../evidence/configuration-shape-linux-20261107.json.
Private output: target/configuration-shape-linux-20261107.
