# PLC4X SLMP/C# functional upstream review, 2026-11-12

Status:9 upstream candidates applied as245 scoped files after11957 exact before
input checks.219 existing before copies and26 new-file absences are accounted for.
183 changes are generated C# final-newline normalization required by the selected
generator check; the remainder covers behavior, regenerated bindings and contracts.
No commit, push, deployment replacement or historical one-time acceptance replay.
The parent management goal and its original14 groups remain active.

## Behavior and local adaptation

C# gains scoped little-endian integer/floating buffers and matching generated
parse/serialize scopes; callers' byte order is restored. SLMP3E framing validates
route/length, word tags validate selection/address limits, and batch read/write
supports numeric word and BOOL bit devices. Requests are serialized per connection;
timeout/cancellation makes an uncorrelatable3E connection unusable rather than
sending the next operation through it. Existing word behavior remains covered.

The two shared SLMP mspec changes carry subCommand through the discriminated
request and compute bit payload length as ceil(points/2); the follow-up removes a
duplicate child parameter. Current-root Java/Go emitter and protocol prerequisites
were installed in a private Maven repository, with the host cache only a read-only
file-repository fallback. The embedded mspec hash matches current candidate source.
Java bindings were regenerated in that root reactor. The one Go SLMP execution
then used those exact fresh artifacts, followed by formatting and consumer checks.
No stale cached protocol was silently substituted. Handwritten Java word-driver
APIs did not require a change; generated parser signatures and helpers did.

Independent bit/word vectors cover odd/even nibble packing, complete parse
consumption and unchanged word frames in Java/Go.16 added C# cases cover real
request-builder bit reads/writes, invalid nibbles/padding/lengths, validation before
dispatch and tag bounds/round trips. The SLMP manual specifies the high nibble first
and zero padding for an odd point count; these are hardware-free wire contracts.
[Mitsubishi SLMP reference, Device Access](https://dl.mitsubishielectric.com/dl/fa/document/manual/plc/sh080956eng/sh080956engl.pdf).

Two additional issues were repaired in this cohort:

- Maven's YOFC2.0.0.B-SNAPSHOT was rejected by NuGet in a real before probe. The
  existing managed build-helper regex-property goal now derives2.0.0-B-SNAPSHOT
  for both dotnet build and test. Maven coordinates, manual Maven3 POM layout and
  upstream assembly-version policy remain unchanged.11 actual packages have the
  normalized version; no operator environment or manual version edit is required.
  [Build-helper goal](https://www.mojohaus.org/build-helper-maven-plugin/regex-property-mojo.html).
- The new floating-model process fixture hung beyond its declared child-exit
  timeout. Process tree/native sample and the eventual failed run are retained.
  Output collection had no complete deadline. The fixture now bounds exit plus
  both streams, and both nested commands disable reusable build/compiler servers.
  Standalone generator tests finish in7.7s and the full solution in17.6s afterward.
  This proves the repaired validation path; it is not a full native CLR root-cause
  claim. [CLI build-server control](https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-build-server).

## Evidence and verification scope

- Isolated existing-target SDK8.0.428:official archive SHA512 verified; no global
  SDK installation or framework-target change. Not a production .NET deployment.
- .NET full solution:1134 pass,0 fail/skip;16 projects compile with0 warnings/errors.
  Its148 generator cases include compiling/running emitted code. The corrected
  Maven entrypoint also completes generated-model checks, build and the same1134
  tests, producing11 NuGet packages at2.0.0-B-SNAPSHOT. No packages were published.
- Python model-check unit tests and actual3-protocol generated-model check:pass.
- Java SLMP:85 unit cases plus2 synthetic driver-suite cases, package/coverage
  verify:pass. Only this affected Java cohort is tested, not all PLC4X tests.
- Go SLMP:69 top-level cases,212 pass events including subtests; targeted default
  vet passes. Four golden cases pass; original generated bindings fail exactly2
  bit-write leaves with the same support libraries, not compile errors.
- Full Go consumer compile after the parser-signature change:pass in161.65s,
  recorded in go-full-consumer-build.json;every in-repository Go consumer compiles.
- NuGet audit reports0 vulnerable packages across16 projects at query time. This
  is package-feed evidence, not a proof of every runtime/OS security property.

Earlier failures remain explicit. Initial .NET tests had2 Modbus timing failures;
corrected before controls261/261, isolated after261/261 and the current full runs
pass. The first before control lacked checkout-root/XML fixtures and is separately
labeled, not treated as a production regression. New test-source API mistakes in
the first C#/Java contract revisions were corrected; compile-error logs remain.
The305.9s generator hang/failure, native sample, owned build-server shutdown and
verified idle-node termination remain, rather than being hidden by a later pass.
The previous full-Go default vet31 baseline diagnostics are not declared fixed by
this selected SLMP vet check.

Evidence root:target/upstream-logical-sync-20261112/plc4x-slmp/.
Key manifests:source-baseline.json, selected-source-summary.json,
validated-source-manifest.json, validated-source/, before-selected/,
fresh-generator-inputs.json, bit-wire-before-after.json,
nuget-validated-packages.json, dotnet-test-v4-summary.json,
dotnet-maven-final-verify.json, java-slmp-final-verify-v2.json,
go-slmp-final-tests.json, go-slmp-vet.json and applied-to-working-tree.json.
The20 unchanged-before generated files additionally recovered from the captured
HEAD are byte-checked against their before hashes; all219 before copies match.

At application all341 cumulative source pins,146 credential source pins,8 ca9
artifacts and frozen f07 manifest match;18 HEAD/branches remain unchanged. Current
running services and retained VM/acceptance fixtures remain. Shared-cache PLC4X
root installation, affected YOFC consumer validation and release integration are
still pending with the4 remaining root dependency candidates. Physical PLC/amd64
and power-loss limits remain within the original3 external groups.7-nonfork
source-origin/consumer closure, deliberate API2 migration and original management
implementation work remain internal pending.
