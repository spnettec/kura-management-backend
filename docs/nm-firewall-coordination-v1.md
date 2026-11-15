# NM automatic NAT and firewall coordination — in progress

2026-11-08, ca9f292d, after sealed f07d2c1a. NetworkManager remains the system
network authority. Existing Netplan code/state/evidence are retained; this work
creates no renderer migration, service removal or second persistent authority.

## Current implementation boundary

The existing NM update path applied network settings before asking the NAT writer
to apply its rules. NAT ownership failure was caught inside the helper, so the
update could still start DHCP/DNS monitors and publish a successful network event.
Two new regressions failed by observing the actual mocked `asyncApply` invocation
for both an attempted NAT addition and removal before the ownership refusal.

The new preflight checks the previous and proposed NAT inputs while the service's
cached properties and monitor states are unchanged. It reuses the original WAN,
supported LAN/NAT and resolved interface semantics. An agent-owned request needing
an actual cross-interface NAT pair is rejected before network application, DHCP/
DNS monitor stop/clear, cached-property mutation or NAT writer construction.
Checking the old state also protects NAT removal; an empty proposed NAT list does
not by itself authorize retiring existing automatic rules. The late writer gate
remains. An eventual NAT error propagates to the update handler and cannot publish
a successful network event.

No-NAT ordinary network settings remain usable under agent ownership; legacy
ownership retains its existing writer. Agent startup/UDS availability is not a
preflight dependency. No marker or grant is created by this code.

Four new cases are included in the complete NM module result:1501 passed,0 failed,
0 skipped; Maven3.10/JDK21/Maven Bundle Plugin6 packaging passed. These are isolated
unit/provider contracts with mocked D-Bus/executors/monitors, not real NM or nft
transaction acceptance. Before-source copies and an entire prior NM target clone
were taken before the build.518 affected prior pins were checked;91 expected
changes have exact prior copies. All18 repository HEADs/branches remain unchanged.

## Still required for the actual coordinator

The preflight refusal is not an implemented automatic NAT transaction. A supported
coordinated path must bind one proposed NM configuration and its automatic NAT set
to current NM/backend ownership and firewall revision, preserve independently
managed/manual firewall rules, and reserve the shared writer boundary before any
NM checkpoint/profile or nft side effect. A caller-provided automatic flag is not
an NM ownership proof.

NM checkpoint/application completion, nft application, explicit confirmation and
compensation must have durable original operation identities. Confirmation needs
a new affected management connection. Lost or partial results stay UNKNOWN and
are queried by original evidence; neither a timeout nor a reconnect authorizes
another write. Late NM/NAT failures still require coordinated compensation; this
preflight does not roll back an already-issued asynchronous NM update or make the
current callback's CM projection an atomic success receipt.

That path must cover the retained ordinary Kura NM fields, current/default/empty
semantics, source leases, protected automatic rules, adoption/retirement and formal
install profiles. Kura remains independently usable without an agent. Existing
successful GWT/NAT/remote-firewall/lifecycle one-time scenes must not be replayed.
New native and isolated Linux NM validation is still required; physical NM/cloud-
init/hardware, amd64 and disk-full/hard-power limits remain separate deferred work.

## Agent NM source observer

The agent now has an explicit local administrative `network.nm.inspect` read API.
`AGENT_NM_CONFIG` selects a private owned0600 JSON file with schemaVersion1 and a
fixed allowlist of1–16 interface names. Omission leaves this observer disabled;
invalid optional configuration disables it without making other agent facilities
or Kura startup depend on NM. Requests accept null/empty parameters, never a
caller-supplied target, bus endpoint, command or program. A configured fixed local
Unix bus address is optional; the default is the system bus. Registered Kura role
connections cannot acquire this administrative method even with a shared UID.

Calls use fixed `/usr/bin/busctl` with an isolated environment, JSON output,
no service auto-start and no interactive authorization. The five-second total
deadline starts before queuing a blocking worker; each child call is bounded to
two seconds or the remaining budget, and uses the existing bounded process-group
kill/reap and output limits. The observer binds bus GUID, unique NM owner and NM
version. All NM reads target that unique owner. Both passes must return matching
device/interface/active connection, applied version, profile UUID, hashes,
persistence flags and state, and the final bus/owner identity must still match.

Applied and stored-profile hashes are separate; no active profile is explicit
absence, not a default candidate. Raw settings/labels/secret-like values are not
returned. Unsupported profile groups/types are unprojectable. `writeEnabled` is
always false and each active profile still requires adoption. This is a source
baseline for the future writer; hashes alone do not replace profile CAS, a shared
writer lease, checkpoint ownership or a durable command receipt. There is no NM
write/checkpoint/profile mutation or Netplan fallback in this method. The old
separate Netplan prototype entry remains historical code, not an NM backend.

Native contracts:10 NM source cases and17 local UDS regressions pass; strict
all-target Clippy passes. Linux ARM64 NM10 tests/build pass against107 fixed source
inputs. On the existing Parallels Ubuntu26.04, the production binary independently
read actual NM1.54.3 and two managed active profiles (applied versions2/3), including
parameter rejection; Kura was not a service dependency. In a new Orb read fixture,
NM was absent and the same API returned UNSUPPORTED without starting a service or
falling back. Sampled nft, link flags/MTU/addresses (excluding lifetime counters)
and forwarding settings match before/after. These are positive/negative read
checks, not NM change, checkpoint rollback, Java update integration or physical
network acceptance. Both agents stopped; the reused Parallels VM returned to its
prior stopped state and Windows stayed stopped; all fixtures remain.

Method contracts were checked against the official [NM device API](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Device.html),
[NM settings API](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Settings.Connection.html),
[checkpoint API](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html)
and [busctl documentation](https://www.freedesktop.org/software/systemd/man/latest/busctl.html).
An actual systemd255 D-Bus JSON envelope was also captured; this was not called
actual NM data. Initial test placement and EOF-closure expectations failed, were
corrected, and their raw logs remain; no production authorization was relaxed.

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
