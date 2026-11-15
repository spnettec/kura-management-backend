# R013 actual PID and isolated SCR — v20 — 2026-11-12

Continue the same unapplied shadow in `target/nm-role-authority-20261112/workspace/`.
V20 supersedes v19's startup PID correctness claim. Original worktrees and all
older evidence remain; R013 is still open.

## Actual defect and correction

V19 used `org.eclipse.kura.net.admin.FirewallConfigurationServiceIPv6`. Both the
actual API constant and DS component declare
`org.eclipse.kura.net.admin.ipv6.FirewallConfigurationServiceIPv6`.
The native assertion repeated the wrong literal, so its previous green result
could not establish cross-language correctness. Current generation and review
projection now use the actual PID; fixture expectations are corrected.

`writer-profile-api-pid-before-v20.json/log` is a genuine failure of the original
publication with the actual expected PID; its38 inputs are retained in
`writer-profile-api-pid-before-v20-source/`. A new explicitly ignored test-only
exporter publishes a fresh private original operation, then exports only the two
framework selections and generation/hash metadata. It copies no JVM/key/token
body and is absent from the normal executable. Before/after exports and their
source evidence remain, with create-only outputs and independent fresh operations.
They do not touch/replay the closed v18 process matrix or real units.

`writer-profile-scr-v20/before-framework.properties` comes from actual old
publication. The same Java probe reports `publishedReservation=false` using the
actual API field, waits with no observer, then fails to reach ACTIVE with those
published PIDs. This is `actual-scr-isolated-before-v7.json/log`, not a simulated
assertion over a copied expected string. With the corrected actual publication,
`after-framework.properties` and `actual-scr-isolated-after-v7.json/log` pass:

- Mandatory observer absence keeps both actual components unsatisfied.
- Both actual SCR firewall facades activate without an agent, with the existing
  legacy bundle installed/ACTIVE and an actual NM barrier implementation object.
- Missing firewall fence, a competing facade and duplicate observers all refuse
  before a transport call.
- Removing the observer deactivates both facades. A replacement creates fresh
  instances; cached old instances cannot reuse its authority.
- Only two synthetic read requests reach the isolated transport. Actual Core,
  real bridge and backend executor do not start in the successful proof.

The observer object is manually registered by the actual ACTIVE NM bundle in
this owned fixture. NM's real DS component and legacy firewall components remain
unsatisfied because backend dependencies are deliberately absent. This proves
actual firewall SCR/registry/lifecycle behavior, not a complete Core/NM startup,
fully satisfied legacy writers, kernel ownership or full system adoption.

## Failed isolation probes are retained

Early probes v1–v6 are failures and excluded from acceptance. The October runtime
lacks the legacy firewall artifact, so it is separately copied/pinned. More
seriously, starting SCR with all runtime bundles at the same start level caused
additional lazy components, including Core/bridge, to activate. An empty isolated
config alone was insufficient. Those processes exited; no cleanup was attempted.

The accepted v7 proof uses explicit empty private configuration/storage, no
startup bundle list, framework start level1, and raises every unselected bundle
to level10. It checks zero applications after framework start and allows only
SCR/Linux-net/NM/legacy-firewall/agent-firewall bundles thereafter. This avoids
dependency autostart while retaining actual class-space resolution.

`isolation-failure-preservation-v1.json` verifies279 original runtime JAR hashes
against v19, no modified metadata among722 original runtime files during the
probe window, and original container/VM states. `personal-runtime-metadata-check-v1.json`
finds no modified metadata among40 personal `.kura-dev` files. These timestamp
checks are not an unavailable historical whole-content baseline. Earlier failed
probe source preimages, results, private runtime directories and logs remain.
Failed accidental activation is never counted as actual Core/adoption acceptance.

## Current source and validation

- `writer-profile-pid-native-full-v20.json/log`:463 pass,0 fail,16 ignored. These
  are the original15 exclusions plus the new explicit exporter. The exporter is
  separately invoked for the new before/after publication; no old exclusion is
  silently treated as passed.
- `writer-profile-pid-clippy-v20`: current all-target strict Clippy passes.
- `writer-profile-linux-v20/build-result.json`: new normal/test builds,44 startup
  contracts and strict Clippy pass; the new exporter remains explicitly ignored
  in that44-case filtered run. Normal ELF is
  `f97e1b737d30606a98553eef64e7f60aabe5282d7d5e109bc5d8c7378b13df87`.
  No normal agent starts here; prior process matrices remain historical.
- `writer-profile-pid-java-reuse-v20.json`: all1107 Java inputs and seven archived
  jars from v19 are byte-exact; reuse2097 package/unit passes without repeating an
  unchanged build. Actual new SCR proof is separate above.

`candidate-source-pins-writer-profile-v20.json` has the same56 changed existing +
31 new files, minus3 fixtures =84 production/test changes. Only two files differ
from v19. `confirmed-writer-profile-source-checkpoint-v20.json` freezes1362 inputs
plus3 fixture reactors. All1331 original inputs and18 Git states remain protected;
all27 owned containers are stopped/retained. Never apply fixture Cargo/POM/reactor
overrides. Review earlier operation/version1 semantics with the retained v19/v18
documents; no old original operation or frozen fixture was migrated/replayed.

## Next work and live upstream gate

Complete actual Core's three reserved handlers and original-bound policy/writer
ownership. In particular, verify fully satisfied legacy firewall DS components:
an activation refusal may leave failed service references, so marker/drain alone
may not close provider selection. A controlled component condition/startup
selection must preserve default standalone Kura and must not silently invoke or
rewrite arbitrary legacy services. Do not claim foreign rules or hide automatic
NAT migration. Confirmed role generations, ordinary fields/modem mappings,
retirement/archive/re-adoption, GUI and complete current live validation remain.

The v20 stage gate found two genuinely new advertised tips:

- Core `06af5ad2cd496b25755bb06607f4e1d38c6ecbe0`: upload/transfer-limit property
  defaults, four files.
- GWT `9301b1c4b1714cfb1b8c8e9754d2813b0214c66d`: bounded upload and shared
  upload/installation admission, fifteen files, including intentionally removed
  Command code that must not return.

Both are exactly one commit ahead of their reviewed tips (connected GitHub
compare evidence). Nine other fork tips are unchanged. Analyze/adapt this coherent
upload-protection cohort before resuming the next R013 implementation. Evidence
is `../upstream-logical-sync-20261112/transfer-limits-delta-v2/`; no application or
reviewed-tip advancement is implied yet. Preserve Java11/GWT/local request lifecycle,
Chinese localization, existing explicit limit compatibility and the seven nonfork
consumer/assembly relationships. Existing8 integrated groups, full14 management
groups,7 API2 and original3 external deferred remain. No commit/push.
