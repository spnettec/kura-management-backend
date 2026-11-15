# Gateway key generations — reader contract and remaining activation gates

This WIP introduces the reader needed for an interrupted key rotation. It does not
rotate a gateway, publish root credentials, enable a rotation button, or remove
any original key. Core12 and all three retained gateway fixtures remain unchanged.

The minimum runtime is JDK 21. Java sources keep `--release 21`; JDK 27 is also
tested. GWT keeps its existing Java 11 bytecode target on a JDK >=21 build/runtime.
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
receipt, complete encrypted multi-key backup/strict matching recovery, a guarded
quiescence/startup handoff, per-provider inventory and atomic re-encryption with
original bytes retained, post-restart reconciliation, and the authenticated GUI
download/verify/confirm flow. UNKNOWN dispatches remain fenced and query-only;
new IDs, package maintenance or a present key never settle them. A key may be
retired only after all dependent retained data/backups and provider boundaries
have an explicit preservation/re-encryption disposition. No automatic purge.

The old single-key backup/restore and its accepted RPC DTOs remain unchanged;
they must not be reused to claim multi-generation backup or full weak-key rotation.
The original internal pending ledger and its three external deferred boundaries
remain intact.
