# Dedicated GWT audit and local configuration plan — 2026-11-07 01a4bbe2

Overall goal active. This checkpoint implements Core admission needed by graph editors; it does not close the GWT Wires concurrency defect.

## Entry and discovery

All 18 HEAD/branch/full porcelain records and all previous 569 source, 1275 artifact and 2611 evidence pins matched. Original Orb four services retained PIDs 244516/244911/286620/268; Ubuntu and Windows were stopped. No live fixture was started this round.

Graph-first tracing found driver/asset, cloud/pub-sub and restricted-service property edits delegate to the general conditional helper. Their full same-page/lifecycle behavior remains unverified. GWT Wires still performs additional-driver legacy updates before calling WireGraphService.update; the latter deletes/creates before its final ordinary update. A general property-only guard cannot protect this complete operation. The bounded audit is not an exhaustive impact claim; initial missing symbol lookups and a stale Core snippet after edits were corrected by exact source inspection and reindexing.

## Implemented foundation

ConfigurationRevisionService adds unsupported-by-default local plan admission and batch paired reads, preserving old callers without a legacy fallback. The typed plan includes property updates, factory creations, factory deletions and revision guards. Core serializes all existing baseline/identity/factory/absence checks with its ordinary update monitor before dispatch. New-PID absence examines dormant CM entries using read-only listConfigurations, never creating entries via getConfiguration.

All validation leases are acquired before deleting anything. Same-PID replacements use the new factory provider definition/defaults. Execution is delete/create/update with one final snapshot; it is not atomic rollback across callbacks. Post-dispatch failures mark all guards and new PIDs uncertain and wrap even a dispatched Rejected as an uncertain operation. Identical plans cannot reuse old baselines. Pending public component views must expose the submitted user values; generated factory identity is not demanded from self-configuring views.

Batch paired reads detach property containers and retain arbitrary public OCD implementations instead of casting to Core Tocd. Limits and direct-CM/component-owned concurrency boundaries follow docs/configuration-plan-local-v1.md.

During validation, the deletion-only empty configuration case exposed an additional boundary: ordinary updateConfigurations(empty,true) skips snapshot saving for an empty current set. The new plan explicitly calls snapshot() when it has no property submissions. A failing regression was retained before the fix; final unit dispatch checks verify explicit empty snapshot invocation. This is not yet an actual on-disk empty snapshot/restart proof.

## Validation and limits

Core complete module install: **130 tests passed**, including 14 new plan tests and existing revision/editor/provider/recovery/snapshot suites. Plan tests execute the real admission code with an in-process dispatcher/CM fixture; they do not prove real factory lifecycle, actual DS callbacks or disk snapshots. Default unsupported API regression failed before implementation; stale guard/new dormant PID/factory replacement/partial failure/delayed views/concurrent plans/container isolation/public OCD/empty snapshot covered. Original red XML and intermediate 128/129 report copies retained.

GWT complete package/install: **120 tests passed**, all **6 login/denali permutations** compiled against the new API. An initial test-phase-only invocation failed two production-policy tests because Maven exposed target/classes instead of the completed JAR; its XML/logs were retained, then full install passed. Initial Core compile used installed old API and failed; after API installation a valid default-unsupported regression was run. A subsequent batch-view constructor compile error (public OCD versus Tocd) was fixed by a detached public ComponentConfiguration wrapper and regression coverage.

No new browser, IDEA, Linux, center-agent transport, firewall or recovery acceptance was executed. Prior selected live evidence remains historical evidence for its own scope, not new plan verification. No new deb, commit/push/reset/clean, old-state cleanup, network control writes or UNKNOWN resubmission.

## Next required work

Follow docs/configuration-plan-local-v1.md in full: revision-aware WireGraph API; persisted topology and complete paired baseline set; graph DTO propagation; all topology/factory/driver planning before Core admission; WireAdmin/tracker actions only after admission with uncertain-outcome handling; complete stale/new/deleted/replaced/empty/two-editor/lost-reply tests; production GWT serialization and bilingual browser; fresh IDEA actual CM/DS/disk snapshot followed by OrbStack and existing Parallels. Current GwtWireGraphServiceImpl and WireGraphServiceImpl remain unchanged and unprotected by the new plan until this is done.

Keep full original internal pending: dedicated editor/CSV/asset/empty/add/remove/component coverage, formal API minimum-version/install/upgrade, production/external PID recovery and empty/partial/inconsistent CM, NM/NAT ownership/adoption/retirement, encrypted-center firewall/remote editing/affected management path, actual early boot/Core handoff, consistent backup/key rotation/migration/release. External deferred remains separate. Historical initial firewall lock-holder cause, two historical evidence gaps and three overwritten old Surefire XML limitation remain unchanged. Full firewall and full Kura recovery are not established.

Evidence: target/gwt-specialized-configuration-20261107; manifest docs/evidence/gwt-specialized-configuration-20261107.json. All previous evidence bytes retained, affected mutable source/artifact originals archived. Original services and all profiles preserved. Goal stays active.
