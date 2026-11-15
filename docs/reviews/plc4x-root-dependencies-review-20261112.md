# PLC4X root dependency cohort, 2026-11-12

Four remaining upstream candidates (`ba058f832a01`, `530f7bf027a1`,
`9404ddf5555e`, `9b6c15c8f4ad`) are adapted as eleven version-value changes in
`plc4x-yofc/pom.xml`. This completes source integration of the 36 frozen PLC4X
candidates. It does not complete the parent management goal or API2 migration.

| Dependency/build component | Before | Applied |
| --- | --- | --- |
| Logback | 1.6.5 | 1.6.6 |
| JaCoCo | 0.8.15 | 0.8.16 |
| Byte Buddy | 1.18.16 | 1.18.17 |
| Joda Time | 2.15.0 | 2.15.1 |
| Felix framework | 7.0.5 | 8.0.0 |
| Maven Enforcer | 3.6.3 | 3.7.0 |
| Guava | 33.7.2-jre | 33.8.0-jre |
| Plexus compiler API and Eclipse provider (two values) | 2.17.1 | 2.18.0 |
| Build Helper | 3.6.2 | 3.7.0 |
| XML Maven Plugin | 1.2.2 | 1.3.0 |

The Maven3 schema/modelVersion4.0.0, YOFC `2.0.0.B-SNAPSHOT`, Jackson3.2.4,
local build policy and earlier 341 source changes are preserved. An isolated
11983-input copy was used; only root pom.xml differs from that stage's before
snapshot. Its before/after hashes and exact eleven-line patch are retained.

## Validation and cache installation

Evidence: `target/upstream-logical-sync-20261112/plc4x-root-deps/`.

- Full serial `-Pwith-java -pl '!website' install -DskipTests --fail-at-end`
  succeeds for97 modules with JDK21/Maven3.10, first in a private cache, then in
  explicit `/Users/heyoulin/.m2/repository`. These builds compile/package with
  tests skipped; they are not all-PLC unit or physical-device acceptance.
- Selected SPI values/buffers, OSGi, OPM and OPC UA tests report2507 invocations:
  2501 passed,6 existing OPM skips,0 failures/errors. Actual test classpaths contain
  Byte Buddy1.18.17 and Felix8.0.0. JaCoCo0.8.16 prepare-agent executes. Felix8's
  own manifest requires JavaSE11; its declaration is test-scoped in plc4j/osgi,
  and this changes no deployed Kura framework.
- A forced Eclipse compilation of plc4j-api succeeds with the Plexus2.18.0
  provider/ECJ on JDK21. Only an owned-copy source mtime was temporarily changed,
  then restored; source bytes are unchanged.
- The real Maven C# verify path passes1134 cases, generated-model checks and
  Build Helper3.7.0's `.B`-to-NuGet regex-property execution. The existing private
  .NET8 SDK was reused. This is development validation, not .NET deployment.
- The resolved97-module Java dependency tree observes Logback1.6.6, Byte Buddy
  and test-only Felix. Guava/Joda are managed versions absent from this selected
  resolved closure: no actual Guava/Joda loading acceptance is claimed. The OPC
  UA selected tests are a consumer sanity check, not proof those libraries loaded.
- XML1.3.0 descriptor/help loading passes. The source maps its transformations
  to optional C and update-generated-code paths; those transformations and their
  external downloads were not executed by the default Java validation.
- Before shared-cache installation,1099 existing PLC4X snapshot/metadata files
  (186551848 bytes) were copied and hash-checked under `shared-cache-before/`.
  All334 installed PLC4X files match their current build sources. The installed
  root POM matches the applied worktree POM.
- A fresh335-source YOFC copy, with no stage repository fallback, packages
  `yofc-iot-apps-opcuaserver -am` against the actual shared cache. All37 embedded
  PLC4X JARs match this installation. No original YOFC source or target changed.

## Preservation and remaining work

342 cumulative applied source pins,146 credential inputs,8 ca9 artifacts and
the frozen f07 manifest are preserved. All18 repository HEAD/branches and staged
diffs are unchanged. Original source changes outside the selected root POM,
target/runtime trees and running deployments were retained. No commit/push,
reset/clean, old one-time acceptance replay or UNKNOWN redispatch occurred.

The seven nonfork projects still require their origin/consumer disposition to be
recorded; CloudCat's relocated official source is included in that analysis.
API2 removals and import-only updates stay a deliberate internal compatibility
migration, not external deferred or silently completed commits. The original14
management groups and original3 external validation groups remain on the ledger.
The prior full default Go vet's31 equal before/after diagnostics remain recorded.
