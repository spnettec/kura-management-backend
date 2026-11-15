# R013 v21 — select firewall providers before DS publication

R013 remains an unapplied candidate in the same original-bound shadow. This
checkpoint closes an actual service-publication defect;it does not establish
complete Core/NM startup or ownership of kernel rules.

All stage evidence is under `target/nm-role-authority-20261112/`.

## Actual before counterexample

`writer-profile-legacy-scr-v21/actual-scr-legacy-satisfied-before-v1` uses the
actual legacy IPv4/IPv6 bundle,actual SCR and v20's private Java cohort. Unlike
the older unsatisfied-dependency proof,it supplies both legacy EventAdmin and a
deny-only PrivilegedExecutorService. The unrelated NM status component is
disabled first;all other application bundles remain above the framework start
level. Its fresh owned configuration and fences never touch the original runtime.

Both legacy components reach state16/FAILED_ACTIVATION because the existing
ownership guard rejects them. SCR nevertheless retains their advertised firewall
service references. Thus an activation exception does not prevent competing
service publication. The before probe fails on these actual references. The
deny-only proxy can execute no command. This evidence replaces the earlier
assumption that installed-but-inhibited legacy providers cannot compete.

## Implementation

A local immediate DS component in linux.net publishes the selected firewall
Condition. Normal missing/legacy provider selects legacy. Agent provider selects
agent without contacting or requiring an agent process. A retained durable fence
with a missing/legacy provider selects neither;invalid provider fails closed.
The condition disappears when its local component deactivates.

Both legacy and both agent descriptors explicitly require their corresponding
`osgi.ds.satisfying.condition` target. Selection therefore happens before their
services can publish. Explicit references also avoid relying on older SCR's
implicit-condition fallback. The relevant bundles import the actual Condition
API;the new selector stays internal. The current supported/tested Java runtime
is21. This follows the [OSGi satisfying-condition contract](https://docs.osgi.org/specification/osgi.cmpn/8.0.0/service.component.html#service.component-satisfying.condition).

The condition is selection only. It grants no write or adoption authority and
does not claim/remove rules. Existing durable network/firewall fences,writer
drain,actual local NM observer validation,fixed PIDs and per-call guards remain.
Agent transport remains optional0..1. No operator environment variable or agent
startup dependency is added. Internal startup settings still belong to the
existing reviewed/generated GUI flow.

## Current actual evidence

- `writer-profile-selection-java-v2`:eight-module offline package2117 passes,
  zero failures/errors/skips:API25,crypto30,Core265,linux.net154,NM1508,bridge103,
  agent firewall18,legacy firewall14. The six new local role/fence tests pass.
  Eight private JARs and270 raw report hashes are archived. Shared API is unchanged
  and was not installed.
- `actual-scr-legacy-satisfied-after-v2`:same fully satisfied legacy dependencies
  and actual published PID properties,now using the fresh eight-module cohort.
  Legacy services remain unsatisfied before publication;new facades activate
  only with the mandatory local observer. The existing seven lifecycle branches
  pass with these changed dependencies,including lost fence,foreign facade,
  duplicate observer,deactivation/replacement and cached-old-object refusal.
  Two synthetic reads reach the isolated transport;accepted/attempted backend
  commands are zero. All probe frameworks stop.
- `actual-role-selection-matrix-v1`:three fresh actual SCR cases verify missing
  provider/no fences selects legacy;retained fences/missing provider selects
  neither;invalid provider selects neither. Actual descriptor targets and local
  selector origin are checked. Backend dependencies are deliberately absent in
  this matrix. It proves role selection without an agent,not standalone kernel
  activation or complete Core startup.
- `packaged-ds-contract-v21`:five actual packaged selector/provider descriptors
  and Condition imports pass;the agent's optional transport and mandatory local
  writer reference remain. Full policy adoption is not inferred from this.
- `native-source-reuse-v21`:host native38/strict38 pins and all138 agent inputs
  from the Linux v20 manifest remain exact. Precisely reuse native463/0/16,
  strict Clippy and Linux44 startup/recovery contracts. No eight-minute native
  rerun,new Linux build,normal-agent start or historical one-time acceptance
  replay is needed. The unchanged normal ELF remains
  f97e1b737d30606a98553eef64e7f60aabe5282d7d5e109bc5d8c7378b13df87.

The first Java run exposed only an ambiguous Mockito overload in the new test;
its source archive/log and preimage remain. The corrected typed matcher does not
change production behavior or weaken any assertion. Before and after service
proofs,all fixture failures and their original source pins remain.

## Authority and remaining work

`candidate-source-pins-writer-selection-v21.json` records59 existing+34 new
paths;excluding the same three build fixtures leaves90 production/test changes.
Only ten source paths differ from v20. The legacy source baseline adds15 protected
originals,bringing the total to1346. The full archive has1380 source files plus
three never-to-apply reactors. All original inputs,20 applied management pins,
370 current upstream pins and18 Git identities/refs remain exact. Original
console/PostgreSQL IDs/health/start times,gateway and two stopped Parallels VMs
are preserved. No R013 source is applied,committed,pushed or deployed.

The NM observer in this probe is still an actual NM implementation manually
registered by the actual NM bundle. Complete participating NM/Core activation is
not claimed. Core currently witnesses only the network PID. Continue all three
external handlers,original firewall policy/automatic-NAT ownership and explicit
foreign-rule treatment,then joint confirmed generations,ordinary/modem semantics,
retirement/re-adoption and GUI/current live validation before guarded application.
Keep R017,the original14 groups and all seven API2 migrations. Legacy download/REST
global limits remain internal pending. External deferred stay the original three.
UNKNOWN is not resent;no closed one-time acceptance is replayed.
