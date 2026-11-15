# ca9f292d — NM automatic NAT coordination in progress

The full objective remains active. This stage fixes an unsafe prerequisite in the
existing NM update path; actual NM→agent automatic NAT coordination is not yet
implemented. Contract: `../nm-firewall-coordination-v1.md`.

Graph discovery traced FirewallNatConfigWriter construction through
NMConfigurationServiceImpl.writeFirewallNatRules/update/activate. Source confirmed
that asyncApply ran first, monitor stop/clear occurred even earlier, and the late
NAT exception was swallowed. Two new tests failed before the fix by proving that
asyncApply had already been invoked on rejected NAT add/remove requests.

Current changes add a side-effect-free ownership preflight over both current and
proposed inputs, before monitor/cache/backend writes. Both sides are checked so a
requested removal cannot orphan existing automatic NAT. WAN/LAN/type/interface
mapping is shared with existing writer semantics. Late NAT exceptions no longer
fall through to a successful network-change event. No-NAT ordinary agent-owned
updates and input detachment pass; a late NAT failure also fails the success-event
check. No automatic dispatch, second authority, startup dependency, marker, grant
or renderer/service change was introduced.

Final complete NM module:1501 tests passed (4 new),0 failures/skips; Maven3.10,
JDK21 and Maven Bundle Plugin6 produced the bundle. Test fixtures mock the actual
NM/executor interfaces; no real D-Bus/network/nft/runtime acceptance is claimed.
Before copies of the three edited sources and the complete target were preserved.
The NM test source was outside the prior source-pin subset and was backed up from
its actual untouched state; it is not falsely called prior-pin-matched.518 prior
NM pins were checked;91 expected differences all have exact before copies.
The initial baseline script assumed that test source was in the old source subset,
stopped before changing it, and was corrected. Old full untracked/status evidence
and18 HEADs/branches remain preserved. No commit, push, reset or clean.

Code and build evidence: `target/nm-automatic-nat-20261108`, with round.json,
before-fix-tests.log, after-fix-tests.log, nm-full-package.log, prior-nm-pin-audit.json
and progress-checkpoint.json. The current public API remains unchanged; the new
preflight method is internal to the NM bundle's writer implementation.

Full transaction/checkpoint/confirmation/compensation and actual NM integration,
other-writer adoption/retirement, installation, full recovery and consistent backup
remain pending. Original source/derived UNKNOWN and all closed firewall scenes are
retained. The initial preflight step started no runtime. The subsequent observer step below
used only new read fixtures; it ran no historical one-time acceptance. Last complete
seal remains f07d2c1a; this stage is unsealed.


## Independent agent NM observation step

The new optional NM source observer is wired through the existing local
administrative UID/registered-role boundary, default-off by private config and
with no remote capability or write API. It verifies actual bus identity/unique
owner before/after, reads applied version and separately hashes stored/applied
settings, performs a complete second pass and rejects inconsistent data. Program,
arguments, total/per-child time and output are bounded; inherited D-Bus/loader
variables cannot redirect the child. No NM service is auto-started. Raw settings
are not published; absent/unprojectable/unadopted sources cannot become defaults
or write grants. Invalid optional config does not become a Kura startup dependency.

Final native NM contracts10, local UDS regressions17 and strict all-target Clippy
pass; Linux NM10/build pass,107 paired inputs. The initial NM filter also ran4
unrelated environment cases; do not count it as15 new NM cases. The first new
local test was placed in a module without its private connection helpers and did
not compile; after relocation its EOF close expectation failed even though the
responses were correct. Those test-only corrections and raw logs are retained.
The call-target tuple avoids an excessive-argument helper without suppressing lint.

Actual product binary in a fresh Parallels namespace: real running NM1.54.3,
eth0/eth1 managed active profiles and applied versions2/3 were read independently
of Kura; caller-selected interfaces rejected. A distinct new Orb fixture with no
NM owner returns UNSUPPORTED and no fallback. Agents stopped and fixtures were
copied/hashed. nft, sampled links/addresses/forwarding stayed unchanged; route
state and full sandbox were not part of that fingerprint claim. The VM was
started only to access the existing backend for this new read task, then normally
stopped; this is not replay of the closed reboot acceptance. Both VMs are stopped.
The first guest probe's multiargument python -c was split by prlctl; a standalone
shared script corrected the invocation without changing guest configuration.

Source/checkpoints: actual-busctl-envelope.json, linux-build-source.json,
linux-release.json, parallels-probe.json, parallels-observation.json and
orb-observation.json under this batch target. The real D-Bus envelope probe is
not actual NM acceptance; positive actual NM profile data is separate. Existing
Orb four services are retained, and neither Java NM1501 unit results nor these
read checks are claimed as the joint NM/firewall write/checkpoint/compensation
integration. That integration, adoption and formal deployment remain pending.

## Final observer naming and build provenance

The final DTO field is `wiredShape`, a device/type/group-envelope observation,
not a typed editable projection or write permission. The actual positive and
negative Linux read scenes used the retained initial ELF/source pair, which named
that field `projectionSupported`. Only the two NM source files changed for this
name refinement; their exact initial bytes are retained as
`nm-before-view-name-refinement.rs` and `nm-tests-before-view-name-refinement.rs`.
The final 107-input Linux pair is recorded in `linux-build-source-v2.json` and
`linux-release-v2.json`; NM10 tests and final native strict all-target Clippy pass.
Its ELF SHA is fb505357be85c3698a818694d036a875ab5f0fe69b90122fa49021ffe4b13e65.
It was not deployed over the historical fixtures. The read sequence and authority
checks are unchanged; neither source pair implements the joint write coordinator.

## Applied-only source envelope correction

The observer originally checked only stored profile groups for `wiredShape`. A new regression first proved that applied-only unknown groups incorrectly returned true. Both envelopes are now checked; native NM11 and strict all-target Clippy pass. Exact before sources are in `before-applied-shape-check`. Current NM sources differ from linux-release-v2; no new Linux build/deployment is claimed. The retained live initial pair and final-name v2 build remain historical exact pairs.

## DNS priority wire type correction

The existing `NMSettingsConverter.setWanPriority` emitted both route metric and
DNS priority as Java Long / D-Bus x. Official [IPv4](https://networkmanager.dev/docs/api/latest/settings-ipv4.html)
and [IPv6](https://networkmanager.dev/docs/api/latest/settings-ipv6.html) settings
require DNS priority int32 / i, while route metric remains int64 / x. A new
regression verifies both families and priorities -1,0,30,MAX-1, including exact
variant signatures and boundary values. It first failed expected i / actual x;
DNS priority now uses Integer and eight old Long expectations were corrected.
Final NM1502 tests pass with zero failures/skips and Maven3.10/JDK21/BundlePlugin6
package succeeds. This is a wire contract correction, not a real NM rejection
reproduction or live write acceptance. No install/deployment/NM mutation occurred.
Both source preimages and the entire current target were cloned before changes;
all three prior NM main artifacts match that clone. Joint coordination stays open.

## Source-bound local candidate preview

The optional administrative `network.nm.preview` method now compiles detached
NM candidates from a freshly observed sourceDigest and the fixed config allowlist.
It reuses the complete two-pass bus/owner/device/profile observation. Registered
Kura-role peers cannot invoke it; no remote operation/capability was added.
Candidate source equality is an observation check, not an atomic profile CAS or
a write lease. Old/minor-unknown NM versions are rejected; the current preview
requires NM1.40+, a managed active persistent wired profile, and supported group
envelopes in both stored and applied data. Unknown/unconfigured/inactive/unsaved
sources cannot become editable empty defaults.

Typed changes currently cover IPv4/IPv6 method, addresses/prefixes, gateway, DNS,
auto DNS/routes flags and optional WAN priority, plus Ethernet MTU/promiscuous
mode. Present IP-family fields replace the explicit address/gateway/DNS values;
empty lists clear them and null gateway removes it. Omitted families are unchanged;
omitted WAN priority preserves existing metric/DNS priority. Legacy address/DNS
aliases for edited fields are removed so they cannot override the replacement.
Other profile/group/property values, including routes, provider policy and identity,
are retained independently in detached stored/applied candidates. DNS priority is
i and route metric x; family/prefix/duplicate/size/MTU checks reject invalid inputs.
Existing or unspecified IPv6 remains subject to the1280 MTU lower bound unless
it is explicitly disabled/ignored. No setting labels/raw provider values are returned.

The response contains typed proposed values and separate compiled settings/applied
hashes, with writeEnabled=false, prepared=false and requiresAdoption=true. It
creates no durable preparation, ownership, journal, NM checkpoint/profile change,
nft change or confirmation authority. The later coordinator must recompile and
validate under its shared writer boundary and authoritative version checks.
Wi-Fi/modem/VLAN/802.1x editing, additional ordinary Kura IPv6/provider fields,
NM-derived protected automatic NAT roles, adoption and all write/recovery/remote
integration remain pending; this preview does not reduce that full scope.

Native affected results: NM20 (9 new preview contracts), local17 and strict
all-target Clippy pass. Dedicated Linux v3 build uses108 fixed source inputs;
its running/final status is recorded separately rather than inferred from native
results. The old Linux initial/v2 pairs and all live read fixtures remain retained.
No new runtime service or network mutation was started by this preview step.

Linux v3 final: NM20 and local17 pass; release built from108 current inputs, all
source pins rechecked against current files. ELF SHA 72be39bb96134ac5b850104a455c2b57b82b162d19bc264c9a459ad7e79daefd.
This final binary was not deployed.18 HEAD/branches and prior full dirty entries
remain preserved; original Orb PIDs244516/244911/286620/268 active and both
Parallels VMs stopped. The build session is terminal; no new runtime was started.

## Retained NM journal admission foundation

The shared system-change guard now recognizes private `nm/transaction.json`
independently of whether the optional observer is configured. An unresolved NM
intent blocks Network, Firewall, Recovery, Configuration, Identity, OrdinaryBoot
and SystemControl writes; existing read observations remain available. Even the
new Nm owner kind must parse/bind retained evidence before admission. No code yet
uses that kind to issue an NM/network write. Later joint firewall ownership must
be explicitly integrated rather than recursively acquiring the existing guard.

The schema1 header binds a nonnil operation UUID, source/candidate digests and an
immutable intent-file SHA. Pending states cannot carry a terminal receipt hash.
COMMITTED/ROLLED_BACK/CANCELLED require a private fixed-operation receipt with
matching SHA, identity, source, candidate, intent and state. Missing current
journal with retained intent/receipt files, unknown/corrupt states, symlinks,
public files/directories and changed evidence fail closed. Reads never repair,
delete or replay records. The reader checks intent identity, not a future typed
NM/firewall payload or live backend health. Matching terminal file evidence is
an admission condition, not proof that backend operations have been implemented.

This step adds a reader/fence and its contracts. The durable writer, per-action
pre-intents, NM checkpoints/profile CAS, automatic NAT composition, owned firewall
mutation/confirmation and complete reconciliation remain pending. Production
creates no NM journal, preparation, grant or write from this change.
The initial test helper created parent directories with default permissions; two
new cases failed the700 check. The fixture now explicitly creates private parents;
production checks were retained.25 affected NM contracts passed that intermediate
version; final source adds the NM-own-caller corruption regression and uses
unconditional retained-state validation. Final full-native status is tracked in
`nm-journal-full-native.log`; Linux v3 is a prior108-input pair, not current-source
validation of this fence. Graph agent index was refreshed after stale method lines
were detected; this does not claim full parser coverage.

Before the writer is enabled, verify actual NM introspection and the distinct
persistent-profile version source. The official [Update2 contract](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Settings.Connection.html)
places its version-id argument at NM1.44+, while the [Settings contract](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Settings.html)
lists a VersionId property. Current appliedVersion is a separate device-applied
version; neither that value nor a settings hash may be substituted for the
persistent-profile compare-and-set token. Actual daemon/version support and this
property placement must be inspected before implementing profile writes. The
preview's1.40 minimum is not a minimum writable-install contract.

Final journal-fence validation: native268 passed/13 ignored and Linux268 passed/14
ignored,0 failures. Strict native all-target Clippy passes. Linux v4 current109
inputs and ELF rechecked; SHA 2fd4d012efe004df37dd28a01aa782787675123ff85caa8ae717a1a0a953a4fc.
No production deployment or NM/nft operation was performed.18 HEAD/branches and
prior full dirty entries, original four Orb PIDs and Home bytes are preserved;
both Parallels VMs stopped. All compilation/test handles are terminal.
