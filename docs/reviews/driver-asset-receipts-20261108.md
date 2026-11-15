# Driver/Asset receipts and permission-scoped startup

Checkpoint `fa088421`, 2026-11-08, extends `8c51aa2d`. Overall goal remains active; full14 internal groups and3 external deferred groups remain.

## Implementation

Dedicated recorded CREATE/DELETE/read/inspect/cancel methods retain WIRES_ADMIN and XSRF. A stable v1 domain mapping separates Core identifiers from generic browser v4 operations without another mutable journal. Typed properties and existing Driver/Asset admission checks are preserved. Browser codec3 retains only correlation and domain. Original success resumes a volatile existing page flow only after matching CORE_COMMITTED plus current READY/ABSENT; failures/reloads/RECOVERED never restore a submission callback. Pending references gate ordinary domain list reads.

Actual WIRES-only testing exposed initialization inside EntryClassUi's ADMIN branch. Moving local retained-reference initialization before that branch restores the domain gate immediately on login, while admin service discovery and all endpoint permissions remain unchanged. No public API/Core/agent/center production changes in this batch.

## Validation

Final Web2 install builds all existing GWT locale permutations with Java11 target; final241 Web2 tests pass. New tests cover stable UUIDv8 golden mapping, returned identity validation, domain codec/legacy compatibility/cancellation marking, continuation gating, XSRF and existing permission annotations, typed single identified plans, wrong-provider rejection, independent receipt access and no retry/compensation after failure.

Fresh native `driverasset-receipts-d8920de2`: IDEA PIDs35199/35849, actual Core259/API plus initial241 Web2, no agent. Normal Driver/actual Asset CREATE and DELETE produce4 complete operations; the referenced Driver UI deletion is blocked. Two additional Driver operations discard original replies once: CREATE ID `op-323470cc-1b75-476b-9c05-765ed12d3616`, DELETE ID `op-ee0912b5-9a93-46c4-9be9-d814969e1eee`. Refresh preserves each ID and dedicated query route, current READY/ABSENT is explicitly reviewed. CREATE spans independent JVM and re-login. Six intents/completions/request/dispatch records, no replay.

Fresh `driverasset-auth-0c592a31`: fixture-only identity setup leaves admin untouched and grants exactly kura.wires.admin to receiptwires. Actual helper rejects kura.admin; admin menus absent. IDEA normal PID39235 creates one Driver, discarding its reply; original ID `op-35d95684-2482-41be-b59d-9335a1af3742` remains retained. Two fixture linkage attempts39796/39991 fail before establishing Core absence and are retained as failed, not passed.

After preserving cache and enabling Equinox timestamp checking only in this fixture, PID40196 disables the actual ConfigurationService DS component, reauthenticates the same identity and queries the original committed receipt. This exposes the startup-panel defect. Final Web2 upgrade and independent PID41056 prove immediate post-login restoration without menu navigation, the same independent receipt query and unavailable current observation with acceptance/cancellation disabled. Its four durable operation files remain byte-exact relative to the stopped normal home. Original production Core/API jars are unchanged. Service absence is deliberately established through SCR; this is not another UNKNOWN recovery or formal hot-upgrade acceptance.

All7 owned JVMs and both proxies are stopped, temporary browser tabs closed, all homes/keys/CM/journals/cache/before jars retained. Authentication scene's browser reference remains intentionally gated and is not silently removed. Screenshots are conversation-only. No historical one-time entry point, UNKNOWN retry, controlled recovery, release, VM action, commit or push.

## Limits and next work

Selected normal Driver/Asset and Driver response-loss checks do not establish every dedicated domain/provider/plan. No forged generic HTTP request was sent; denial evidence is actual helper plus interface and browser restrictions. Broader updates/channel/Wires/cloud/import/clone, all provider ownership/typing/CM strategies, generic deletion recovery, NM/remote/installation/backup/release remain in the parent ledger. External NM/cloud-init/physical, amd64, disk-full/hard-power remain deferred; old exact-lock-holder/missing-evidence limits remain separate.

Next advance generic deletion recovery, without repeating these closed selected scenes. Preserve the authentication fixture's retained committed reference, old original UNKNOWN fixtures and all failed/successful evidence. Weekly quota is checked each turn and before large verification; at exhaustion only minimal closing work may use a little extra credits before pausing. No reset credit is authorized.
