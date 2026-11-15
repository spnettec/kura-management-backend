# R013 v22 — Core three-handler ownership proof

Status: independently validated v22 checkpoint. R013 remains unapplied in the original-bound
shadow. The immutable v21 source archive remains historical; current working
sources are recorded in `target/nm-role-authority-20261112/core-owner-cohort-source-checkpoint-v22-final.json`.

This change closes a network-only owner-proof gap. It does not establish ownership
of foreign kernel rules, migrate legacy firewall policy, or complete participating
Core/NM startup. Evidence below uses synthetic, owned test data only.

## Defect and implementation

The retained before test demonstrates that the previous bridge returned a valid
network owner even when both firewall handlers were absent. The first test-fixture
error and the subsequent genuine failing counterexample are retained separately.

The existing unreleased configuration API1.9 now offers an optional-default batch
read. Core captures all requested, startup-reserved handler registrations before
calling any, and checks identity plus registration generations before and after
each read and at completion. Removing and rebinding the same object is detected.
Returned typed values are detached. This is a stable registration cohort; it does
not promise an atomic transaction across independent backends. Older API providers
fail closed through the default UNSUPPORTED result. Generic revision/write access
to the reserved PIDs remains forbidden and no CM fallback is introduced.

The bridge owner projection now requires network plus both actual API firewall
PIDs from that Core batch. Wire schema2 adds matching firewall revision and digest
for both families. The revision is a canonical positive u64 decimal string, so
values above JavaScript's exact-integer range are preserved. The digest commits
the existing agent Confirmed representation, without adding another policy store.
The bridge's role read also handles the existing reserved-PID revision refusal by
using this complete confirmed cohort.

Fresh agent startup completion and activation require the complete proof and
compare it with the existing safe confirmed firewall file after reacquiring the
original operation guard. Existing original CAS/journal and backend checks remain.
Historical persisted owners keep their exact serialization through an optional,
omitted-when-absent field. Original terminal receipts and an already-published
activation lease can be queried or acknowledged without acquiring new authority,
recreating evidence, or dispatching operations. Missing contradictory original
evidence stays UNKNOWN and is never resent.

## Current evidence

All paths below are relative to `target/nm-role-authority-20261112/`.

- `core-owner-cohort-v22/core-owner-cohort-java-full-v1`: eight-module offline
  package, 2128 passed, no failures/errors/skips. API25, crypto30, Core273,
  linux.net154, NM1508, bridge106, agent firewall18, legacy firewall14. Eight
  immutable JARs and 272 raw reports are archived in
  `artifacts/core-owner-cohort-java-v22/`; the shared Maven API was not installed.
- `core-owner-cohort-v22/interop-v1/`: actual Rust firewall view producer, actual
  Core/crypto/NM/both firewall handler implementations, and actual Rust Owner
  decoder exchange a schema2 projection and identical confirmed policy digest.
  Missing/duplicate handlers refuse before transport callbacks. Old schema1 is
  rejected as new authority. Core objects are manually bound and transport/context
  are isolated; this is not full Core DS or kernel startup acceptance. Exact reuse
  after correcting a disjoint existing unit-test fixture is recorded separately:
  every byte outside that cfg(test) function and the rebuilt normal ELF are
  unchanged. The closed interop phases were not replayed.
- `core-owner-cohort-v22/scr-current-v22/actual-scr-core-cohort-v22-v1`: current
  archived Java cohort resolves and the actual SCR lifecycle branches pass with
  fully supplied legacy dependencies and deny-only execution. The actual NM
  observer object is manually registered; actual Core/bridge remain inactive.
  Two synthetic reads, zero accepted commands. The unchanged codec companion has
  all 20 original source pins and its immutable artifact rechecked.
- `core-owner-cohort-v22/linux-current-v22-v3/`: fresh offline Linux/aarch64 normal
  and test builds, 44 startup contracts with one explicit historical exclusion,
  and all-target strict Clippy passed. The normal ELF contains none of the five
  checked fixture-only strings. The first isolated build stopped before compile
  because inherited Cargo workspace metadata was omitted; that failed fixture is
  preserved. The corrected build pins all 150 native/shared inputs, including
  unchanged parent/member Cargo metadata. No normal agent was executed.
- Host full regression v1 found an obsolete fixture assertion: it used Core
  observer unavailability to represent an uncertain NM backend, while the new
  contract intentionally permits complete original-receipt recovery without a
  new Core protocol. That 468-pass/one-failure result is retained. The corrected
  test independently removes NM observation or changes its D-Bus owner for both
  legacy and startup-bound receipts; all four cases retain UNKNOWN, exact files,
  unchanged owner-read count and zero writes. Focused v3 passes both query tests.
  The first corrected fixture attempt also exposed and retained an invalid DTO
  with an extra startup token; the final fixture uses the actual two-field API.
  Final `core-owner-cohort-native-full-v2` passes 469 tests, with zero failures
  and 17 explicit exclusions; final all-target host Clippy also passes. Both pin
  all 150 native/shared inputs. No accepted one-time system test is replayed.

The canonical candidate has 61 changed existing and 35 new files, of which three
are explicit build fixtures: 93 production/test source changes. This generation
changes 14 files from v21. The archive holds 1381 source inputs and three separate
fixture reactor entries. All 1346 original inputs were checked unchanged.

## Scope still open

Participating real NM/Core startup; original legacy CM-policy conversion and
foreign-rule preservation; ordinary DHCP/DNS/WiFi/VLAN/multi-WAN and logical modem
mapping; joint role/NAT generations and interrupted-prefix receipts; retirement,
history and cancellation-to-new-adoption; guided GUI startup/recovery and safe
source application; R017 packaging/deployment, certificate lifecycle and bulk
enrollment remain internal pending. Keep the original 14 groups and seven API2
cohorts. Only the original three external validations remain deferred.

`core-owner-cohort-v22/legacy-policy-next-source-scout-v22.json` records the next
source constraint: the old API parser skips malformed open-port/NAT clauses and
coerces some invalid Boolean text. Migration must reject partial conversion with
the original CM source bound; it must not silently adopt an empty/reduced policy.
The existing legacy service's live-rule getter is guarded and keeps automatic NAT
distinct from static NAT. Its return value does not attest arbitrary kernel-rule
ownership. No implementation or acceptance of migration follows from this scout.

At this generation's upstream gate, ten fork tips were unchanged and PLC4X gained
seven commits. Their seven functional/prerequisite groups are separately recorded
in `../upstream-logical-sync-20261112/plc4x-json-client-delta-v4/` beneath the target
stage. Existing ten integrated groups, 63 dispositions and 370 source pins remain
authoritative. New metadata is not source acceptance and reviewed tips have not
advanced. Nonfork direct merges remain skipped with their dependency mapping kept.

No commit, push, normal agent start, original runtime action, CA-custody change or
real NM/nft/sysctl operation belongs to this validation checkpoint.

`core-owner-cohort-v22/evidence-validation-final-v22.json` rechecks the exact
final source/archive, all fresh Java/native/Linux artifacts and source inputs,
the disjoint-fixture interop reuse, original 1346 inputs, 370 upstream and 20
management pins. `preservation-final-v22.json` also verifies all 18 heads,
branches, staged/ref/remote identities, current tracked diffs, eight ca9 artifacts,
f07, original 141 credential inputs plus five already-authorized overlays, the
unchanged live container start identities and gateway/two stopped VMs. The three
owned build containers are stopped; the first failed one remains for evidence.
