# Original GWT firewall transactions — 056e9101

2026-11-07. Implementation and affected contracts/build passed. Browser
interaction and the subsequent IDEA → OrbStack → existing Parallels gates remain
**internal pending**. The full management redesign remains active.

## Implemented behavior

The original firewall page now supports the optional agent provider through six
typed rule editors, a seventh default-policy tab and explicit transaction
controls. The original legacy profile remains available without agent. Existing
NETWORK_ADMIN authentication, the backend password gate, menu availability and
XSRF checks remain authoritative. English and Chinese messages are provided.

Each panel receives an opaque edit ID with its own frozen server baseline.
Preparing one category changes only that category/family and consumes older
drafts before the single service call. Original rule references preserve priority
through modal replacement; automatic NAT, nullable protocols and original manual
NAT rule types survive unrelated edits. Duplicate or foreign references fail
before the provider call.

The authenticated HTTP session holds the owner secret, original request/change,
token and bounded outcome records. State is additionally keyed by authenticated
principal, preventing a replacement principal from inheriting another user's
transaction in a reused session. Browser DTOs contain typed rule fields and safe
state projections; no owner secret, transaction token or provider candidate map
is serialized to the browser.

Prepare temporarily applies rules. Confirmation requires explicit acknowledgement
of reconnecting through the affected management path; rollback is separate.
Lost/malformed replies retain the original intent and block new writes. Polling
and explicit query only read the original outcome. They never resend a prepare,
confirm or rollback. Missing or unrelated evidence remains UNKNOWN. The existing
RECOVERY_REQUIRED write guard is unchanged. Agent-profile legacy void setters
remain rejected. Firewall writes remain disabled by default.

Contract: [firewall-gwt-transactions-v1.md](../firewall-gwt-transactions-v1.md).
The provider/UID/journal contract remains the preceding a44da9fc contract.

## Completed verification and limits

- Final affected run: **38 passed, zero failures/errors/skips** — frozen session
  14, authenticated RPC facade 7, actual production GWT serialization policy 1,
  existing network-draft contract 16. Other older XML reports are retained and
  are not counted as rerun tests.
- Full production GWT compilation/install passed for login and denali, including
  six permutations each. A second full build includes the final client changes.
  A subsequent incremental package/install includes the final server principal
  binding; its client sources were unchanged from that second full build.
- The actual compiled firewall widget rendered both families and policy controls
  in a standalone test fixture. It uses the real RPC/session implementation with
  a fake transaction service; it is neither full Kura nor kernel validation.
  The fixture predates the last client status-message refinement. No successful
  user interaction or browser transaction is claimed.
- Browser input failed beyond GWT: an independent static HTML button also
  remained unchanged after a click. The initial failed fixture page had a native
  500-error alert; its dialog lookup returned no handle and closing that tab
  timed out. Whether that alert causes the general input failure is unproven.
  Manual handling of the old test page has been requested. This is a desktop
  validation blocker, not evidence of a product defect or external hardware
  deferral. No Codex application controls were used to bypass the tool boundary.
- Two fresh complete IDEA configurations were prepared with 285 inventory
  entries. The first is superseded; only **Kura Firewall GWT v2 20261107**, port
  18519, is the intended next launch. Neither was launched. Test-only password
  bypass and firewall-menu flags are confined to these isolated fixtures;
  production defaults and first-password enforcement are unchanged.
- No new full Kura runtime, agent daemon, namespace, VM service or firewall write
  was launched for this round. Actual Core ConfigurationService read/update/
  snapshot/rollback and GWT transactions remain unverified for this version.
  Earlier a44da9fc provider/API/system results remain scoped to that version.

Initial test/build/fixture failures remain preserved: the test classpath lacked
Guava, a serializer test used an unsupported immutable List implementation, and
the first browser server lacked runtime logging/module dependencies. The test
dependency and test construction were corrected; production serialization was
not relaxed. A later browser bind race and a non-listening helper remain logged.

## Environment and preserved state

All five new helper processes (35610, 35933, 35975, 37341, 37391) are stopped.
Prepared IDEA profiles and all failed fixture files remain on disk. No new
center, PostgreSQL, Vite or keepawake process was started for this round.

After the reported host sleep, OrbStack commands respond again. Its original
four services retain PIDs 244516 / 244911 / 286620 / 268 and the original
console26 / agent20 packages. They were not restarted or upgraded. Ubuntu and
Windows Parallels VMs remain stopped. Old state, keys, backups, quarantine,
namespaces, volumes and one-time entries are preserved. The previous web2 JAR
was archived by content before replacement. All 18 HEADs/branches and prior
dirty paths remain; no commit, push, reset, clean or remote change was made.

## Required continuation

1. Resolve the browser input blocker and compile the standalone widget from the
   final client sources before browser transactions. Use a new fixture/run marker;
   preserve the failed pages and logs. Verify explicit confirm/rollback and lost
   replies using original-outcome reads, without automatic write repetition.
2. Launch only the prepared v2 complete Kura configuration through IDEA UI;
   verify original admin/admin login, no-agent startup and default-off behavior.
3. Proceed to a fresh Orb fixture, then the existing Parallels VM. Verify actual
   GWT writes and Core ConfigurationService PID projections/snapshot/rollback,
   pending versus confirmed values, lost-reply reconciliation and actual flows.
   A loopback proxy is not proof of a changed management connection.

Other internal pending work remains: NM automatic NAT and other-writer formal
adoption/retirement/install profiles; center encrypted firewall tasks and bilingual
remote workflow/new affected management path; confirmed-only actual early boot;
full component/Java recovery scope and empty/partial/inconsistent CM policies;
remaining operations/Core install/upgrade/bootstrap; consistent database,
artifact and key backup/restore/rotation/migration and formal release.

External deferred remains separate: real NM/cloud-init/physical hardware,
amd64, disk-full/hard-power checks. The two historical evidence gaps stay
separately recorded. Java is available. Existing closed recovery, cancellation,
restart/reboot and NAT acceptance entries must not be replayed.

Manifest: [firewall-gwt-transactions-20261107.json](../evidence/firewall-gwt-transactions-20261107.json).
Private evidence: `target/firewall-gwt-transactions-20261107`.
