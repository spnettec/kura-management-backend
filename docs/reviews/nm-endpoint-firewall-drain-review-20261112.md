# R013 endpoint admission and legacy firewall drain — 2026-11-12

Progress in the same unapplied candidate, not complete system adoption or release.
Continue `target/nm-role-authority-20261112/workspace/`; preserve the original
worktrees, prior source seals, full source checkpoints and runtime evidence.

## Required endpoint admission

The original preparation previously accepted a CM-managed interface absent from
the adopted allowlist, or a CM modem role borrowing an Ethernet device's name.
Both actual counterexamples fail against the previous implementation in
`endpoint-controller-before-v1`. They pass in `endpoint-controller-v1`: the
whole preparation refuses before exporting ciphertext, publishing a transfer
intent/fence, creating a checkpoint or changing NM/nft.

Current admission and pre-activation checks bind every explicitly managed CM
interface, including disabled and L2-only roles, to the actual observed managed
NM device and its DeviceType. Unmanaged/unknown intent does not acquire a role
from its NAT flag. Existing CM types use the retained Java NMDeviceType mapping,
not Java enum ordinals. Automatic NAT derives the entire required WAN×eligible
LAN matrix from those verified endpoints and must equal the observed original
automatic policy. A familiar nft table name/shape is not ownership: the original
committed journal/receipt must prove this agent's policy, and NAT requires actual
IPv4 forwarding. No rule is created, deleted, adopted or defaulted by these reads.

Exact backend names work for Ethernet, Wi-Fi, VLAN and modem role projections.
An unresolved logical modem ID still refuses; explicit mapping to an actual
adopted backend remains internal work. These are fixture observations, not
physical Wi-Fi/modem/VLAN acceptance. The current candidate refuses existing
legacy NAT that differs from the confirmed agent policy. A reviewed, original
once-only migration must be added to the guided adoption workflow; do not relax
this check or claim/delete legacy/foreign rules merely to get adoption through.

## Actual legacy firewall drain

The existing FirewallOwnership suppression now has one fair in-JVM read/write
barrier shared by IPv4/IPv6. Leases span26 actual legacy entry points in
IptablesConfig and AbstractLinuxFirewall, including blocking executor calls,
backup/restore cleanup, configuration-file changes and forwarding updates.
Nested work remains bounded by the outer lease; queued work rechecks the fence.
The read-only observer waits up to1s for participating writers and never takes
their component monitor. A provider selection alone still suppresses new writes
but cannot attest a durable system/profile fence.

NM's actual NetworkWriterStateService now requires both network and firewall
quiescence. Existing bridge fence/owner reads consume this same observer, before
and after their paired configuration read. No caller readiness boolean or new
manual operator environment flag was added. The iptables export is1.4 and NM
imports[1.4,2); retained firewall import[1.3,2) accepts the additive change.

The tests block an actual mocked executor call, publish a relocated private
fence, verify that quiescence refuses while that command is still running and
that new dispatches refuse, then release the command and observe quiescence.
The actual NM observer refuses a network-only fence and accepts both private
fences. No real kernel command, system marker or sysctl change is performed.

This proves the participating Java writer barrier. It does not prove the full
effective startup profile, init scripts, foreign writers or migration of retained
kernel rules. Generated startup selection/reservations and both durable fences
must be bound to original receipts and an actual controlled restart. That full
closure remains required before source application.

## Validation and next gate

`endpoint-nm-v1`:140 Rust NM cases pass, one existing ignore. Java's
`writer-java-final-v2` packages six private modules: API25,crypto30,Core265,
bridge98,NM1508,linux.net148 =2074 passes, zero failures/skips.
`endpoint-native-full-v1`:390 pass/0fail/15 existing ignores at two threads.
`endpoint-clippy-v1`:all targets pass with -D warnings. All jobs are terminal;
earlier mirror-v5 evidence remains immutable.

Current source authority is `candidate-source-pins-writer-v6.json`:39 existing
and20 new files, excluding three fixture overrides =56 production/test changes.
The complete1340 source inputs and both fixture-only reactor files are archived
in `artifacts/confirmed-writer-source-v6/`; source checkpoint and six-module
classpath/jars/reports are sealed. `writer-preservation-v6.json` verifies all1320
original inputs,18 original HEAD/branch/index/tracked states,20 management and343
upstream pins,141 credential inputs+five prior overlays,ca9 eight artifacts and
f07. Original runtime is retained. This source generation remains unapplied.

Next continue the existing controlled/ordinary profile machinery for guided
startup/provider selection/PID reservation/restart and original migration
receipts; do not add a disconnected prototype or require operators to write
environment variables. Then complete confirmed typed generations, retirement/
archive/re-adoption, legacy UNKNOWN reconciliation, ordinary network fields,
capabilities and GUI, current Linux/OSGi/live gates, R017/full14/seven API2.
External CA Scheme A and exactly the three existing external deferred remain.

`stage-tip-recheck-r013-endpoint-writer-v6` again found all11 fork advertised tips
unchanged and preserved all18 Git states. Keep logical functional integration,
prerequisite/consumer analysis and the full18-repository gates at coherent R013
and release freeze; nonforks remain consumers rather than direct merge targets.
