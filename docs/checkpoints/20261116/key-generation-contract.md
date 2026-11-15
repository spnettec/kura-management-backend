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
