# Configuration value shape — 2026-11-07

Round ID: 1f7815c6. Continues b5c0e9c7 and the complete 6336f5c7 ledger.
Overall goal remains active; this is selected implementation and validation.

## Entry and preservation

All 18 HEAD/branch/full porcelain states matched the previous current manifest.
Its 549 source, 125 artifact and 1332 evidence hashes matched at entry. The 674
source/artifact inputs were copied before edits/tests. Additional clean tracked
inputs were reconstructed from their unchanged entry HEAD (not historical dirty
files); see additional-before-copies.json. New output never overwrote historical
one-time acceptance records. No commit/push/reset/clean, runtime deletion, package
installation or UNKNOWN retry occurred.

## Defect and contract

Legacy Kura XML/default and GWT paths treat cardinality 0/+1/-1 as scalar; revision
and bridge paths had instead required every nonzero value to be a container.
True one-element arrays also rendered through String.valueOf as JVM object IDs.

AD now has a default scalar-cardinality contract preserving legacy implementations.
The native OSGi provider adapter overrides it: every nonzero native cardinality
retains standard container semantics, including absent/default/localized fields.
The effective shape combines this metadata contract with existing values. Explicit
legacy arrays/collections of capacity one remain containers. No CM record migrates.
Core conditional writes reject scalar/container swaps before admission; revisions
bind the metadata shape policy as well as values (canonical token domain v2).
Capacity limits and scalar validation remain authoritative.

Bridge views include optional valueShape SCALAR/ARRAY/COLLECTION independently of
cardinality, even for redacted/absent fields. Bridge patches, GWT DTO/renderers and
React use this shape. React falls back to visible old-server values when available.
Secret contents remain redacted. GWT copy operations preserve the new flag.
This unreleased API addition requires the matching API/core/bridge/GWT bundles
together; formal install/upgrade minimum-version enforcement remains pending.

## Validation

- Modification-before regressions reproduced legacy +/-1 scalar rejection and
  one-element primitive array object-ID rendering. Both failure logs are retained.
- API/util/core reactor install succeeded; core configuration suite passed108
  before the last added tests. Final affected constraints/revision/provider/admission
  run passed23, including native/localized defaults and rejection without burning
  a revision. The added admission test first had a test-fixture type compilation
  error, then correctly hit pending observation when mutating a fixture before
  observing its prior submission. Both logs retained; fixture corrected, production
  pending/UNKNOWN protection unchanged.
- Bridge reactor install passed65. GWT full login/denali compilation and package
  install passed; web tests112 passed. No Rust/protobuf production source changed
  and this round does not claim a new Rust or formal deb acceptance.
- React generated OpenAPI types, typecheck and production build passed. Build
  retains the existing large-chunk warning. Four new Playwright cases passed;
  one uses the actual editor in Chinese and English with stubbed session/tasks,
  confirming exactly one typed write. This is not a live center/agent acceptance.
- Fresh complete native Kura launched only through IDEA, PID60038, independent
  profile d00e6af1: inventory283 includes launcher/framework, actual OSGi282 includes
  framework. Original admin/admin login passed with a fixture-only password-gate
  setting. Production authentication unchanged.
- New dedicated ConfigurableComponent/DS fixtures exercised XML +/-1 scalars and
  real capacity-one arrays/collections, plus native provider +/-1 defaults. Actual
  registered bridge handle rejected a shape swap and retained the revision, then
  delivered one scalar update to DS. It is a direct registered-service frame test,
  not a new authenticated agent/UDS-to-center transport proof.
- Original GWT submitted two confirmed updates: a legacy scalar, then native
  one-element array and collection. Real DS callbacks retained their types. The
  latest actual Core snapshot was parsed: six expected values and scalar/ARRAY/LIST
  shapes matched, whitespace preserved. This is save/export evidence; no restore,
  restart, recovery or physical/network acceptance is inferred.

## Current state and ledger

New native60038 and Vite59495 stopped; all fixtures/state retained. Original Orb
services still active at PIDs244516/244911/286620/268. Ubuntu and Windows Parallels
remain stopped; neither was booted this round. Both VM validations of this new
configuration shape contract remain internal pending. The previous firewall
read/countdown selected scope remains closed and was not replayed.

Remaining internal work: full component/metadata/GWT editor coverage (including
add/remove/empty containers and snapshot restore of all field kinds), configuration
shape live Linux/center-agent path, formal API upgrade packaging; production and
external PID recovery and empty/partial/inconsistent CM policy; NM automatic NAT
and other-writer ownership/adoption/retirement profiles; encrypted center firewall
tasks and bilingual remote editing with affected-connection proof; actual early
boot, install/upgrade/bootstrap/handoff; consistent DB/artifact/key backup, rotation,
migration and release. Full firewall product and full Kura recovery remain false.
External deferred remains real NM/cloud-init/physical hardware, amd64 and disk-full/
hard-power. Historical CONFLICT attribution and the prior two evidence gaps and
6336f5c7 missing XML originals remain explicitly unresolved. Java is available.

Evidence: ../evidence/configuration-value-shape-20261107.json.
Private output: target/configuration-scalar-edit-20261107.
