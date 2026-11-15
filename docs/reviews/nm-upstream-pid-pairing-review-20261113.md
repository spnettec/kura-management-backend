# Core/GWT logical upstream integration and paired R013 review — v8/v27

2026-11-13. The independent upstream fixes are source-applied to eight original
paths,with five preimages and a per-file verified journal. No commit,push,
deployment,service restart,shared API installation or management-candidate
application occurred. All18 repositories outside these paths,including retained
untracked source and fixture metadata,are preserved by the application receipt.

Core4e05e78afdc1dd9b39dd4bd0e357b8ddacb65385 handles actual SCR DS1.3 multi-valued
service.pid. YOFC previously chose an arbitrary last List value. The adaptation
deduplicates scalar/collection/array values,excludes service.factoryPid,and then
uses a unique actual CM configuration match. Ambiguity,missing CM or list failure
never invents a PID. Removal releases the logical kura.service.pid without
looking up an already-deleted CM configuration. Existing self-configuring fallback
and fork external/stored/conditional/journal/snapshot logic stay intact. Eleven
upstream cases are adapted to current JUnit5;one existing test that required the
old arbitrary last-List behavior now requires rejection. Before-fix5failures and
the old-test1failure are retained as evidence,not hidden.

GWT949546862b0aaa89e9ff89de83d85600d3ee65cd restores stored DHCP range display when
Router Mode is off. Null NetworkConfigurationService/configuration properties
produce SERVICE_NOT_ENABLED. The existing status consumer catches GwtKuraException
and retains its controlled empty-network-section behavior. Java11 casts replace
upstream pattern binding;current bundles layout/JUnit5 and fork netplan branch
are retained. No POM,API,Crypto or dependency source change is applied.

Actual-original API/Crypto/Core/Web2 private reactor passes337 tests:25/28/270/14,
zero failures/errors/skips. Both GWT login and denali modules compile;the three
changed UI class files have bytecode55. Current API/Crypto source is a build
prerequisite only. The shared API remains52205637a2742e99d052bff835ca68b41d83b774bbcb7d76403e2c35316114ed.
Retained setup failures explain the initially stale shared API and mixed original
API/paired Crypto. Successor archives use actual-original dependencies together.

Paired R013 adaptation changes only uniquely matched PID helper/registration
regions and one existing test,plus the new test;other fork changes remain.
Its current eight-module Java build passes2163 tests:API25,Crypto30,Core294,
linux.net154,NM1508,bridge120,agent18,legacy14. All1151 inputs,eight JARs and280
raw reports are pinned. This replaces the previous2152 Java evidence for current
source. Frozen source is1389 files+3 fixture reactors,three source deltas fromv26.
The complete management candidate remains101 production/test paths UNAPPLIED:
62existing+42new-3fixture edits. Never copy fixture POM/Cargo/reactor entries.

Two fresh actual Equinox/SCR/CM scenes exercise four DS factory/singleton/deletion
cases each,with exact upstream DS1.3 descriptors and real SCR merged ArrayList
service.pid values. Original mode performs three accepted Core updates and
verifies actual snapshot inclusion/removal. Paired mode uses the actual selected
external-owner profile and Core/NM/bridge/firewall DS with an absent owned socket.
Three complete-state Core updates correctly refuse offline before any CM write;
three direct CM updates are fixture setup for actual DS callback/Core-read
lifecycle checks. They are not successful remote/Core writes. Reserved CM counts,
values and contradictory retained snapshot remain unchanged;external reads refuse
CM fallback. NetworkService/Executor are deny-only;backend calls are zero. No
manual PID/NM service registration,normal agent,real NM DBus,production launcher
or physical-gateway acceptance is claimed. Both frameworks stop. Retained helper
OCDService and offline-update expectation mistakes are explicitly classified.

All153 native inputs are byte-exact to sealedv26. Reuse492/0/17,strict Clippy,
Linux normal/test builds/five contracts and the two actual production-reader
kernel observations exactly;do not replay one-timev18/v15/v24 acceptance. The
prior Core/NM restart scene remains historical since Core changed;the new paired
startup/lifecycle scene is current. Oldv24/v25/v26/C#v6 bindings remain immutable.

Active original management baseline is the explicit successor1347 inputs:two
existing Core hashes replaced and one new test added. Five old1346 baseline
files remain immutable historical evidence;do not run old-baseline verifiers
against intentionally advanced original Core. Management20pins do not overlap;
upstream authority is380pins/13integrated groups/66explicit dispositions.
Core/UI reviewed tips advance only to the two reviewed commits. Stage recheck has
10forks unchanged;PLC4X global accepted tip stayse591d9f8 because the prior7Java
groups remain pending. C#671 is already integrated and must not be repeated.
Seven nonfork direct merges stay skipped,with actual327Maven+1Cargo and
source/assembly/runtime consumer relationships retained.

Evidence:target/upstream-logical-sync-20261112/kura-core-ui-adaptation-v8/ and
target/nm-role-authority-20261112/upstream-core-pid-v27/. The latter final binding
is the current management source authority. Continue complete original-bound
legacy policies/ICMP/additional rules+automatic-NAT/role safe conversion and
transfer,ordinary/modem,joint generations,retirement/re-adoption,GUI,production
Linux paired/live validation and complete guarded application. Full14original
groups/sevenAPI2/R017 remain;SchemeA external CA remains. External deferred stay
exactly real NM/cloud-init/physical hardware,amd64,disk-full/hard-power. UNKNOWN
never grants physical redispatch/re-export. Goal active;weekly70%,ordinary allowed,
credits54476.0999360000 unchanged,no reset;pause only actual exhaustion.
