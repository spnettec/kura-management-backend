# Final GWT widget and fixture request binding — 4a2cb19d

2026-11-07. Progress following 056e9101; the overall goal remains active.
Production source and artifacts are unchanged. Interactive browser, IDEA and
subsequent actual-system gates remain internal pending.

The preceding manifest and all 816 top-level file pins matched. All 18 repository
HEADs, branches and complete dirty states also matched before work. No completed
one-time transaction or acceptance entry was replayed.

## New evidence and correction

The standalone firewall widget has now been compiled from the **final** production
client sources, closing the previous fixture's source-version gap. All six GWT
permutations compiled and linked successfully. Its 541 source/resource inputs are
hashed. The HTTP fixture uses the final production web2 JAR; its server/session
class bytes match the final packaged classes.

The new first fixture rendered a FORBIDDEN alert. Investigation found a fixture
concurrency defect: its request-binding method replaced the servlet's shared
ThreadLocal on every request. A deterministic two-thread proof showed that the
second request removed the first thread's request binding. This differs from
the production servlet lifecycle; production authentication was not weakened.

The corrected fixture initializes one stable ThreadLocal in its constructor,
sets each thread's request and removes that binding after the call. The same
two-thread proof now retains both requests. A further 32 concurrent calls to the
actual production authenticated RPC facade returned 32 distinct, projectable,
writable frozen edits at confirmed revision 1. The provider is fake; prepare,
confirm, rollback and system-write counts are zero. This is a fixture preflight,
not a substitute for browser transactions or actual Kura/kernel validation.

The first helper PID 38574 is stopped and its source/classes/state/error page
evidence remains. The corrected v2 fixture is compiled and **not launched**.
Its one-time launcher verifies the final client inputs and production JAR before
claiming a new run. Compiler PID 38525 and the short probes have exited.

## Remaining blocker and continuation

Both documented locator and native AX clicks on the independent static HTML
button left it unchanged. A native click on the new alert also timed out. Old
test page 18511 and the new failed page 18520 remain the manual error-page
handoff; whether a native alert causes the wider input failure is still unproven.
The existing request for user handling has not been resolved. No browser
transaction, new original-console login, Core CM integration or new VM validation
has passed for this version.

After input recovery, use the prepared corrected fixture in
`target/firewall-gwt-browser-final-20261107-4a2cb19d/browser-v2`, via its parent
`launch-corrected-server.py`, port 18521. Do not rerun the failed first fixture.
Verify actual widget confirm/rollback/lost-reply original-outcome behavior, then
continue through IDEA UI **Kura Firewall GWT v2 20261107** (18519) → fresh Orb →
existing Parallels. The older IDEA provider/v1 configurations are not this gate.
No new full Kura runtime or actual firewall write was started in this follow-up.

Original Orb services remain on the original packages and PIDs; both Parallels
VMs remain stopped. All old/new state, fixtures, keys, backups, quarantine and
volumes are preserved. No commit, push, reset, clean or remote change occurred.

All broader internal pending work and external deferred/historical gaps remain
as listed in the preceding review. Java is available. NM NAT/ownership, center
remote workflow/management-path proof, actual early boot, full recovery scope,
consistent backup/key migration and release are not claimed complete.

Evidence: [firewall-gwt-browser-final-20261107.json](../evidence/firewall-gwt-browser-final-20261107.json).
