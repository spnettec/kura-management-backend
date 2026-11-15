# Logical upstream synchronization

User requirement: synchronize upstream improvements at a stable implementation
boundary, by behavior and function rather than blindly merging upstream history.
Preserve dirty worktrees and YOFC deviations. No commit or push is authorized.

This remains a recurring implementation gate. Recheck all18 repositories after
the current R013 ownership/confirmed-role chain reaches a coherent tested source
checkpoint, and again before release source freeze/packaging. A security or
correctness change affecting an active contract brings the relevant review
forward. This is an in-task checkpoint, not a scheduled automation.

At each smaller independently verifiable functional-group seal, also check the
advertised tips. Do not defer the first incremental check until all14 management
groups are finished. An unchanged tip needs only a preservation/tip audit; new
behavior starts its own dependency-aware review before it is applied. Keep the
complete R013 and pre-release gates above as broader compatibility checkpoints.

At each gate, read the advertised upstream default-branch tips against the last
reviewed pins first. An unchanged tip reuses the existing disposition/evidence;
do not repeat closed acceptance scenes. A changed tip starts a new bounded delta
review and evidence generation, without rewriting the earlier source seals. Group
changes by behavior and prerequisites, review actual consumers including the
seven nonforks, and adapt/test each selected group before scoped source application.
Breaking API2 removals/import migrations remain their own coordinated internal
cohort, including exports, handwritten/generated imports and downstream resolution.

Boundary: credential refactoring source seal in `credential-mount-20261112.md`.
Evidence root: `target/upstream-logical-sync-20261112/`.

## Repository scope, verified by live GitHub metadata

11 forks have upstream parents: kura, kura-artemis, kura-camel, kura-deployment,
kura-management-ui, kura-networking, kura-opcua, kura-position, kura-triton,
kura-wires and plc4x-yofc. Kura's canonical parent is `eclipse-kura/kura`; retain
the configured remote and account for the historical `eclipse/kura` redirect.

7 nonforks: kura-cloud, kura-container, kura-docker, kura-management-agent,
kura-management-console, kura-yofc-runtime and yofc-iot. Skip direct parent merging.
Cloud/container source origins and all consumer relationships still need analysis;
GitHub nonfork status does not mean no upstream-derived code or dependency impact.
The console origin still uses the historical backend repository name; do not
replace or rename that remote as part of this task.

## Work sequence

1. Preserve18 repository HEAD/branch/full dirty states, current source seal and
   remote configuration. Read current per-repo instructions and intentional
   upstream exclusions before selecting changes.
2. Fetch verified upstream refs without pruning, tags, checkout changes or
   automatic GC. Capture old/new refs and canonical parent/default branches.
   Cached ref counts do not prove current upstream state.
3. Group incoming changes by actual behavior: correctness/security fixes,
   configuration/lifecycle, network/ownership, protocol/driver, build/dependency
   and UI improvements. Record prerequisite commits and local equivalent fixes.
4. Use per-child code graphs and actual POM/MANIFEST/build consumers to derive the
   dependency order. Core API changes precede affected siblings/YOFC; PLC4X is an
   independent prerequisite for its YOFC consumers. Docker/management/nonfork
   consumers still get compatibility review.
5. Integrate selected behavior in isolated worktrees or copies including the
   relevant dirty overlays. Keep existing working trees intact. Do not use an
   ours merge to consume unrelated ancestors, reintroduce removed subsystems or
   undo Maven3/MBP/Java/GWT compatibility and YOFC fork policy.
6. For each group, record upstream SHA(s), problem and resulting behavior, selected
   or adapted source, prerequisites, local deviations, cross-repo consumers,
   validation and disposition. A clean apply or ancestor relationship is not
   evidence that the behavior is correct or already integrated.
7. Apply validated scoped adaptations while retaining before copies and exact
   evidence. No commits/pushes, UNKNOWN replays or old one-time acceptance scenes.

At the initial ref-fetch boundary, functional integration and dependency mapping
were still pending. Subsequent sections record the actual applied groups and their
validation; metadata alone is not synchronization completion. The original
management requirements and three external deferred are preserved separately.

## Current evidence after ref fetch

All11 verified parent fetches completed successfully. Each preserved HEAD,
branch and full dirty status, with no prune/tags/GC/checkout. The fetched default
branches are `develop`; exact tips and merge bases are frozen in the evidence.
54 nonmerge candidates: core9, management-ui4, networking1, opcua1, position2,
wires1, PLC4X36. Artemis/Camel/deployment/triton have no new default-branch DAG
candidates in this snapshot; this does not exempt them from compatibility review.

306 current POM declarations yielded327 cross-repository declarations. Reverse
sibling/YOFC edges into `kura-dev-runtime` are assembly dependencies; edges into
`kura-endpoint-tests` are test fixtures, not core-implementation compile cycles.
The matrix retains profile/dependencyManagement context rather than claiming an
effective runtime graph. Agent's local `[patch.crates-io]` maps the versioned
management-protocol crate to the console repository; release packaging requires
the published checksum-verified crate. The architecture graph returned counts
rather than cross-repo dependency details, so exact build-config fallback was used.

First source decision: upstream networking `aeb4220f9791` strips `@parent` from
interface names. Current `findIfname` already does so, including the YOFC IPv6
path, with empty working diff and history under `5df41df`. No production patch is
needed; upstream regression coverage can be assessed separately. Do not overwrite
the larger local class with the smaller upstream version.

Next review groups include strict identity boolean parsing, API removals and
consumer2.0 import cohorts, watchdog API/GWT status, build pins and shared PLC4X
protocol/generator effects. Command/LED-related subjects require inspection against
intentional removals; subjects alone are not skip or import decisions.

## Applied group: strict identity password flags

Selected upstream `4c27d2cbaa6269c6697665b14cbd4279d5fea94b` behavior has been
adapted and applied without a commit. Before-copy and candidate each contain2107
current source inputs, including dirty overlays. All existing selected inputs
matched before applying7 files; the earlier DS XML dependency repairs remain.
The four Java changes preserve upstream source exactly. Gson remains managed
2.11.0, becomes provided rather than test-only in this module, and the handwritten
manifest explicitly imports Gson, annotations and stream. No Tycho/POM replacement,
API package version bump or embedded Gson was introduced.

The same corrected48 new contracts produced22 failures before:16 string
coercions and6 missing/null V2 flags returned500. After the repair all69 module
contracts and bundle packaging pass. V1 optional missing/null semantics and valid
V2 true/false writes remain unchanged. A failed assertion in the first test version
mistook an error object's toString for JSON; its reports are retained, and the
corrected test checks the serialized message. This was a test correction, not a
production workaround.

An isolated Equinox probe uses a new owned storage directory and current runtime
JARs read-only. It initializes, resolves and loads without starting the framework
or application bundles, checks all3 Gson package wires to2.11.0, rejects32 invalid
values, and preserves8 real booleans. The candidate stays RESOLVED (state4), with
zero active application bundles. Earlier probe attempts that started the framework
triggered candidate lazy activation; logs and sources are retained. Every probe
terminated; no current runtime/configuration/identity service was started or
modified. The successful probe is classloader validation, not full system or
historical HTTP/MQTT acceptance. No old one-time acceptance was replayed.

Evidence: `target/upstream-logical-sync-20261112/identity-booleans/`, especially
`applied-to-working-tree.json`, `before-corrected-regression.json`,
`after-module-package-v2.json`, `bundle-package-check.json`,
`osgi-probe-v4-result.json` and `osgi-probe-inputs.json`.

All18 HEAD/branches still match the prefetch baseline. This group remains source
integrated only; the running deployed image/runtime has not been replaced. API
removal cohorts, watchdog, other build/PLC4X groups and the original management
scope remain internal pending, separately from the three external deferred.

## Compatibility cohort and LED behavior

API removal/import changes have been reviewed together; see
`target/upstream-logical-sync-20261112/api-cohort/review.md`. Keep existing1.x
channel/driver/comm/message compatibility in this synchronization group. Pure
breaking API2 removal/import switches remain a deliberate internal migration,
not external deferred or integrated functionality.40 handwritten declaration files
across9 repositories and generated YOFC imports require coordinated treatment.
GWT DriverDescriptorService-only feature detection and position USB2 import are
already source-equivalent. Cloud's remaining deprecated getter assertion is test
code; SystemService/GwtSession/entity getKuraVersion usages are separate contracts.
Do not apply broad text replacement.

Core LED `76a118ec59a1` is relevant despite the GPIO removal: local Linux sysfs LED
and log fallback remain. The actual bug reproduces with a recording LedManager:
FAST_BLINKING has a307ms median before, incorrectly matching slow cadence. After
selecting each status's own on/off periods, all20 status module contracts and MBP
package pass; the2 new timing tests require fast<240ms and slow>=250ms and ensure
worker termination.3 selected files were applied after before-content comparison.
The upstream GPIO-based test was adapted to avoid GPIO or real sysfs access.
An initial fixture import compile error is retained; final results are
`led-status/before-regression-v2.json` and `after-module-package-v2.json`. No public
API, live LED, deployed runtime or historical acceptance changed.

Upstream Jackson2.21.8/annotations2.21 is behind the current managed
Jackson2.22.0/annotations2.22; preserve the newer local set and separate tools.jackson3
coordinates. GWT prefs pin review is undergoing isolated module build validation.

## Corrected GWT applicability decision and Command exclusion

After full candidate build, applicability review found upstream's GWT2.13.1
differs from fixed local2.12.2. The current published compiler POM/resolved tree
has no affected Equinox preferences -> OSGi prefs dependency. The inherited BOM
already pins prefs1.1.2. Therefore do not apply the candidate extra gwt-dev
dependency. Both6-permutation modules and MBP packaging passed in isolation, and
the bundle contains only the existing3 runtime embedded JARs. This is positive
build evidence for the candidate, not evidence the upstream workaround is needed.
See `gwt-build-pin/decision.md`, POMs and dependency tree. No real UI source was
changed by this experiment.

Command empty-upload05b16135df94 only changes the removed command form/archive
handler. Live FileServlet routes support deploy, configuration snapshot and asset,
then reject unknown routes; the command handler/service is absent. Exclude the
change under the explicit Command removal boundary. Do not restore the subsystem.

JWT eb0c9a1eac40 is a security/lifecycle improvement, not a generic dependency
change. Its3 existing primary Java files match the upstream parent. New family
tracking, local XML metatype/localization and JUnit5 conversion need an isolated
adaptation. The old retention property and default must remain operable from the
UI without manual environment or snapshot edits. No JWT production patch has yet
been applied.

## Applied group: JWT refresh token families

Selected core `eb0c9a1eac40446fba13e178d2aae5a2bd52d6e5` behavior has been
adapted and applied to22 selected files, after before-content checks. Four primary
Java classes implement family ID/generation, replay revocation, owner/password
binding, identity-removal revocation, issue-failure rollback and periodic
expiration cleanup. Existing three Java inputs matched the upstream parent; the new
tracker and changed JWT service/constants retain upstream source. RestServiceOptions
retains the deployed `auth.jwt.used.refresh.token.retention` property key and
300-second default while exposing the new family-retention semantics internally.
The manual XML metatype/AD key/minimum remain unchanged, avoiding snapshot or
environment migration. JWT remains disabled by default.

Local adaptations preserve the Maven3/MBP build, handwritten manifest, XML metatype,
DS dependencies, endpoint fixture topology and Jupiter. English/Chinese labels now
explain one-use rotation, full-family replay revocation, password/identity changes
and mandatory login after service restart. The preexisting fused `authJwtEnabled`
resource key was split onto its own line in both languages. Five contracts verify
legacy values0/300/1177, default300, both language labels and the unchanged UI AD.
No CA signing key or production configuration was introduced.

A private counterfactual fixture, identical against both revisions, produced4/4
failures before and4/4 passes after: family claims/rotation, descendant revocation
after reuse, password change and service restart. It stays in the evidence copies,
not the original repository source. The final module run passes154 contracts:
150 maintained contracts plus those4 private counterfactual cases. Upstream unit
family coverage was ported to Jupiter without JUnit4/Vintage. Existing HTTP
coverage is retained; five new family cases validate replay/descendant revocation,
password change through login/XSRF/session HTTP, removal/recreation, expiry with
clock skew, and lifetime-vs-retention through fresh loopback Jetty/Jersey with real
fixture identity and token cryptography. Mocked SCR registration/target arrival
remains a test-fixture boundary, not real system wiring acceptance.

MBP bundle and fixture JAR packaging pass. A separate Equinox init/resolve/load
probe loads all4 changed classes, reads legacy retention987 unchanged, and rejects
both replay and its descendant using the bundle's actual classloader. Application
active bundles stay0, candidate RESOLVED, and the framework exits. This is a fresh
classloader contract; no old remote/VM one-time acceptance was replayed.

Evidence: `target/upstream-logical-sync-20261112/jwt-family/`, especially
`adaptation.json`, `applied-to-working-tree.json`, `before-regression.json`,
`after-module-package-v2.json`, `test-summary.json`, `osgi-probe-result.json`,
`osgi-probe-inputs.json` and `all-baseline-inputs-audit.json`. Before/candidate
copies preserve2110 source inputs; all original baseline inputs outside the selected
changes still match. The three applied groups now have32 exact source pins in
`applied-source-pins.json`. No original target/runtime, deployment, commit or push
changed. Restart refusal is a resulting JWT behavior for future deployment; the
currently running old service has not been restarted or replaced.

Remaining upstream groups: core/GWT watchdog observation,36 PLC4X candidates and
nonfork dependency/source-origin closure. Deliberate API2 compatibility migration
stays internal pending; original14 management groups and3 external deferred remain.

Final three-group preservation audit:18 HEAD/branches,32 applied source pins,146
credential source pins,8 ca9 artifacts and f07 manifest all match. Full current
dirty status is captured in `three-groups-preservation-audit.json`. Old Docker
console/postgres retain their IDs and healthy state, Orb gw-001 runs, and both
Parallels VMs remain stopped. All owned Maven/Jetty/Equinox probes have terminated.

## Applied group: watchdog API, native lifecycle and GWT status

Coref7717a30cb7b and UI4c9b9b8ae60f have been functionally adapted and applied
to35 selected files after2750 before-input checks.154 tests, both6-permutation
GWT modules and bundle packaging pass. Additive watchdog2.1/default unsupported
status preserves old providers; GWT handles old/missing API/provider, localizes
state labels and escapes paths. Shared JNA5.19.1 remains provided, both watchdog
policies default false, and the emulator never writes a physical device. Local
review also fixed native return validation and shutdown/update/reboot cancellation
races with retained failing-before contracts.

Actual new/old API classloader probes and a Java11 old-provider binary pass. The
GWT receipt -> configuration -> XML codec/password-source capability closure uses
real unchanged captured providers, not synthetic capabilities. Linux ARM64 regular-
file ABI probes pass, including actual OSGi classloaders with default JNA loading.
The stricter noexec/nounpack sample fails and is retained as a limit; it is not a
passing full sandbox/hardware acceptance. Native loading under a chosen release
sandbox stays in the internal release gate. Physical/amd64/power boundaries remain
the original three external groups.

Details: `docs/reviews/watchdog-upstream-review-20261112.md`; evidence under
`target/upstream-logical-sync-20261112/watchdog-observation/`.35 source inputs are
sealed; global applied source pins now67 across4 functional groups. Originals'
target/runtime and deployed services were not replaced. Next:36 PLC4X candidates,
nonfork origin/dependency closure and the deliberate internal API2 cohort. The
parent14 management groups are preserved.

Preservation audit `target/upstream-logical-sync-20261112/four-groups-preservation-audit.json` verifies all18 HEAD/branches,67 applied source pins,146 credential source pins,8 ca9 artifacts and the frozen f07 manifest. Full current worktree fingerprints for18 repositories were captured (not retrospectively compared with nonexistent initial diff hashes). The original console/PostgreSQL containers and Orb gateway remain running; both Parallels VMs and all5 new owned native-probe containers are stopped.

## Applied PLC4X Go group and remaining scope

23 incoming Go candidates are now applied as29 selected files after a11953-input
before check. Four affected packages pass581 top-level tests;3 before leaf
counterexamples pass after. Full Go build, module verification and isolated Go
emitter packaging pass. Default full vet still fails with31 identical before/after
diagnostics; affected-package vet with stdmethods excluded passes. The2 existing
skips,2 explicit multicast exclusions and generator packaging limits are recorded
in `plc4x-go-upstream-review-20261112.md`, with full evidence under
`target/upstream-logical-sync-20261112/plc4x-review/`.

This changes no Java PLC driver/API, shared SLMP protocol, local POM/version or
running deployment.96 applied source pins and all protected credential/remote
seals match;18 HEAD/branches remain unchanged. PLC4X still has13 candidates to
review/integrate (4 build/dependency,9 C#/shared SLMP), followed by complete7-nonfork
origin/consumer mapping and deliberate API2 migration. Cloud/container history
confirms they were split from Kura despite GitHub nonfork metadata. Metadata clues
are not a completed source mapping. The full original management scope remains.

## Applied shared SLMP/C# cohort

9 additional PLC4X candidates are now applied as245 scoped files,183 of which only
normalize generated C# final newlines. Current mspec,Java/Go bindings,word/bit wire
contracts and C# runtime are reviewed together.1134 .NET cases,85 Java units plus2
synthetic driver cases,69 top-level Go cases/selected vet,generated-model checks
and the actual Maven-to-NuGet build path pass. The full Go consumer compile also
passes after the generated parser-signature change. See plc4x-slmp-upstream-review-20261112.md and evidence
under target/upstream-logical-sync-20261112/plc4x-slmp/.

Local adaptations fix automatic .B Maven-to-NuGet version conversion and bounded
nested compiler-process/output waits. All failures/corrections are retained.341
cumulative source pins,146 credential pins,8 ca9 artifacts and f07 freeze match;
18 HEAD/branches remain unchanged. PLC4X source integration has4 root dependency
candidates left. Complete its full root build/shared-cache install/YOFC consumer
checks,7-nonfork origin mapping and deliberate API2 migration before declaring
this synchronization phase complete. The original management scope remains.

## Applied PLC4X root dependencies and nonfork closure

Four final PLC4X candidates are applied as eleven version changes in the root POM.
All36 frozen PLC4X candidates now have functional source integration. Full97-module
serial private/shared-cache builds pass with tests skipped. Selected Java tests
pass2501 with6 existing skips; C# passes1134. Eclipse compiler/Build Helper and
JaCoCo execution are verified. Guava/Joda are managed-only in the resolved selected
closure; XML optional transformations were not run.334 installed PLC files match
their build sources, and a fresh YOFC shared-cache build embeds37 matching PLC
JARs. Details and limits: `plc4x-root-dependencies-review-20261112.md`.

All7 nonfork origin/immediate-consumer dispositions are now recorded in
`nonfork-upstream-relations-20261112.md`. Cloud/container are traced to original
Kura paths; Docker artifact-copy and runtime-export relationships supplement the
POM graph. CloudCat's relocated official kura-apps source is checked at a frozen
pin:4 Java token streams and2 descriptors are equivalent, with local manifest
ranges preserved. No functional CloudCat patch is missing in this bounded review.

The54 frozen fork candidates include41 source-integrated SHAs across7 groups.
The7 API2 removal/import candidates remain a deliberate internal compatibility
cohort; equivalent/not-applicable/excluded dispositions are retained separately.
342 applied source pins,146 credential inputs,8 ca9 artifacts,f07 seal and all18
HEAD/branches match. Staged diffs are unchanged; only the scoped PLC root POM
changes tracked content relative to the prior six-group audit. Old Docker/Orb/
Parallels states are preserved. No source checkout,commit,push,clean,deployment
switch,UNKNOWN redispatch or closed one-time replay occurred. The original14
management groups and original3 external validation groups remain on the ledger.

## Incremental gate: network table request lifecycle

A read-only advertised-tip check found ten fork default branches unchanged and
one new management-ui commit,23de6479b8c745f00b86d43124001a1887014c00, after the
previous frozen UI tip4c9b9b8ae60f. The GitHub comparison confirms exactly one new
commit and one changed file. No fetch, ref, checkout or remote modification was
needed; the connected GitHub reader supplied the pinned comparison, commit patch
and verified Git blob3123089b5aa71571fbdcb7fc5d0bb550dcac1fe9.

NetworkInterfacesTableUi now loads its interface list through the existing
RequestQueue and registers its sort handler once during table creation. Existing
selection, dirty-edit handling, ordering and Java11 instanceof/explicit-cast
syntax remain. The local class matches upstream tokens after this one explicit
Java11 normalization. RPC/backend/API contracts and Docker UI artifact paths are
unchanged. The previous nonfork source/consumer relationship map still applies.

The actual table and unchanged actual RequestQueue run with the same headless
widget/RPC doubles before/after. Three failures before demonstrate premature
spinner release, five sort handlers after four refreshes, and a failed list
request hiding a following request's spinner. All five cases pass after,
including empty-result and session-selection controls. This is JVM behavior
evidence, not browser DOM or live NetworkManager acceptance.

An initial isolated module build failed because the shared API JAR predates the
already applied watchdog API; its log is retained. A fresh private current-API +
UI reactor passes Java11 server compilation, both GWT modules with six permutations
each and MBP packaging. No candidate API was installed in the shared Maven cache.
The bundle retains its three private runtime JARs, with no compiler or prefs
classes added. The original deployed runtime and target outputs remain.

All1233 current UI/API/build-support inputs were checked before applying exactly
one selected source file. All18 HEAD/branches/staged diffs and tracked changes
outside that file are unchanged. The immutable old342-pin manifest is retained;
`target/upstream-logical-sync-20261112/applied-source-pins-after-ui-delta-v1.json`
is the new343-pin authority. Eight functional groups now account for42 integrated
SHAs. The original54 candidates plus this incremental candidate total55 reviewed
incoming records. Seven API2 migrations remain internal pending. Nonfork direct
merges remain skipped while their source/consumer relationships stay reviewed.

Evidence: `target/upstream-logical-sync-20261112/stage-tip-recheck-v1.json` and
`management-ui-delta-v1/{upstream-comparison.json,adaptation-review.json,
behavior-before-v1.json,behavior-after-v1.json,gwt-package-v2.json,
applied-to-working-tree.json,preservation-audit.json}`. The checkpoint now points
to `reviewed-upstream-tips-after-ui-delta-v1.json`; later gates compare with these
reviewed tips.20 management pins,1206 untouched R013 original inputs,141 exact
credential inputs plus five prior overlays,ca9 eight artifacts and f07 still
match. Console/PG remain healthy, Orb runs and both Parallels VMs/five retained
management proof containers remain stopped. No deployment, commit/push or old
one-time acceptance occurred. The original management scope remains active.

## Reconfirmed recurring gate and unchanged tips

The user's recurring logical-sync requirement is retained in the active
implementation ledger. The read-only check in
`target/upstream-logical-sync-20261112/stage-tip-recheck-user-stage-policy-v2.json`
finds all11 fork advertised default branches unchanged from the reviewed tips.
All18 HEAD/branch/status/tracked/staged/ref/remote fingerprints match before and
after that check. The supplementary relocated CloudCat source also advertises
the same reviewed kura-apps tip; see
`supplementary-origin-tip-user-stage-policy-v2.json`. The seven nonfork origin and
consumer dispositions remain applicable to this unchanged source delta.

No new fetch, source integration or closed acceptance replay is needed for this
unchanged delta. The current applied authority remains the343-pin manifest and
eight functional groups/42 selected incoming SHAs; this is a behavior ledger,
not a claim that Git ancestry has advanced. Seven API2 migrations stay internal
pending. The unapplied R013 template checkpoint adds crypto1.5/configuration1.9
contracts; its API-manifest overlap is explicitly recorded in
`target/nm-role-authority-20261112/upstream-candidate-overlaps-template-v3.json`.
Future application must preserve the older342/343 seals and create an explicit
overlay. Original management requirements, dirty trees and external limits remain.

The next tested functional seal, original R013 template export/deposit/preparation,
also rechecks all11 advertised fork tips unchanged in
`stage-tip-recheck-r013-template-export-v3.json`. The18 Git fingerprints match
before/after this read-only check. This is new R013 implementation evidence, not
another upstream source cohort; the343-pin/eight-group/42-SHA authority remains.
Its unapplied overlay is now `upstream-candidate-overlaps-export-v4.json` under
the R013 evidence directory. Continue logical delta/consumer review at later seals.
