# Nonfork origin and relationship review, 2026-11-12

All seven GitHub nonfork projects skip a direct parent-history merge. Their
upstream-derived modules and consumers still participate in functional review.
Evidence: `target/upstream-logical-sync-20261112/nonfork-map/`.

| Project | Source and integration consequence |
| --- | --- |
| kura-cloud | Eight bundles split from Kura. Seven original paths remain upstream; four connectors have the already inventoried message-API removal and must join the API2 cohort. CloudCat moved to official kura-apps and is reviewed separately below. |
| kura-container | Both provider/orchestration modules map to original core upstream paths. Neither has a new path commit after the pinned core merge-base. Core API and local Docker/Jackson boundaries still apply. |
| kura-docker | Independent YOFC artifact assembler. Its Maven plugin copies core/UI/wires/deployment debs and runtime/YOFC dp files; Docker extracts/registers them. These are assembly edges, absent from ordinary POM dependency rows. |
| kura-management-agent | Independent YOFC management implementation. Its versioned Rust protocol dependency is locally patched to the console crate. Published checksum-verified crate/release validation remains internal pending. Kura/agent runtime contracts remain relevant without introducing an agent startup dependency. |
| kura-management-console | Independent center with integrated frontend; shared protocol provider. Keep the historical backend origin URL. PostgreSQL, TLS, frontend and release relationships remain management work even though there is no Maven edge to the Rust agent. |
| kura-yofc-runtime | Independent packaging of Vert.x/Fastjson/Javacan/digitalpetri and other declared runtime suppliers. YOFC imports their OSGi exports; packaging/API changes require consumer resolution and assembly checks. IDE/PDE outputs retain their separate compatibility purpose. |
| yofc-iot | Independent downstream applications consuming core provided APIs, local runtime exports and PLC4X. The current-source shared-cache OPC UA server build passes; all37 embedded PLC4X JARs match the new installation. Generated Kura imports remain part of API2 compatibility work. |

The core merge-base is `a01a10737e094d9f52f96845479d5ea54d938f4d`.
The path map records the fetched core upstream pin and exact per-module history.
Cloud/container extraction roots are respectively `bb431a4b3229` and
`c55d45483358`; these origins explain why nonfork metadata alone is insufficient.

## Relocated CloudCat source

Official `eclipse-kura/kura-apps` develop is frozen at
`61a0fd53f46cbd18960b7bc1d65d155329ba85e8`. Its CloudCat history contains the
August import (`e235edf9be93`) and October style change (`0b8ac49d66cd`). Pinned
GitHub blobs were downloaded read-only and Git blob hashes checked. No remote,
branch or worktree was created or changed in the eighteen repositories.

All four Java files have identical token sequences after removing comments and
formatting while retaining string/character literal tokens. Both DS/metatype
element/attribute/text trees are equivalent. Local configuration `[1.1,3.0)` and
SLF4J `[1.7.0,3)` import adaptations are retained. No new functional patch is
needed; importing upstream formatting/older manifest ranges would add churn.
These two supplemental history records are separate from the54 frozen incoming
fork candidates and are not claimed as newly integrated functionality.

## Scope and continuation

The seven origin/immediate-consumer dispositions are recorded. The306-POM/327-edge
matrix distinguishes tests, development assembly, managed declarations and parent
metadata;26 additional Docker/runtime supplier declarations supplement it. Graph
queries also confirm YOFC source imports of runtime-provided Vert.x web types.
These are bounded source/dependency findings, not a complete effective OSGi graph,
third-party release audit, fresh Docker build or live deployment acceptance.

Keep API2 removal/import changes as a deliberate internal compatibility cohort.
Do not update only an import lower bound or only the export major. The original14
management groups and three external validation groups remain separate and open.
