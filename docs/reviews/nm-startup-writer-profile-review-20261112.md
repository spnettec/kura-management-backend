# R013 startup writer profile — v19 — 2026-11-12

This is the same unapplied candidate in
`target/nm-role-authority-20261112/workspace/`. Original sources, runtime and all
older acceptance remain preserved. R013 remains open.

## Defects and changes

The generated startup arguments selected the agent firewall provider but reserved
only the network PID. A genuine original-publication counterexample failed with
the missing IPv4/IPv6 reservations. New schema4 plans now bind
`generatedArgumentsVersion:2` and reserve the network and both firewall PIDs,
preserving unrelated arguments and existing reservations. No operator environment
variable or manual bundle deletion is introduced. Publication uses the contract
bound in its original plan; missing version means the historical version1.
Historical schema1/2/3/4 identities, argument hashes and original bytes remain
queryable without silently upgrading or regenerating their material. Stripped or
unknown generation bindings are refused. New plans still store hashes and typed
grant metadata, never raw JVM argument/key bodies.

The existing confirmed firewall facade also refused any installed legacy firewall
bundle. Its real before-counterexample failed even with the participating writers
drained. The candidate now permits installed legacy code only with exactly one
active local NM writer observer, durable network and firewall fences, actual
participating-writer drain, and no foreign/competing IPv4 or IPv6 facade.
Observer references and bindings are rechecked; every later agent call repeats
the proof, and deactivated instances refuse calls. No agent request is made by
activation/drain. Both actual DS descriptors have a mandatory static local writer
observer; the agent transport remains optional0..1. Kura still starts independently
of an agent. The firewall module imports net.transaction[1.1,2).

## Current evidence

All paths below are relative to `target/nm-role-authority-20261112/`.

| Evidence | Actual result and scope |
|---|---|
| `writer-profile-before-v1.json/log` | Genuine old-firewall coexistence counterexample failed; original1034 inputs and reports retained |
| `writer-reservations-before-v1.json/log` | Genuine old startup publication omitted fixed firewall PIDs; exact38 inputs retained |
| `writer-profile-native-full-v1.json/log` | Current full native463 passed,0 failed,15 existing ignored; test threads2,38 exact source pins |
| `writer-profile-clippy-v1.json/log` | Current all-target strict Clippy passed,38 exact source pins |
| `writer-profile-java-full-v2.json/log` | Current seven-module package2097 passed,0 failures/errors/skips;1107 exact source pins; no shared API install |
| `writer-profile-classpath-v19.json` and `writer-profile-test-summary-v19.json` | Seven immutable jars and raw reports archived under `artifacts/confirmed-writer-profile-final-v19/` |
| `writer-profile-package-contracts-v19.json` | Actual packaged import and both DS descriptors verified |
| `writer-profile-linux-v19/build-result.json` | Fresh bounded network-none Linux build/test-build/strict Clippy passed;44 startup/recovery contracts passed |
| `writer-profile-osgi-resolution-v3.json/log` | Actual Equinox resolves the seven private modules, shares the actual writer API class, and invokes actual NM/network/firewall barriers; no application bundle activated |

The initial Java after-build retained two test expectation failures: the public
API wraps the local exception in FirewallTransactionException. Expectations were
corrected without weakening CONFLICT/no-agent-call assertions. Rust focused v1/v2
retained historical-fixture failures: those fixtures incorrectly retained new
generation fields/hashes after declaring older schemas. Genuine historical
generation material is now constructed; original no-replay/privacy assertions
remain. Full current native and Linux tests above pass. Failed evidence is kept.

OSGi resolution v1/v2 also remain as failures. The old October development runtime
lacks the existing snapshot-codec capability required by current Core. Adding the
actual prior-built codec companion, whose20 current original source inputs still
match, resolves the cohort. Its jar and provenance are pinned in
`writer-profile-codec-companion-v19.json`. Production import/capability checks were
not relaxed, and the running development installation was not replaced.

Linux current normal ELF is
`0fbcbd3cc4309c820781bbe652e16b3a9e3c23a6900057cdcced10878da5b177`.
It was built here; this stage starts no normal agent process. The closed v18
six-prefix/twelve-start proof belongs to its historical, different ELF and was
not replayed. This stage's owned build container is stopped; all26 owned
containers remain retained and stopped.

## Source boundaries and remaining work

`candidate-source-pins-writer-profile-v19.json` seals56 changed existing files,
31 new files and three excluded fixture changes:84 production/test changes.
There are nine file deltas from v18. All1331 baseline originals still match.
`confirmed-writer-profile-source-checkpoint-v19.json` freezes1362 source files
plus three fixture reactors. Never apply the standalone Cargo.toml, NM/bridge
POM overrides or the three fixture reactors. Existing v18 and earlier seals are
immutable. `writer-profile-preservation-v19.json` verifies all18 HEAD/branch/
tracked/staged states,20 management pins,343 upstream pins, original runtime,
credential boundaries and ca9/f07 evidence.

This proves local package/class-space/barrier contracts. It does not prove actual
SCR activation, running Core's three external handlers, full system writer
ownership, legacy firewall/kernel-policy migration, actual NM/cloud-init/hardware
behavior, GUI or deployment. Network Core owner observation currently binds only
the network PID. Complete firewall/automatic-NAT ownership and all required
endpoints still need an original-bound proof; a provider flag, empty registry,
fence, drain or stable digest alone cannot adopt foreign rules. Confirmed role
generations, logical modem mapping/ordinary fields, retirement/archive/re-adoption,
GUI, current Kura/OSGi/live validation and guarded application remain internal
pending. Preserve original UNKNOWN and do not replay uncertain actions.

Upstream advertised tips were checked at this tested functional seal:
`../upstream-logical-sync-20261112/stage-tip-recheck-r013-writer-profile-v19-close.json`.
All11 fork tips are unchanged and18 repository states were preserved during the
check. The seven nonfork origin/build/assembly/runtime relationships and eight
already integrated functional groups remain in that stage's ledger. Full delta
review remains required at coherent R013 and before release freeze, sooner for
relevant security/correctness changes; no bulk merge or commit/push.
