# Firewall GWT, actual Core snapshots and paused handoff — 6336f5c7

2026-11-07. This session made progress. The user requested a pause after this
batch and a knowledge/ledger handoff because the chat is long. The complete
management redesign is **not complete**. No commit, push, reset, clean, package
installation, production ownership adoption or historical one-time replay occurred.

## Session reconciliation and delivered changes

The prior 4a2cb19d manifest initially matched all 739 pins: 543 source, 45 artifact
and 151 evidence entries. All 18 repository HEADs/branches and complete dirty
statuses matched at entry. Existing changes and test/recovery data were retained.
The earlier F01–F19 and selected recovery, cancellation, restart/reboot and NAT
acceptances remain closed within their recorded scope; they were not replayed.

The accepted direction remains NetworkManager as system network backend, Kura
usable without agent, an always-running agent with a separate remote-channel
switch, and Kura-generated private key/CSR. Agent installation controls visibility
of the original GWT remote menu. Existing login/first-password backend gates,
default-off configuration/firewall grants, separate identity/ownership, UNKNOWN
non-retry and RECOVERY_REQUIRED write blocking remain unchanged.

Browser input became usable; its previous failure's underlying cause was not
proven. The final standalone GWT fixture from 4a2cb19d was launched once and used
the unchanged production web2 JAR. Chinese and English checks covered explicit
acknowledgement/confirm/rollback, UNKNOWN with disabled writes, exact-original
outcome lookup after lost prepare/confirm/rollback replies, six editors, policy
editing and a nullable manual NAT row edit. Its exact final counts were five
prepares, three confirms and two rollbacks; system writes were zero. The NAT
candidate changed only the selected row's output interface, preserving its
position, nullable fields, automatic NAT and every other category. This was a
fake-provider browser gate, not actual Linux traffic or full product acceptance.

Actual Orb Core reads then exposed a production defect: the IPv6 port-forward
destination property contained `org.eclipse.kura.net.IP6Address@...`. Core API
`FirewallConfiguration` appended the address object rather than its host address.
The serializer now calls `getHostAddress()`. No public signature changed. A new
API regression first reproduced the exact failure; a new provider regression
checks typed destination versus confirmed PID export used by Core snapshots.

Final affected tests passed: API 24, provider 13, bridge 63, Core configuration
107 and selected web2 firewall contracts 22, total 229. API/provider bundles were
rebuilt and installed in the explicit local Maven cache. The production GWT
client and server source/JAR and Rust agent source/binaries were unchanged;
their prior build evidence is reused only for those unchanged inputs. This is
not a new full workspace, Docker or formal release acceptance.

## Complete isolated IDEA and preserved failed fixtures

All full macOS Kura launches used IDEA UI, never CLI. Each had 285 inventory
entries and 284 actual framework bundles, the real firewall provider and real
Core ConfigurationService. Original admin/admin login and hidden remote menu
without a formal agent installation passed. The test-only first-password bypass
was confined to fresh isolated profiles; production behavior was not relaxed.

- Native v2 JVM 39281 failed to start the fixture observer because its manifest
  omitted `org.eclipse.kura`; the provider/web2 were active and original login
  worked. No observer write ran. Its 600 retained file pins and real framework
  startup log are preserved. A separate minimal resolver diagnostic timed out;
  it is not full-Kura validation or a passing result.
- Native v3 JVM 39837 and temporary production agent 39871 verified no-agent
  AGENT_UNAVAILABLE, then an uninitialized/default-off view and real Core
  UNINITIALIZED rejection rather than fabricated empty rules. GWT writes were
  disabled. An initial CONFLICT warning was observed and remains unexplained.
  Both processes stopped; 619 retained file pins.
- Native v4 JVM 42508 loaded the fixed API and actually serialized the full V3
  IPv6 destination as `2001:db8:2:0:0:0:0:2` in that framework; original login
  passed with no agent. This is an in-framework serializer check, not initialized
  Core export. The later two VMs supply the actual initialized Core evidence.
  JVM stopped; 293 static and 602 retained file pins.

The first Orb fixture `/opt/yofc-fw-gwt-orb-20261107-087c2845` was stopped after
the genuine IPv6 export defect was found. It retains its revision-1 baseline,
malformed Core read/snapshot, state and namespace; no new GWT write was issued.
JVM 741981, agent 741930 and proxy 742202 stopped; 633 retained file pins.
It must remain a failed attempt and must never be replayed.

## Actual Orb selected GWT/Core/traffic evidence

Fresh fixture `/opt/yofc-fw-gwt-orb-20261107-e66a226e` used JVM 743806 as UID
65534/CapEff=0 and root agent 743755 with only CAP_NET_ADMIN. The final production
ELF remained `d4a13b920f7b5d3e31848a9f23e58dc81a745e579a24a5f72b4d81928dfefe15`.
The existing Orb global systemd override was retained: Kura NNP=0, agent NNP=1;
this is not strict read-only Kura sandbox proof. Both write grants were explicitly
enabled only for the new test fixture. An explicit root test adoption created
confirmed revision 1 before Kura; it is not formal ownership adoption.

The original authenticated GWT submitted real transactions through the registered
Kura UID/UDS boundary. Core confirmed PID exports and snapshot readback contained
the numeric IPv6 destination. During a pending ICMPv6 change, actual ping changed
from denied to allowed while Core reads and a newly created snapshot remained
exactly confirmed revision 1. Explicit GWT confirmation advanced to revision 2.
Matching Core update was read-only; mismatching legacy update was rejected.
Rolling back the old revision-1 Core snapshot preserved the exact raw/semantic
kernel table and all prior snapshot files. A new snapshot still exported current
confirmed revision 2.

The first lost-reply injection did not match the server's gzip response; the UI
received a normal WAIT_CONFIRM reply and that known transaction was explicitly
rolled back. The unsuccessful injection, marker and DOM evidence remain. A
test-proxy v2 decoded gzip only for fault detection, preserving normal wire data.
Only that new fixture's original proxy was replaced; no Kura/agent was restarted.

Two independent new transactions then proved actual lost prepare and lost confirm
replies. UNKNOWN kept write/confirm/rollback disabled; withheld original outcomes
kept UNKNOWN. Restoring read-only original lookup found WAIT_CONFIRM, followed
by explicit rollback, or COMMITTED, without another confirm. Final actual GWT
counts are **4 unique prepares / 2 confirms / 2 rollbacks**, including the known
non-injected attempt. There were exactly two deliberately dropped replies and
no automatic write resubmission. Final confirmed revision is 3, IPv4 ICMP denied,
IPv6 ICMP allowed. Fresh TCP passed; other full-V3 rules and automatic NAT were
unchanged. The first TCP helper had a Path/string construction error before any
server launch; a separate corrected helper passed and stopped.

All new units/helpers, including replacement proxy 745212, are stopped. Host
rules/forwarding were unchanged. The final fixture has 691 retained file pins.

## Existing Parallels strict sandbox evidence

Reused Ubuntu VM 8061d57e-0120-42ec-a9a9-5799b436c4eb; no new VM was created.
Fresh `/opt/yofc-fw-gwt-parallels-20261107-e01b93a1` used JVM 2547 as UID65534,
CapEff=0 and root agent 2465 with CapEff=0x1000. Both had NNP=1 and actual
mount probes returned EROFS=30. All 184 private JRE file hashes matched.

Original GWT admin/admin login, a real prepare/explicit confirm, pending versus
confirmed Core export/snapshot, matching/mismatching update, old Core snapshot
non-rewind, exact previous snapshot preservation, current revision-2 snapshot,
IPv4/IPv6 ping and fresh TCP passed. Its exact GWT counts are one prepare and one
confirm; no lost-reply fault was injected on this VM. This scope must not be
inflated to repeat every Orb/browser fault scenario.

Initial page loading reproduced the unresolved CONFLICT warning before any write.
Its DOM/screenshot are retained. Dismissing the warning and loading the independent
policy baseline allowed the selected transaction to pass; it does not close the
initialization defect. Two long inline read-only guest invocations could not open
a Parallels Tools session; passing the same diagnostic as a shared script in a
new observation succeeded. No firewall command was resubmitted.

New units/helpers stopped; 846 retained file pins, unchanged host rules/forwarding.
Ubuntu shut down normally; Windows remained stopped.

## Final state and evidence limits

All this batch's new Mac JVMs, temporary agent, browser helper, fault observers,
VM units/proxies/echo helpers are stopped. Original Orb four services remain
active at PIDs **244516 / 244911 / 286620 / 268** with original console package
`0.1.0~package20261102-26` and agent `0.1.0~package20261103-20`. All namespaces,
state, keys, backups, quarantine, volumes and failed fixtures remain. Personal
`~/.kura-dev` and running target/runtime were not cleaned; 192.168.2.125 was not
contacted. All 18 HEADs/branches remain unchanged; existing dirty work is retained.

The previous 151 evidence-file pins remain byte-identical. Original rebuilt
API/provider artifacts and edited source contents were archived before changes.
Three mutable prior Surefire XML reports were regenerated by this round's tests;
exact old XML content was not retained in the searched archives. Prior manifests
and logs remain, but **do not claim all old 45 artifact pins still match or all
old XML content survives**. This retention limitation is separate from the two
historical evidence gaps. New reports are explicitly pinned as current results.

Test browser proxies use an unaffected loopback management path. Checkbox
acknowledgement exercises the UI contract only; a real new connection through an
affected management path is not proven. Full firewall product, complete Kura
business recovery, formal ownership adoption, actual early boot and release
remain unverified.

## Next internal work and external boundaries

Start by diagnosing the reproducible **initial GWT CONFLICT** from retained
native-v3/Parallels evidence and current read/lock/session code. Terminal UI also
continues to display a decreasing confirmation countdown after COMMITTED or
ROLLED_BACK; this remains a separate UI defect. Do not rerun closed one-time
transactions just to investigate reads.

Continue the original ledger: full component applicability/metadata and GWT
editing, production/external PID Java recovery and empty/partial/inconsistent CM
baseline strategies; NM automatic NAT/other-writer formal adoption/retirement and
installation profiles; encrypted center firewall tasks, bilingual remote workflow
and affected new management-path proof; confirmed-only actual early boot; remaining
operations/Core installation/upgrade/bootstrap/handoff/retirement; consistent
DB/artifact/key backup/restore, rotation/migration and formal release.

External deferred stays separate: real NM/cloud-init/physical hardware, amd64,
disk-full/hard-power validation. Keep the two historical evidence gaps separately;
Java is available. Previous cancellation/restart/reboot/selected recovery/NAT
closures stay closed within their proven scope.

Authoritative manifest: `docs/evidence/firewall-gwt-system-20261107.json`.
Private evidence: `target/firewall-gwt-system-20261107-6336f5c7`.
Parent knowledge note: `Experience/2026-11-07-Kura防火墙GWT与真实Core快照系统验证暂停-6336f5c7.md`;
read it together with the latest Tasks.md and Session Summaries.md.
