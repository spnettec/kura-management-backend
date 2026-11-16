# Gateway key generations — reader contract and remaining activation gates

This WIP introduces the reader needed for an interrupted key rotation. It does not
rotate a gateway, publish root credentials, enable a rotation button, or remove
any original key. Core12 and all three retained gateway fixtures remain unchanged.

The minimum runtime is JDK 21. Java sources keep `--release 21`; additional JDK 27
validation has stopped at the user's request. GWT keeps its existing Java 11 bytecode target on a JDK >=21 build/runtime.
The candidate CI guard accepts a parsed Java major >=21, including early access
suffixes, and rejects older/unparseable versions before a build starts.

## Private credential

The fixed optional systemd credential ID is `kura_encryption_keyring`. It is not
an environment value. Its exact binary representation is:

1. ASCII `YOFC-KURA-KEYRING-v1` and LF.
2. Three unsigned bytes: entry count, active-entry index, untagged-legacy index.
3. For each entry: one unsigned length byte followed by that many raw key bytes.

The maximum credential size is 512 bytes. There must be 2–8 unique entries; each
has 16, 24 or 32 bytes, and the active entry must have 32 bytes. Active and legacy
indices differ. Trailing data, duplicates and unsupported lengths fail closed.
Key IDs are the lowercase SHA-256 of the raw key bytes, computed by the reader.
The separate `kura_encryption_key` credential must equal the active entry. A
present unsafe, oversized, malformed or mismatched credential prevents activation;
it never falls back to an environment key, another generation or the default key.
Both bounded credential reads reject links/nonregular files. No key bytes appear
in errors, logs, public results or checkpoint bindings.

## Ciphertext

Without a ring, existing writers retain `base64(iv)-base64(ciphertext+tag)`.
With a ring, every writer uses:

`KURA2.<active-key-sha256>.<base64(iv)>-<base64(ciphertext+tag)>`

AES-GCM uses the existing 12-byte random IV and 128-bit tag. The exact ASCII
header before `-` is authenticated as AAD. Stripping/changing the header, an
unknown generation, a bad tag, truncation or trailing encoded bytes is rejected.
Untagged ciphertext always selects the one designated legacy key; readers never
try keys or implicitly try the public default. Tagged values select exactly the
named retained generation. An explicitly supplied single-key recovery context may
read a tagged value only when its own fingerprint matches; the AAD still applies.

All six value/stream entrypoints share this selection. Stream decryption first
authenticates the entire encoded value, then returns plaintext from a second
explicit-Cipher pass over the retained ciphertext. At most 64 KiB of ciphertext
is retained in the wrapper's memory buffer; larger values spill ciphertext only
into a private 0700 directory/0600 file and remove it on failure or close. No
plaintext spool is written. Provider-internal GCM buffering is separate and is
not claimed to be bounded by this wrapper limit. Authentication/finalization
failures are propagated, including encryption-stream close failures.

The JDK documents that `CipherInputStream` can hide integrity failures; the new
reader uses `Cipher.doFinal` explicitly and releases no plaintext before the
first authentication pass succeeds. Reference:
[JDK21 CipherInputStream](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/javax/crypto/CipherInputStream.html).

The actual configuration writer still obeys `kura.snapshots.encrypt` from the
merged properties. False keeps plaintext snapshot XML. CM scalar/array/vector
password reads and logical-password boundaries are exercised with real crypto;
keystore `store.save` reads old values and writes tagged new ones. This is not
full CM/provider/systemd rotation acceptance.

## Before any gateway may rotate

Root must publish a durable prepared generation, retain both sides, and bind
the package/reader capabilities before changing canonical credentials. Existing
Core12 and old offline readers cannot read `KURA2`; activation must be gated by
the complete capable launcher/provider/recovery cohort, with no downgrade that
silently selects a default key. The new optional reader alone is not permission
to install or synthesize a ring in a retained fixture.

The next work remains the root rotation operation and immutable generation
receipt, released GUI backup/recovery and startup integration, a guarded
quiescence/startup handoff, per-provider inventory and atomic re-encryption with
original bytes retained, post-restart reconciliation, and the authenticated GUI
download/verify/confirm flow. UNKNOWN dispatches remain fenced and query-only;
new IDs, package maintenance or a present key never settle them. A key may be
retired only after all dependent retained data/backups and provider boundaries
have an explicit preservation/re-encryption disposition. No automatic purge.

The existing v1 backup format and accepted RPC DTOs remain available for single-key state;
they must not be reused to claim multi-generation backup or full weak-key rotation.
The original internal pending ledger and its three external deferred boundaries
remain intact.

## Root generation backup, original-set restore and startup admission (2026-11-16)

The native actions now include `generation-backup-create`,
`generation-backup-inspect`, `generation-restore`, and the read-only
`generation-start-admission`. The binary codec and retained-binding parser are
shared in `kura_key_store.py` by backup/recovery and the credential startup gate.
There are no user-supplied paths, commands, keys or replacement generation sets.

Backup creation requires the fixed private `kura_encryption_keyring` credential
and a pre-existing Root-owned `/etc/kura/credentials/key-generations.json`
binding: exact `schema:1`, purpose `yofc-kura-gateway-key-generations-v1`,
`ringSha256`, `activeKeySha256`, `legacyKeySha256` and ordered `keySha256s`.
The complete credential, active canonical key and original provisioning receipt
must match. Missing or corrupt state is refused before INTENT. Backup/status
never invent a key or binding; Root publication of a newly prepared set remains
pending. Do not manually create these production credentials to bypass that gate.

The encrypted payload preserves all 2–8 retained generations, provisioning
receipt, purpose, original UUID and creation time. Purpose and UUID inside the
authenticated payload are checked; relabelling an envelope cannot turn a
single-key artifact into a generation-set backup. Public results and journals
contain encrypted backup bytes and fingerprints, never raw keys.

Inspection can match an uploaded archive and passphrase against the retained
Root binding even when original key files are missing. `matchesRetainedReceipt`
describes that check; `activeKeyReady` and `keyringReady` describe existing
matching files. They never settle a prior UNKNOWN. `restoreSupported` requires
Linux renameat2; `restoreAllowed` also respects original-set matching, unresolved
key/start operations and the restricted recovery peer's missing-file condition.
`restoreBlockedByUnresolvedOperation` reports that fence explicitly.

Restore accepts only the encrypted file/passphrase and expected original ring
fingerprint. Both the uploaded complete set and its active key must match the
retained generation/provisioning receipts. An existing key or ring must already
be byte-identical. Only absent files may be published, with 0400 mode and
renameat2 RENAME_NOREPLACE; no existing file is replaced, and neither receipt is
rewritten. A partial pair or lost final result stays INTENT/UNKNOWN and blocks
new IDs, boot admission and package transitions. Present keys are not evidence
of success. Original-result reads never publish again. The cold recovery peer
may inspect/restore with matching material and read its original restore result;
it cannot export the generation set or request the boot-admission RPC.

Startup has an explicit `check-loaded-generations` mode. It requires loaded
active key, binary ring and generation binding, verifies private owner/ACL/link/
size/stability constraints, then checks the exact pair of public fingerprints
against the Root service over the fixed local socket. Linux SO_PEERCRED must
identify Root; errors, wrong hashes, duplicate/trailing fields, noninteger schema
or unresolved operations refuse admission. No raw key travels in that request.
The ordinary loader also validates a generation file if either extra is present;
explicit generation mode prevents fallback when both are absent. Generation-aware
prepare refuses incomplete credentials or unsafe/unresolved records and preserves
retired legacy key sources instead of selecting/regenerating from them.

The installed default unit has not been switched to generation mode. Controlled
publication still must install/attest the mandatory generation credential and
startup mode, the capable Crypto/launcher/offline-reader/provider cohort, and
actual safe Core drain. `check-loaded-generations` is a package-managed gate,
not a manual environment-variable deployment flow. The old recovery-start DTOs
have not been reused as generation-start authorization. GUI and typed generation
start integration remain pending; no existing gateway has rotated or restarted.

Validation used the real JDK21 Argon2id/AES-GCM worker and fresh temporary
credentials only. Linux ARM64: service 51, startup 15, credential policy 32 and
package lifecycle 20 distinct cases pass (118 total). The 15 startup cases were
rerun after fixing sibling loading under the actual Python -I mode. Tests include
both missing files, either missing half, no-op inode preservation, a concurrent
foreign-file publication, lost second write/final result, UNKNOWN startup/new-ID
fences, retired-key preservation, explicit-mode absence, Root peer authentication
and a real local Root socket exchange. The disposable container used an existing
base image, read-only source/toolchain mounts, no network/capabilities and bounded
tmpfs; it was removed. It did not boot Kura or replace full gateway/platform
acceptance. The three retained scenes still have their seven original processes,
38 private-file metadata records and six old data contents exactly preserved.

Next: guarded Root publication and explicit commit using the prepared generation
and verified backup; complete actual cohort/provider inventory,
atomic re-encryption and authenticated GUI download/upload/start confirmation.
No automatic key retirement, no UNKNOWN redispatch, and no old acceptance replay.
The full original internal pending ledger and three external deferred boundaries
remain open.

## Offline and controlled readers (2026-11-16)

Standalone validation and the controlled Java bootstrap now share a bounded
private key/ring reader. A generation selection explicitly includes the complete
binary ring and its independently supplied SHA-256. Exactly one of ring,
single-key or explicit historical default may be selected. Missing/ambiguous,
linked, public, oversized, unstable or mismatched material is rejected without
fallback. Both the ordinary AES IV prefix and a versioned KURA prefix require
authentication, even under the explicit legacy-password policy.

The controlled reader previously treated logical password text that happened to
be valid ciphertext as already encrypted. This changed the restored value. The
logical-V1 marker now forces exactly one encryption of the literal value while
preserving scalar/array/Vector/List shapes and the source objects. Existing
unmarked authenticated ciphertext retains its bytes through CM conversion.

The offline bundle requires `kura.crypto.key.generations` 1.x, now provided by
the capable Crypto bundle. Built manifests and exact packaged class bytes were
checked. The canonical eleven-module dependency build succeeds; 70 offline
tests pass on JDK21, including eight new generation/logical recovery cases. A
negative control reproduced the logical-value defect before its fix. The actual
packaged `java -jar` validates old/new mixed passwords on macOS JDK21.0.13 and
Linux ARM64 JDK21.0.12.1. Linux used the existing read-only image/toolchain, with
no network, new image, data volume or source copy; its temporary container was
removed. These checks do not establish complete OSGi/package/provider acceptance.

The agent still accepts an explicitly supplied single-key validation source and
its controlled-startup allowlist does not emit ring fields. It must use a typed,
Core-owned delegation path before generation publication; do not grant the agent
raw access to the canonical Root key/ring or infer an adjacent ring. This gap is
internal pending, as are the complete capable cohort and actual installed unit.
No retained gateway was changed, restarted or rotated.

## Root preparation and uploaded-backup verification (2026-11-16)

`generation-prepare` now stages a fresh 32-byte generation under the fixed Root
state directory. The request binds the expected existing key and exact optional
ring fingerprint. A present ring/binding must be complete and match; both absent
selects the explicitly bound canonical single key. Every original key stays in
order, the untagged legacy selector stays fixed, and the new key is appended as
active in the proposed ring. Eight retained generations is a hard limit;
preparation never retires a key to make space.

After a durable original INTENT, preparation writes a private 0700 directory with
0400, single-link, no-replace ring and history files. An existing source ring is
retained byte-for-byte separately. The immutable history binds the original
provisioning/generation receipts, proposed receipts and exact encrypted archive
fingerprint. The encrypted downloadable archive includes every proposed retained
key and uses the existing real Argon2id/AES-GCM worker. No passphrase/raw key is
stored in a public response or operation journal. The private ring is Root-owned
material and is not given to the agent.

`generation-backup-verify` takes the original preparation UUID, expected ring
fingerprint, uploaded encrypted archive and passphrase. It authenticates the
exact original archive, checks the complete private staged set and unchanged
source receipts, and writes a separate durable verification result. It does not
claim that a user has stored an offline copy. Both results say `published:false`
and `requiresExplicitCommit:true`. A second preparation is fenced while the
first staged history exists; cancellation/committed-history disposition is not
implemented or inferred. Original exact-request/result reads do not generate,
encrypt, verify again or change staging. The cold recovery peer may not prepare,
verify or export these proposed generations.

Any interrupted ring/history write, failed worker or lost final receipt remains
INTENT/UNKNOWN. Present staging files never establish success. Original UUIDs
are query-only; new key-operation IDs, Root generation-start admission and package
checks remain fenced by the same validated journals. At the earlier preparation
checkpoint, ordinary single-key boot did not have this Root journal gate. The
later managed-source startup change below closes that gap; retained installed
units remain unchanged. No new canonical key/ring, provisioning receipt, unit,
running process or application data is changed by these two actions.

Linux ARM64 validation now passes 132 distinct native cases: service 65 (14 new),
startup 15, credential policy 32 and package lifecycle 20. The first run lacked
the disposable private `/run` socket directory and skipped one startup case;
rerunning only the 15-case startup suite with that bounded tmpfs passed all 15,
including an actual Root peer exchange. Failure injection covers partial and
lost-final preparation/verification, bad password/tamper/relabel/wrong backup,
stale source receipts, linked/changed staging, immutable result reads, capacity
and UNKNOWN fences. No retained scene or old one-shot acceptance was used.

Actual canonical publication, multi-step commit/recovery, full capable package/
provider/agent cohort and installed generation mode, provider re-encryption,
GUI download/upload/confirm and subsequent key retirement remain internal pending.
These internal native actions are not a released manual deployment procedure.

## Managed admission for ordinary single-key and generation startup (2026-11-16)

Publication review found that the historical single-key `check-loaded` only
checked the loaded file. An unresolved Root operation could therefore coexist
with that valid file and still pass the ordinary pre-start check. A real isolated
negative control reproduces that result; the old command remains a compatibility
reader and is not an accepted rotation-cohort startup gate.

The current source default unit now requires/orders the fixed key-service socket
and invokes `check-loaded-managed`. The shared credential shell helper invokes
the same entrypoint, covering background, foreground/detached and debug scripts
before runtime configuration generation or Java. The gate is package-wired;
enabling it adds no agent dependency, operator environment switch or manual
script step. The three
retained installed gateway units/processes have not been changed or restarted.

The managed entrypoint validates private loaded files and makes one bounded,
read-only `credential-start-admission` request to the fixed Root socket. The only
payload is schema, action, key SHA-256 and the explicit optional ring SHA-256.
Root checks maintenance and the original key/start journals, then matches both
fingerprints to its complete canonical receipt(s) under the existing locks.
Missing loaded generation extras cannot fall back when Root has a generation
set. A missing/unbound Root set, mismatched key/ring, unavailable/older service,
untrusted peer, ambiguous frame or original INTENT refuses startup. There is no
retry, key publication, journal rewrite or UNKNOWN settlement in this request.

The legacy explicit generation RPC keeps its exact contract and shares only the
strict transport. A cold recovery peer cannot call the new boot-admission action.
The new capable launcher and Root service must be packaged together; package
capability/asset attestation, required generation credential loading and actual
publication/drain/commit are still internal pending. The source unit change is
not evidence that a retained gateway has the new gate installed.

Linux ARM64 passes 149 distinct cases: service 65, startup 27, credential policy
32, lifecycle 20 and launcher 5. The 27 startup cases include actual Root peer
communication and the real Python -I managed entrypoint succeeding for the
original single key then refusing an original UNKNOWN without changing files.
Strict mode/key/schema/peer/framing errors and missing loaded/canonical generation
halves are covered. Launcher tests preserve the real three scripts/helper/reader
and replace only the Root transport in a temporary test copy; they verify refusal
before generator/JVM/PID output for all three entrypoints. Their transport is not
claimed to be a second real Root exchange. The final expanded five-case launcher
suite passed on macOS and Linux without rerunning unrelated slow suites. No
image, persistent volume or source tree copy was created; disposable containers
were removed. These are isolated native/launcher contracts, not installed full
Core/OSGi/provider or systemd release acceptance.

## Package reader/launcher pairing and independent cold base (2026-11-16)

Core packaging now includes the standalone recovery JAR and its private classpath,
ordered by an explicit provided Maven dependency. Private Crypto, configuration
and XML mapper libraries must be byte-identical to the actual Core plugins.
Mandatory capabilities, class presence and Java21 class headers are checked;
duplicate reader BSNs, missing classes/capabilities, unsafe files and foreign
classpath entries are refused. Main JAR attributes are bounded to 64 KiB;
signed-JAR named digest sections grant no capabilities and need not be
decompressed. Byte-counted UTF-8 folding is joined before decoding.

The unreleased protocol-2 manifest separates two checks. Top-level files/units
retain the protected backup/cold-recovery base. Its nested `generation` section
binds the main Core unit, reader JARs/private libraries, cohort helper, credential
shell helper and three launcher templates. Base `assets()` does not read/execute
the generation group. A missing/broken main reader, main unit or cohort helper
cannot disable an otherwise intact independent cold base. Protocol-1 base
profiles remain supported without claiming generation capability.

`generation_assets()` verifies the full file group, managed first pre-start check,
socket ordering, Core owner and control-group kill policy. It loads only attested
validator source bytes, never an unverified pyc. Installer RAM tuning may change
only the one numeric Xms/Xmx pair; all other launcher bytes must match their
protected templates. The binding uses exact validated launcher bytes and the
complete manifest hash, without a second unvalidated read. The fixed INSTALL_DIR
token is resolved before hashing the installed main unit. This is package wiring;
it needs no new operator environment switch or manual script step.

Both generation-start admission and managed credential admission when Root has
a ring require this pairing. Failure returns `COHORT_UNAVAILABLE`, without key or
journal changes. Base backup/strict original-set restore remains independent of
the main Kura reader.

Linux ARM64: 165 distinct cases pass (service65/startup27/credential32/lifecycle20/
launcher5/cohort16). Storage and Root transport suites isolate the package check
at that boundary; the cohort suite uses real built JARs and real Manager admission
without mocking pairing. It covers good/bad generation admission and cold-base
independence, same-version mixed artifacts, missing classes, duplicate BSNs,
forged outer hashes, classpath escape, main-unit bypass, RAM transforms, read-only
binding and a second-read race. This is not full installed systemd/OSGi/provider
acceptance.

The canonical eleven-module reader dependency install and independent
three-module ARM64 distribution reactor succeed with the existing shared cache.
Final source: `e4d89bada197b50d5808cf1a19af46ec0f447d50`.
Actual deb: 203385690 bytes, SHA-256
`6cfd7e1ece25c9ed2eab4f5e9b402f9cf0a6c117c34acf403fff292b04cc844f`.
Forty-one actual archive files, exact current preinst embedding and the standalone
JAR extracted from that deb were checked. Its real JDK21 `java -jar` validates
mixed old/new encrypted passwords. The candidate was not installed. Two superseded
debs and their changes files were deleted without backups (406762610 bytes);
one verified current package remains. Git source can rebuild it; old binary or
evidence directories are not a continuation prerequisite.

This is package-file pairing. Actual drain, durable publication/commit recovery,
dynamic provider/runtime/optional-agent readers, typed generation start and GUI
confirmation remain internal pending. A bounded literal inventory is not full
provider coverage. No existing key was published/retired, no retained Core was
restarted, and no UNKNOWN or closed write was replayed.

## JVM lease, guarded publication and independent generation start (2026-11-16)

Source checkpoint: `d19fe030e573aa9894c659a55b2c1d326f308238`, following
`0cd253dbcfd075ce3294c103fa5c1e57c77e8f85`. These are local WIP commits.

All three packaged launchers now enter the shared `exec-java` guard. After
validating loaded credentials, Root validates the actual package and passes one
empty, read-only, Root-owned coordination descriptor over authenticated
SCM_RIGHTS. The JVM inherits descriptor 9 and the shared kernel flock across
exec, preserving the systemd/PIDFile PID. Root releases its copy. Private runtime
registrations bind kernel PID/start ticks, boot ID, UID, original fingerprints,
lock inode and the exact package/launcher cohort. Stale runtime cache may be
pruned during a new lease acquisition; operation/start journals are never pruned
or settled by that cache. A reused live registration is refused. The package
requires writer protocol 1, and the main unit makes the runtime socket directory
read-only even though it can connect to the socket and receive the descriptor.

`generation-publication-plan` is read-only. `generation-publish` binds the
original preparation UUID, successful uploaded-backup verification UUID, expected
ring and exact package cohort. Before INTENT it requires the protected managed
unit, active registered Core PID, matching loaded-source/package binding, cgroup
v2 and no same-UID processes outside the managed control group. A registered or
installed old single-key agent is explicitly refused until Core-owned generation
delegation exists.

After the durable original INTENT, publication preserves exact source key and
receipt bytes privately, requests synchronous fixed `StopUnit` once, then checks
inactive/dead, zero main/control PIDs, an empty descendant cgroup and an exclusive
lock on the original lease inode. It rechecks package, staged set and unchanged
source before replacing canonical credentials. Every retained key stays in the
prepared ring. A fixed package-owned drop-in loads both mandatory generation
credentials; no operator environment switch is introduced. Daemon reload, final
stopped state, canonical set and package are checked before immutable publication
and final success receipts. Any ambiguous failure from INTENT stays UNKNOWN,
including partial canonical files or a lost final journal. Original reads do not
stop, reload, publish or start again. A successful publication leaves Core stopped
and requires explicit start.

`generation-core-start` has its own typed contract binding the original
publication, ring and package. It accepts one fixed `StartUnit`, with a durable
receipt scoped only to `SYSTEMD_JOB_ACCEPTED`. This is not Core health. Ordinary
single-key/generation admission and writer leases require the matching accepted
generation-start authorization after publication. Neither missing ring halves
nor a missing convenience marker can downgrade a successful publication to a
single-key backup/boot. The old restore/start DTOs are unchanged. Lazy helpers now
share the actual service entrypoint's protocol exception classes; the independent
cold base remains usable without the generation helper. Attested helper source
is executed directly, without trusting a pyc cache.

Linux ARM64 validation passes 194 distinct native cases: service65, startup27,
credential32, lifecycle20, launcher5, real-JAR cohort16, writer13 and publication16.
Publication tests use real Argon2id/AES-GCM, storage, package JARs and kernel locks;
their systemd/process boundary is an explicit fake. Failure injection covers lost
stop/start/final receipts, surviving locks, nonempty descendants, partial writes,
changed packages, missing registrations, optional-agent and cold-peer fences.
A separate fresh systemd container passes one real unprivileged JDK21 lease test:
actual Root socket/credential transfer, effective read-only runtime mount, stable
PID, no extra JVM capabilities, kernel exclusion until stop and helper-error
survival without Root restart. This is a JVM probe, not a full Core/provider boot.
Both temporary containers and their private fixture keys/data were removed.

The canonical three-module ARM64 distribution build succeeds. Its actual deb is
203397206 bytes, SHA-256
`a0a435ba7e46d0c72fc4609f78874d8ecb08f5de0e065a0e7f2c5a86274ca02e`.
Forty-three actual payload files, the current lifecycle embedded in preinst and
the real extracted standalone JAR on JDK21 are verified. It was not installed in
a retained gateway. The superseded e4d89ba candidate is disposable; the original
legacy baseline and this current candidate serve different upgrade-test roles.

Remaining internal work includes actual full Core/systemd publication and
restart/reconciliation, full provider/runtime inventory and atomic re-encryption,
Core-owned agent delegation, authenticated GUI preparation/download/upload/confirm
and cold generation-start, package-maintenance start authorization, interrupted
publication recovery and staged-history/cancellation disposition. These native
actions are not an end-user deployment procedure. No key retirement, retained
gateway mutation, original UNKNOWN redispatch or closed one-shot replay occurred.
All twelve original internal groups and the three external deferred boundaries
remain in the open ledger.


## Actual installed Core handoff and persistent templates (2026-11-16)

Current local source: `2282426e4782093aa978b0ada7231823dd35082f`.
The real Core run exercised package `29890a9f547bca29e9ed6ce1657f8c5e61159d8d`.
Its first pre-fix run exposed a real packaging defect: postinst removes the whole
`install/` tree, which also removed the generation template required by admission.
A negative control reproduces this; the template now lives in durable protected
`share/key-management/key-generations.conf`. The installed-tree cleanup regression
passes. Core starts normally with that directory removed.

A fresh Ubuntu24.04 ARM64 systemd container installed the archive and its actual
declared dependencies. The original baseline archive supplied only its legacy
unit as a verified old-key input; that old unit was never started. The installer
retained the original weak key. The actual packaged Core unit then ran with a
bounded 256 MiB heap and no management agent. A small test-only DS consumer used
the real CryptoService and audited ConfigurationService, and required the actual
snapshot reader to be active. Its provider class resources match the installed
package. ConfigurationService intentionally returns ConfigurationServiceAuditFacade;
the verifier now checks that wrapper instead of requiring the internal implementation
object. Production audit behavior was not changed.

The one-shot flow prepared a generation, downloaded/re-uploaded its encrypted
archive for verification, stopped Core through the actual Root/systemd path,
verified cgroup/writer exclusion, published the retained set and required its
separate typed start. The new actual JVM decrypted the original ciphertext with
its original bytes unchanged; new ciphertext carries the published active key ID.
Original/new JVM PIDs were 5189/5597. All four original operation receipts succeeded.
Their IDs are recorded in README and are history-only; the disposable fixture was
removed. These closed operations are not replayed to refresh evidence.

Source validation now includes 195 native cases plus two real systemd/JDK21 lease
profiles (no capabilities/NNP1, actual Core capabilities/NNP0) and one actual Core
flow: 198 distinct cases. This proves one standalone Core/crypto/configuration/
snapshot profile with synthetic provider data. It does not prove all providers,
customer data, the agent, the GUI, all packaging modes or key retirement.

Archive comparison also found host Python caches in the distribution. Common
and variant unfiltered resources now exclude pyc/pyo/__pycache__, and the builder
uses Python -B. The current archive has no host caches. All 333 remaining payload
entries match the actually exercised package in contents, modes, owners and links;
only tar timestamps are normalized. Five cache entries were removed. Normalized
runtime payload SHA-256:
`dd2c19b8b233a39a7635ef7ce793700cefed5a443219ac04a67e8564923f1566`.
This allows the runtime proof to carry forward without repeating the original
mutations. Actual installed-cache handling across package maintenance remains
part of the complete Root/provider release review.

The prior 2282426e47 ARM64 deb was 203325118 bytes, SHA-256
`e99555c4378d86f93056e3c39eb6621f7960823aa13cad33f9a3084a62058fa3`.
Forty-three actual payload files, current preinst embedding and the extracted
standalone JAR on JDK21 pass. Six superseded deb/changes intermediates were
removed without backups (610195021 bytes). Only the distinct legacy baseline and
latest candidate remain; no new image, permanent volume or repository copy was
created. The retained three scenes still have seven original processes, 38 exact
private-file metadata records, six original data contents and unchanged journals.

Next: explicit staged-history/cancellation disposition, Core-owned agent
delegation and authenticated GUI preparation/download/upload/confirm/start;
complete provider inventory/atomic re-encryption, maintenance start authorization,
interrupted-original-operation recovery and formal six-component/platform gates.
No original UNKNOWN was redispatched or inferred successful. All twelve original
internal groups and three external deferred boundaries remain open.

## Core-owned agent validation candidate (2026-11-16, 0d77355a0d / 499bc64)

The new Root read-only protocol accepts one private regular O_RDONLY snapshot FD,
owned by the authenticated caller or the server-resolved Core UID. It verifies the
artifact and stable metadata, copies into Root's PrivateTmp, runs the actual paired
standalone worker with canonical single/ring credentials, checks the exact binding
again and removes the temporary copy. It emits only the existing bounded redacted
report and credential/cohort fingerprints. No operation UUID or journal settlement,
key publication or Core start/stop occurs through this protocol.

Agent schema 2 stores no Java/JAR/key paths. Its private proof includes Core's exact
binding; configuration loading does not contact Core. Controlled-start manifests
can select fixed PID1-loaded credentials with complete key/ring fingerprints,
exclusive of legacy key selections. Missing/wrong/unexpected rings fail closed,
including an active credential inconsistent with the ring writer. Legacy grants
are retained and are not silently migrated.

There are 104 distinct related checks: native validation/FD 12, Java 33, macOS Rust
6, source packaging 6, selected cohort/writer/publication regressions 46, and one
fresh real systemd fixture. The latter uses the actual Root unit/JARs and a Linux
Rust test executable in the packaged agent sandbox with explicit recovery/test
overrides. It verifies Root-stopped configuration loading, hidden private directories,
kurad-owned input FD transfer, old/new password keys and plaintext/old/new encrypted
artifacts, rejected administrative backup access, and unchanged keys/Root journals.
Main Core is never started; controlled Java loading from actual PID1 credentials is
still pending. Historical ignored Linux takeover fixtures were compiled after a
missing deadline parameter was corrected; they were not executed.

The new ARM64 candidate is 203330752 bytes, SHA-256
`a971167dbe641640cb916be7145076e41de88f1efdc7945f70953101456f0992`.
Its 45 actual payload files, current preinst embedding and extracted JDK21 reader
probe pass. It is not installed on retained scenes. The prior 333-entry payload
equivalence applies only to the historical package above, not this changed candidate.

The agent unit hides canonical key/Root state paths and protects cgroups; Root's
socket rejects administrative requests from the agent's fixed cgroup and descendants.
The existing Root agent UID/other host administrative interfaces remain part of the
unfinished least-privilege work. Publication with any installed agent is still
blocked. Effective runtime/binary/grant admission, privileged broker, complete
controlled startup, Root interrupted-intent recovery, staged history/cancellation,
GUI and all provider/re-encryption/release gates remain internal pending.

## Actual PID1 ACL correction (2026-11-16, f718cbf799)

The actual systemd credential reader exposed a production defect: root-owned
`0440` credentials use an exact named service-UID ACL, but Java rejected the mask
bits as owning-group access. Java now reuses the native ACL policy through a fixed,
bounded read-only child, verifies the returned file identity/timestamps/permissions
and content digest, and retains strict legacy file rules. No raw key is returned
by this helper; it makes no Root RPC and creates no operation or receipt journal.
The native protocol declaration and paired Crypto/managed-reader capabilities
also reject incompatible components after an outer-manifest hash refresh.

Current relevant checks: native lifecycle/ACL/proof 33, actual JAR cohort 20, Java
crypto/recovery 26, and one fresh systemd integration test with 32 JVM scenarios
across empty-capabilities/NNP1 and actual Core-capabilities/NNP0 profiles. All pass.
The reader verifies single/ring credentials, old/new data, credential failures and
stale inode/version/digest receipts. Every scenario must pass before the test can
emit its success marker. Main Core/CM application is not part of this reader test.

The new 203332744-byte ARM64 candidate, SHA-256
`1efb07dc1063dbead9f0fe0779cf03ff590d9c4591c93b91ce3dd0a2b88d8d69`,
passes 45 actual-file checks, current preinst embedding and the extracted JDK21
reader probe. It has not been installed on retained scenes. The superseded
0d77355 candidate was removed without a backup; the legacy baseline and current
candidate remain. Main controlled startup, agent admission/broker, the GUI and
complete provider/release work remain pending. Full original scope is unchanged.

## Controlled Main and persistence increment (2026-11-16, 0a6da0e3e0)

The optional bootstrap now fences every non-infrastructure bundle before target
or request validation. A framework stop during initial startup alone cannot
prevent the launcher from advancing. Controlled profiles must explicitly pin
osgi.startLevel 5 through 9; the installed full-Core test pins 6.

One fresh installed Main/PID1-ring/real-CM test passed at source 4c6d50ce63:
17 configurations and 141 properties matched, the original synthetic ciphertext
was unchanged and decrypted, and ordinary writes remained blocked. No generation
publication, completion or write release was invoked. This is selected synthetic
CM acceptance, not customer/provider recovery or formal profile installation.

Source 0a6da0e3e0 separately replaces in-place store.save truncation with private
staging, file/directory sync and mandatory atomic replacement. Read failures
never become an empty store; same-JVM updates retain unrelated ciphertext entries.
Nine relevant Java classes (118 tests) and six Linux/JDK21 filesystem tests pass.
This is not cross-provider rekey or a keystore/database password-change transaction.

Networking c16cf57 and agent 3cf66e4 additionally accept strict KURA2 template
syntax. Actual Core Crypto authenticates the header/content; agent transfers only
exact original ciphertext. Java 19 and Rust 18 related tests pass, including
legacy reads, malformed headers, immutable reopen and original UNKNOWN/no-retry.
The default weak-key template rejection remains.

The current ARM64 package has 45 paired payload checks and the extracted reader
probe passed. Its Main and recovery Crypto classes match the tested classes.
The actual controlled-Main proof remains bound to 4c6, not retroactively to this
new package. See managed-core-start-checkpoint.json and key-provider-inventory.md.
Agent runtime/binary/grant admission, privileged broker, original UNKNOWN
recovery, stage history/cancel, maintenance startup, GUI, all providers and formal
six-component release remain pending. Installed-agent generation publication
still fails closed; old keys are not retired. Continue from Git and OPEN-TASKS,
without restoring old evidence trees or replaying closed operations.

## Immutable preparation read (2026-11-16, d20f4b56eb)

The new internal generation-preparation-read request takes only schema, action
and preparationId. It returns metadata from the original immutable preparation
receipt, with scope IMMUTABLE_PREPARATION_ONLY. It emits RECORDED, UNKNOWN or
NOT_FOUND; RECORDED does not mean published, current, started or ready to commit.
No key bytes, encrypted backup, passphrase or private path is returned. An
original INTENT is never completed or used to load candidate material.

Six isolated Linux/JDK21 filesystem/crypto model tests pass under UID65534,
empty capabilities, NNP1 and no network. The actual packaged cipher is used;
Core cohort admission is mocked as in the existing storage fixture. Tests cover
metadata and exact file preservation, missing canonical keys/other UNKNOWN,
missing/changed/linked history, strict fields and recovery/agent peer refusal.
These are not live Root-service, full runtime or GUI acceptance.

The candidate package has 45 payload checks and the extracted reader passes.
All nine native key scripts exactly match Git and distribution sources. The
existing nonempty preparation-directory guard remains: cancellation, complete
history, second-round admission and GUI must be implemented explicitly without
deleting original history or retrying unresolved operations.
