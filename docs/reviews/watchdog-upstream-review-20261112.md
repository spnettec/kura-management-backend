# Watchdog and GWT upstream adaptation, 2026-11-12

Status:35 selected source/config/test files applied after exact before checks.
Source integrated, not deployed. Original worktrees, runtimes and closed evidence
are retained. The parent management goal and its14 groups are not closed.

## Functional input and resulting behavior

Core upstream `f7717a30cb7b4ebb297ce097a0ab05df5afb8a3a` and UI upstream
`4c9b9b8ae60fc3c3e9ebace352447eccf361df22` were reviewed as one functional group.
Hardware timeout is read in seconds and exposed in milliseconds; effective polling
is capped at half a known timeout. Device-unavailable grace/recovery, requested
reboot and failed-reboot retry states become observable. The unavailable-device
reboot policy is exposed in the existing XML configuration UI and remains false
by default, as does watchdog enablement. Existing property IDs and default paths
are preserved; no operator environment variable was introduced.

The immutable API snapshot includes state, device/path, last successful refresh,
effective interval and failure reason. The new status method has a default
unsupported implementation so old providers retain their source/binary contract.
The additive package version is watchdog2.1.0. Linux/emulator imports require2.1;
GWT imports it optionally. The deprecated channel/driver/comm/message API deletion
cohort remains a separate internal migration, rather than being folded into this
additive change.

GWT isolates optional types in a helper, keeps existing device/network/cloud
summaries, localizes8 new keys in English/Chinese and escapes device paths and
state text. Missing API/provider keeps the page usable; a legacy/null status is
shown as unavailable, rather than claiming disabled or supervising. Status reads
only call the snapshot getter. The emulator reports no physical supervision and
its compiled class no longer contains a physical watchdog writer.

## Local implementation review

- Preserve Maven3/MBP/manual SCR/XML, Java21 core and Java11 GWT output. Use shared
  provided JNA5.19.1 with explicit package imports; do not embed/downgrade JNA5.17.
  Current core distribution already copies shared JNA into plugins/6s.
- Snapshot option maps and reject nonpositive refresh intervals before device
  open; expose the positive minimum in XML. Existing configuration values remain
  valid. Preserve the actual device path and both disabled defaults.
- Validate native descriptor/ioctl/close results and require exactly one written
  byte for keepalive/disarm. Map Linux ssize_t to NativeLong. Failed close is
  reported without retrying a descriptor that might have been reused.
- Serialize shutdown admission against updates/scheduling, discard queued work,
  await executor exit before final close, and roll back an open completed during
  shutdown. Stop cannot keep reporting active supervision. Post-stop updates get
  a lifecycle error and the same instance can reactivate with unchanged config.
  Preserve thread interruption and stop a cancelled sync/reboot sequence before
  issuing another command. An uninterruptible kernel operation remains a real
  hardware boundary; the software tests do not prove such a device recovers.
- Unit fixtures inherit fail-fast native/service-stop/reboot/command boundaries.
  The marker is under the owned module target directory, never the real
  /tmp/watchdog. A reviewed registration-only legacy test never activates its
  provider. Native-wrapper contracts use a recording proxy. No unit test invokes
  an operating-system reboot, daemon stop or real watchdog node.

The Linux watchdog documentation describes keepalive, magic close and GETTIMEOUT;
write(2) specifies the actual byte count returned. These are the native wrapper's
contracts, not evidence of a physical watchdog test.
[Kernel watchdog API](https://www.kernel.org/doc/html/latest/watchdog/watchdog-api.html),
[Linux write(2)](https://man7.org/linux/man-pages/man2/write.2.html).

## Verification and real dependency closure

Final four-module run:154 passed,0 failures/errors/skips: API25, Linux watchdog113,
emulator9, GWT7. Both GWT modules compiled6 permutations and all bundles packaged.
No JUnit4/Vintage or compiler/test libraries are embedded. The original native
wrapper fails4/5 recording contracts; the repaired one passes. Three deterministic
shutdown counterexamples fail before the lifecycle fix; the final5 shutdown cases
include in-flight open rollback and same-config reactivation. Four command-recorder
cases verify cancellation through the real reboot implementation without dispatch.

A Java11 old-provider class was compiled against the exact before API/watchdog2.0
artifact. Fresh Equinox init/resolve/load probes, without application activation,
verify that binary against2.1, its unchanged timeout behavior, unavailable status,
absent-provider behavior and actual JNA5.19.1 wiring. A second probe uses the old
API2.0 with the new UI: the optional import is unwired, RPC reflection works and
the watchdog section falls back cleanly. Both report0 active application bundles
and terminate.

The old runtime cut lacked the current GWT configuration receipt capability.
Its real provider then required the snapshot codec, while that codec requires
configuration password-source capability. The probe uses the exact unchanged
captured configuration and XML codec sources, packaged in isolation, to resolve
that real pair. No synthetic capability was substituted. These prerequisite
builds skipped tests and are not counted in154. They neither change original
provider sources nor replace original target/runtime artifacts. The final6 JAR
pins distinguish this closure from the earlier four-bundle snapshot.

## Linux ARM64 native ABI evidence and limits

The existing Orb gateway has no Java; no package was installed there. Fresh owned
containers use the cached Linux ARM64 Temurin21 JRE, network none, UID65534,
read-only root/inputs, dropped capabilities, no added devices and bounded memory.
Only newly created ordinary files are opened; no /dev/watchdog node is exposed.
Native open/write/ENOTTY ioctl/close are real Linux calls. The returned bytes are
w,V and ioctl errno25; idempotent close is verified. These are file/ABI contracts,
not watchdog supervision, magic-close or reboot acceptance.

`linux-libc-abi-v3` passes with a read-only native library extracted from the pinned
JNA JAR. `linux-osgi-native-default` passes through the actual bundle classloaders
with default JNA bootstrap properties, no manual JNA path and0 active application
bundles. The ordinary-file bytes and real ioctl failure are verified again.

Earlier negative fixtures are retained: the first standalone classpath lacked the
KuraException logging API; adding it exposed noexec temporary-directory loading
failure. A noexec/nounpack OSGi diagnostic also fails, rather than proving automatic
Bundle-NativeCode bootstrap. JNA5.19.1 defaults to skipping system library search
and can extract from classpath; that requires an executable native-library location.
The actual successful default-profile probe uses executable owned temporary/cache
mounts. The noexec diagnostic is not reported as a passing sandbox test, and no
existing service or global configuration was weakened. A stricter release sandbox
must validate its automatic native-library arrangement as part of the existing
internal release gate, without requiring operator flags.
[JNA5.19.1 loading implementation](https://raw.githubusercontent.com/java-native-access/jna/5.19.1/src/com/sun/jna/Native.java).

Actual Linux service/hardware supervision, real reboot/nowayout behavior, amd64 and
full release/runtime/UI integration are not proved by these probes. Keep physical
hardware/amd64/power-loss limits in the original three external groups; release
and integration engineering remain internal pending.

## Evidence and preservation

Root: `target/upstream-logical-sync-20261112/watchdog-observation/`.

- `source-baseline.json`:2750 exact before inputs across core/UI, including dirty overlays.
- `validated-source-manifest.json`, `validated-source/`, `applied-to-working-tree.json`:
  the35 selected inputs, before/after hashes and actual application.
- `final-four-module-package-v3.json`, `validated-test-summary.json`:154 contracts
  and final package/GWT results. Earlier failures and intermediate passing runs
  remain separately labeled.
- `native-contract-before.json`, `native-before-reports/`,
  `shutdown-contract-before.json`, `shutdown-before-reports/`: counterexamples.
- `legacy-provider-compile.json`, `osgi-new-v5-result.json`,
  `osgi-old-v4-result.json`, `osgi-runtime-inputs.json`: real classloader evidence.
- `receipt-codec-provider-package.json`, `validated-bundle-pins-final.json`:
  the two unchanged real prerequisites and final6 artifact hashes.
- `linux-libc-abi*/`, `linux-osgi-native*/`: source/input hashes, exact Docker
  commands, image IDs, logs, terminal states and failed/successful profile limits.
- `all-baseline-inputs-audit.json`: no unexpected original/candidate source changes.

All new containers are stopped and retained. No current deployment, commit, push,
reset/clean or closed one-time remote acceptance was performed. The four upstream
functional groups now have67 active source pins; earlier evidence remains scoped
to its own frozen revisions.

Preservation audit `target/upstream-logical-sync-20261112/four-groups-preservation-audit.json` verifies all18 HEAD/branches,67 applied source pins,146 credential source pins,8 ca9 artifacts and the frozen f07 manifest. Full current worktree fingerprints for18 repositories were captured (not retrospectively compared with nonexistent initial diff hashes). The original console/PostgreSQL containers and Orb gateway remain running; both Parallels VMs and all5 new owned native-probe containers are stopped.
