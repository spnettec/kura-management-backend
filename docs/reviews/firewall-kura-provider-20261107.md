# Kura firewall provider and actual public API validation — a44da9fc

2026-11-07. Progress; the full management redesign remains active. This follows
5e0d8a41 and preserves its source/evidence baseline and every worktree.

## Delivered scope

The optional `org.eclipse.kura.firewall.agent` provider supplies both original
firewall service PIDs, confirmed property projections and the new Core public
`org.eclipse.kura.net.firewall.transaction` API. One typed snapshot reads both
families at the same revision. It requires the explicit agent firewall profile,
both external PID reservations and absence of the old firewall writer bundle.
The agent reference is optional/dynamic; Kura can start and log in before agent.
NM remains the system network backend. The default distribution is unchanged.

The agent adds a registered, exact-Kura-UID `firewall.kura.*` boundary and a
separate default-off `AGENT_KURA_FIREWALL_WRITE` grant, in addition to the existing
global write grant. Prepare temporarily applies under one shared system lock;
confirm/rollback remain explicit. UID/owner/canonical typed-input digests bind
the original request. Settled Kura journals are archived before replacement.
View/outcome observations work while other recovery journals block writes and
do not invoke nft, replay, repair or clear the write guard. Lost/malformed Java
write replies remain UNKNOWN. Missing outcome evidence remains OUTCOME_UNKNOWN.

The provider closes legacy setters/raw flooding and snapshot rewind. Old
confirmed records without a matching typed V3 candidate remain unprojectable;
there is no automatic empty/default-rule adoption. The Java V3 mapper now accepts
omitted nullable NAT protocol/ruleType fields as null, matching Rust. Unknown
fields and the original family/range/automatic-rule constraints remain strict.

Contract: [firewall-kura-provider-contract-20261107.md](../firewall-kura-provider-contract-20261107.md).

## Validation and precise limits

- Java: 63 bridge tests plus 12 new provider tests. These cover profile gates,
  optional agent, integer precision, coherent typed views, IPv6 property names,
  unsupported/uninitialized data, snapshot non-rewind, legacy denial, malformed
  and lost responses, original outcome identity and nullable full-rule mapping.
- Rust: full host suite 216 passed / 10 ignored; the final affected host and
  Linux runs each passed 28 / ignored 1, including the subsequently added exact
  registered UID/default-off role test. Final fmt, strict Clippy and both release
  builds passed. The final full suite was not rerun after that isolated test addition.
- IDEA UI launched fresh complete isolated Kura profiles. Final profile has 285
  inventory entries / 284 actual framework bundles and 289 static pins. JVM
  32184 started with no agent, registered both real provider services and returned
  AGENT_UNAVAILABLE; original admin/admin GWT login passed. Agent 32196 later
  connected over a short local UDS: both write grants remained off; prepare was
  FORBIDDEN, missing outcome stayed OUTCOME_UNKNOWN, and the old setter failed.
  Both processes are stopped. The test-only first-password bypass is confined to
  these isolated fixtures; production password behavior is unchanged.
- Orb actual Kura UID 65534 / root agent, JVM 740534 / agent 740528, executed the
  public API prepare/apply, explicit confirm, pending/confirmed separation,
  original outcome, wrong-owner denial, legacy denial, explicit rollback and
  historical outcome lookup. A deliberately duplicated known-WAIT_CONFIRM
  prepare was rejected before another apply; it was a negative test, not an
  automatic retry after UNKNOWN. A fresh TCP flow passed. Final revision is 1,
  transaction ROLLED_BACK. The script's subsequent Java NNP=1 assertion encountered
  the retained Orb global override; Java actually has CapEff=0/NNP=0 and agent
  CapEff=0x1000/NNP=1. No global override was changed.
- A new Orb read-only normal reopen used a consistent copy of that stopped agent
  state and the retained namespace, with both write grants off. JVM 740915 /
  agent 740908 read the confirmed typed/original API values and settled journal;
  no firewall write was issued. Another fresh flow and original GWT login passed.
  Both units and helper processes are stopped; state and namespaces remain.
- Existing Parallels Ubuntu was reused. Final `/opt/yofc-fw-provider-parallels-20261107-4369e64e`
  JVM 2692 / agent 2686 passed 13 actual public API/flow checks and original GWT
  login. Kura ran as UID 65534 with CapEff=0; agent as root with only 0x1000.
  Both had NNP=1 and actual read-only mount probes returned EROFS=30. Its private
  JRE's 184 file hashes matched. Final revision is 1 / ROLLED_BACK. Both units and
  helpers stopped, then Ubuntu shut down normally. Windows remains stopped.

Browser access used an explicit test-only loopback proxy into each namespace.
It proves original login, not a new affected management-path connection. No
GWT transaction editor, center task, production ownership adoption, actual
early-boot installer, new deb/Docker release or full firewall product is claimed.
The external configuration handler's snapshot/update policy is unit-tested;
actual Core ConfigurationService rollback and GWT transaction editing remain
part of the next integration scope.

## Preserved attempts and environment state

All failed attempts remain. Native initial fixture exceeded macOS UDS SUN_LEN;
the v2 staging preflight then encountered a recently closed port before creating
a fixture; v3 exposed the nullable Java contract mismatch before agent mutation.
Fresh v4 passed. Initial Orb had a fixture JVM command missing `-jar`, before any
firewall request; Orb v2 completed business/flow checks before the known NNP
assertion. Its completed writes were not replayed by the read-only v3 reopen.
Initial Parallels could not execute the ordinary Kura UID through root's umask-077
fixture directories; no Kura firewall request was issued. A new fixture explicitly
granted traversal to its copied runtime/JRE, retained root-private agent state,
and passed. Old fixtures, keys, state, archives, failed unit states and volumes
were not removed or reset.

Retained file pins: final native 639; Orb first/v2/v3 309/670/642; Parallels
initial/final 492/859. Additional failed native fixtures and static runtimes are
also pinned. Before building, all 19 previous mutable/artifact contents were
archived privately. Prior evidence files remain unchanged.

Original Orb console/center2/agent/phase2 services remain active with PIDs
244516/244911/286620/268 and package versions 26/20. All six new Orb units have
MainPID=0, including retained failed states. All new native processes are stopped;
both Parallels VMs are stopped. No new center/PostgreSQL/Vite/keepawake process
was started. Host firewall and forwarding values were preserved.

## Continuing work

Internal pending: original GWT authenticated session ownership, per-panel frozen
draft baseline, explicit confirm/rollback and actual lost-response reconciliation;
actual original CM handler integration; NM automatic NAT/other-writer ownership
adoption and retirement; center encrypted tasks and bilingual remote workflow;
new affected management-path proof; confirmed-only actual early boot; formal
installation/Core upgrade, full component/Java recovery surface, consistent
database/artifact/key backup/restore/rotation/migration and release.

External deferred remains separate: real NM/cloud-init/physical hardware, amd64,
disk-full and hard-power validation, plus the two retained historical evidence
gaps. Java is available; this batch's selected Kura/Java checks are not deferred.

Authoritative manifest: [firewall-kura-provider-20261107.json](../evidence/firewall-kura-provider-20261107.json).
Private logs/fixtures: `target/firewall-kura-provider-20261107`. All 18 HEADs,
branches and prior dirty entries are preserved. No commit, push, reset or clean.
