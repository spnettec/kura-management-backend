# GWT concurrent configuration — 2026-11-07 8f421382

Overall goal active; selected general component editor concurrency scope implemented and verified. Previous checkpoint fd837c85 and original full ledger remain applicable.

## Changes and evidence

Core ConfigurationRevisionService adds a conditional local-editor batch API. Every displayed PID baseline must match before the first write. Ordinary validation/snapshot behavior remains; this is not atomic rollback across provider callbacks. Unsupported implementations reject by default. Post-dispatch failures mark all admitted PIDs uncertain and do not expose a pre-write rejection. Identical-value lost-reply retries using the old token are rejected. A ReadView adapter retains the existing record API while allowing Java 11 GWT compilation.

GWT reads pair configuration and revision, including the OSGi-filter overload used by the main sidebar. Missing baselines cannot fall back to the legacy writer. Conflict and unknown outcomes have English/Chinese messages; unknown does not auto-replay. A successful clean selected form is rebound from fresh server data after the existing listener refresh, so a second save works on the same page; dirty edits are preserved.

Pre-change missing-baseline regression failed in gwt-baseline-red-valid.log. Native v1 exposed the omitted filtered-list overload: even fresh saves conflicted. gwt-filtered-list-red.log retained that regression; v2 uses the corrected real main-menu overload. All failed initial compiler/test/fixture runs and v1 state are retained. Initial missing-path invocations are not counted as valid regressions. Java 11 record compilation and fake-widget focus issues were corrected without weakening product admission.

## Validation

- Core reactor install succeeded (core configuration 110 existing tests); final targeted admission suite 14 passed, including six new editor tests.
- Full GWT package/install and all six login/denali permutations passed; final suite 120 tests.
- Production ServicesUi widget: two bilingual browser tests passed, with fake provider only. Conflict/manual reload/two saves/unknown no-auto-replay verified; this is not real Core unknown injection.
- Native v2 original admin/admin login and actual main Services UI: explicit Core external edit makes stale form conflict with no CM/DS write; Reset refetches without browser reload; two same-page fresh saves succeed. Actual stale batch rejects before first write; fresh batch updates both components. CM/DS and four scalar snapshot fields agree. Runtime 283 pinned files, 282 OSGi bundles. Test external writer calls actual Core API, not center/agent transport.
- Fresh OrbStack and existing Parallels Ubuntu profiles each verify actual GWT server helper filtered reads, stale-batch rejection, fresh two-component batch, CM/DS counts, four snapshot fields and all 283 runtime pins. No Linux browser or center/agent transport is claimed this round. No agent installed. nft rules and forwarding unchanged.
- Parallels private JRE 184 files verified; actual strict sandbox CapEff0/NNP1/EROFS30. Original Orb global overrides remain; actual ProtectSystem=no and NNP0 are not strict-sandbox proof.
- Both code graphs reindexed successfully; exact changed Core/UI paths freshness metadata matched. Graph overload aggregation had missed the main-menu overload and cannot establish source completeness. Wrong-path initial coverage query retained and corrected.

## Preservation and remaining ledger

All new JVMs/widget server/VM units stopped, Parallels shut down normally; original Orb four services retain original PIDs. All new and old profiles, commands, admitted markers, snapshots, results and keys retained. No commit/push/reset/clean, formal deb or historical entrypoint replay. UNKNOWN requests were not resent.

Internal pending: dedicated wire/driver/asset/CSV/custom GWT paths and all components/metatype/empty/add/remove behavior; formal API minimum-version/install/upgrade enforcement; production/external PID recovery and empty/partial/inconsistent CM; NM/NAT other-writer ownership/adoption/retirement; encrypted-center firewall and bilingual remote editing/affected management path; actual early boot and Core bootstrap/install/upgrade handoff; consistent DB/artifact/key backup, rotation, migration and release. Full firewall and full Kura recovery remain unverified. This round does not establish live React writes or center-agent concurrency behavior.

External deferred stays separate: real NM/cloud-init/physical wired gateway, amd64 and hardware/disk-full/hard-power conditions as recorded previously. Historical initial firewall CONFLICT exact holder remains unproven; two historical evidence gaps and the three old overwritten Surefire XML limitation remain unchanged. Prior 2348 evidence pins must stay byte-identical; affected source/artifact originals are archived before mutation.

Evidence: target/gwt-concurrent-configuration-20261107; machine-readable manifest docs/evidence/gwt-concurrent-configuration-20261107.json. Goal remains active; continue full ledger rather than interpreting this checkpoint as completion or pause.
