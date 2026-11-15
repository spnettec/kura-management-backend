# PLC4X Go functional upstream adaptation, 2026-11-12

Status:23 selected upstream candidates applied as29 source/module-lock/test files.
No Git ancestry merge, commit, push or deployed replacement. The parent management
goal remains active. Remaining PLC4X candidates:4 build/dependency and9 C#/shared
SLMP candidates. Nonfork source-origin/consumer closure and deliberate Kura API2
migration remain internal pending, alongside the original management scope.

## Selected behavior and compatibility

- KNX connection metadata no longer type-asserts every service family to an
  unsupported interface. Device addresses compare by value; their connection map
  uses a guarded numeric address key. Device-connect results are actually used.
- A device response is passed out of the codec expectation worker before sending
  its ACK, avoiding waiting on its own expectation lock. Only connected individual
  data addressed to this client is acknowledged. Group responses match destination.
- Unsolicited tunnelling handling lives through the connection lifetime, survives
  the Connect caller's cancellation and terminates on close. Connection logging
  options reach codec/reader/browser. Ordinary unclaimed ACK logging is trace.
- Device memory decoding accounts for bare values, element width and empty reads.
  Untyped group tags remain accepted and deliver resettable RawPlcValue payloads.
  Discovery uses an unconnected per-interface UDP socket so unicast replies arrive;
  the reviewed regression substitutes a loopback destination.
- KNX float buffer encoding, Go generator floatEncoding forwarding and generated
  KnxDatapoint/KnxProperty are applied together. Raw byte-array list output remains
  one raw value rather than a nested list. BACnet reports refused COV cancellations
  while dropping the local handle; unbounded UDP writes clear a prior deadline.
- Go module locks follow the reviewed upstream tip. Local Go POM,2.0.0.B-SNAPSHOT,
  Maven3 schema, Java21/Jackson3 fork and the existing Gopacket test are preserved.
  Only the Go emitter's Java implementation changes; no Java PLC driver/API or
  shared SLMP mspec is changed in this group. No public Maven install was performed.

The candidate contains11953 exact tracked before inputs. All original inputs were
checked again before applying; all11928 inputs outside the selected existing25
were unchanged. Four selected files are new. All29 applied bytes match their
immutable validated copies and reviewed upstream files.

## Validation and limits

Go1.27.4 (existing module requires1.27), full `go build -p1 ./...`:pass.
Four affected packages:581 top-level tests pass,2128 pass events including nested
subtests,2 existing skips. The skips are the short-profile multi-second lease-gap
case and the pre-existing platform-dependent dump case. Two named KNX discovery
cases that can send real-interface multicast are explicitly excluded; the loopback
unicast-discovery fixture runs. No physical PLC/network discovery acceptance is
claimed. Before-source with identical dependency locks and two upstream test
files fails3 leaf assertions: COV reject/abort and stale UDP deadline. Those same
leaf cases pass in the candidate. These are assertion failures, not compile failures.

Affected-package `go vet -stdmethods=false`:pass. Full default `go vet ./...`:
**fails in both before and after**, with the identical31 diagnostic multiset:
27 standard-method signature/name diagnostics,2 generated unreachable-code and
2 generated CBus shift diagnostics. There are no new diagnostics. They remain an
explicit internal baseline follow-up, not a passing whole-repository vet result
and not an added external deferred group. The actual before/after logs are retained.

`go mod verify`:all modules verified. Isolated root-reactor Go-emitter package and
its prerequisites:pass with JDK21/Maven3 and explicit existing Maven cache. Tests
were skipped in that package command; it is not a generator test-suite or full
root regeneration claim. The unnecessary requested update-generated-code profile
was not present in the selected reactor, emitted a warning and was not activated.
Runtime tests exercise the selected committed generated Go outputs. The shared
SLMP group must separately regenerate and adapt its Java/Go call sites from the
current root, avoiding stale cached mspec artifacts.

Build concurrency is1, GOMAXPROCS2 and Go's memory target2GiB (a soft GC target,
not a hard RSS cap). The first2-way build was explicitly interrupted before later
steps to reduce peak memory and select the network test profile. All owned build
handles have terminated. Existing runtime, VM and frozen remote evidence are kept.

## Cross-repository relation and evidence

Current YOFC OPC UA server POM has37 PLC4J declarations at2.0.0.B-SNAPSHOT, no direct
SLMP driver declaration; this is a declaration map, not a full runtime graph. Go/C#
are not its consumed Java artifacts. Shared mspec changes still affect generated
Java APIs and require their own coordinated treatment. Cloud/container repositories
are nonforks created by splitting Kura bundles, so nonfork status does not remove
source-origin or compatibility obligations. Their detailed mapping remains pending.

Evidence root: `target/upstream-logical-sync-20261112/plc4x-review/`.
Key files:source-review-inventory.json, go-functional-candidates.json,
go-source-baseline.json, go-before-selected/, go-validated-source/,
go-validated-source-manifest.json, go-applied-to-working-tree.json,
go-test-summary.json, go-loopback-profile.json,
go-before-after-regression-comparison.json, go-vet-before-after-comparison.json,
go-generator-package.json/log, go-module-verify.json/log,
yofc-plc4x-consumers.json. Earlier interrupted/failed runs remain labeled separately.

Preservation at application:18 HEAD/branches unchanged, all96 applied source pins,
146 credential source pins,8 ca9 artifacts and frozen f07 manifest match. Original
console/PostgreSQL containers remain healthy; no management one-time acceptance,
UNKNOWN re-dispatch, runtime replacement, reset/clean, commit or push occurred.
The original3 external groups and14 management groups are retained.
