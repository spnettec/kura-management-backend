# R013 complete-template crypto checkpoint — 2026-11-12

Historical v3 checkpoint. Export/transfer/preparation status is superseded by
`nm-template-transfer-review-20261112.md`; its old source/artifact seals remain.

Continue the same unapplied `target/nm-role-authority-20261112/workspace/`
candidate. No original source application, live NM/nft action, marker, deployment,
commit or push happened. Full R013 and the original14 management groups remain open.

## Result and evidence boundary

The original1320 inputs still match:1206 initial,102 Core configuration and12 Core
crypto inputs. Current `candidate-source-pins-template-v3.json` records32 changed
existing/11 new files, including three fixture-only changes:40 production/test
files. The standalone Cargo manifest and NM/bridge private-API POM overrides must
never be applied. Old source/evidence generations remain immutable.

The private paired `template-core-final-v2` reactor packages API25, crypto30,
configuration265 and bridge83 passing tests, no failures/skips (403 total).
The same seven new codec contracts have three failures in `template-codec-before-v1`:
coerced schema/field shapes, an opening character limit weaker than sealing's
UTF-8 byte limit, and primitive-array numeric widening. Exact field/number/type
checks and a shared UTF-8 byte budget fix them. Before reports remain retained.

`real-template-crypto-probe-v1.json` runs the actual candidate codec and Core
CryptoServiceImpl against the private classpath. Typed values/password provenance
round-trip; wrong key/revision and legacy default-key export refuse. The probe
uses synthetic in-memory keys, starts no application/OSGi/VM, and is neither OSGi
wiring nor live-provider acceptance. Jars/reports are archived and pinned by
`api-core-template-classpath-v4.json`. No shared-cache install occurred. Prior
native367/15 evidence has unchanged recorded Rust pins; strict/Linux/GUI/current
live candidate acceptance remains pending.

## Implemented contracts

CryptoService's new1.5 default capability is false. Actual Core checks a valid
configured encryption key different from the compatibility default. This does
not attest entropy, administrator separation or CA custody. Core crypto/bridge
now import1.5; other handwritten crypto ranges accept1.5 in the bounded workspace
review. ConfigurationRevisionService's additive1.9 logical read defaults
unsupported. Core normalizes password representation, detaches values and
recomputes the CM change-count/admission revision against that exact copy. A
logical value beginning `encrypted:` is never guessed to be ciphertext.

The codec binds schema1, original operation UUID, fixed NM PID, opaque paired CM
revision and `logical.v1` provenance. Wrapper scalars, Password, primitive/reference
arrays including empty arrays, List and Vector retain type/precision. Unknown,
nested/null, malformed or oversized values refuse the entire template. Opening
requires hash, AES-GCM authentication and exact envelope/types. Char/temporary
encoded-byte buffers are cleared; immutable JSON/String objects mean this is not
comprehensive memory erasure. Diagnostics redact content. Ciphertext is bounded
at4MiB, plaintext at2MiB UTF-8; planned chunks are256KiB. No full-template export
endpoint or transfer persistence is wired yet.

## Immediate connected implementation

1. Agent persists original operation/actor, roles/profile/firewall/NAT baseline,
   paired source, actual agent instance/registered bridge epoch and private deposit
   nonce before asking Java to encrypt. This original transfer reserves other
   writers. Release the OS guard while waiting for the reverse UDS callback.
2. Java reads only the fixed-PID paired logical view with custom-key capability,
   encrypts once, rechecks revision/provider bindings, then deposits bounded
   ciphertext chunks once under a total deadline. Use a bounded outbound request
   overload while preserving the existing LocalAgentService request behavior.
   Do not add a duplicate Java disk store. No plaintext full template goes to
   center or agent.
3. Agent accepts deposits only from the captured authenticated registered-Kura
   connection epoch and original intent/nonce/revision. A caller-provided epoch or
   generic registered boolean is insufficient. Persist chunk pre-intent hash,
   ciphertext and receipt through existing no-follow descriptor/sync helpers.
   Reject duplicate/reordered/foreign/late-cancelled chunks. UNKNOWN is queried,
   never re-encrypted/re-dispatched. Local recovery may prove only original chunks
   already present; it cannot invent missing backend or lease evidence.
4. After complete ciphertext/hash proof and a fresh original-baseline check,
   publish the immutable plan bound to the template receipt/hash. The ciphertext
   payload excludes the eventual plan hash, avoiding circular hashing. Original
   queries may reveal a confirmation token only to the original actor. Provide
   pre-fence cancellation with a durable receipt; pending/READY transfers without
   a settled matching handover must continue blocking competing writers.
5. Reserve the fixed-PID external handler before Core startup. It exposes only
   agent-confirmed generations, decrypting the opaque template in Kura. Avoid
   ConfigurationService recursion from handler methods. Add actual operation/plan/
   mirror metadata after proof. Publish changed roles only with their original
   terminal NM receipt. Preserve Kura-off operation and legacy original recovery.

Actual drain/profile/firewall closure before lease publication, complete mirror,
activation/cancellation/full retirement with fences last, joint keep/disable/set,
NAT-only/multi-WAN/modem/ordinary fields, capability gates and GUI are still one
connected R013 chain. The partial candidate must stay unapplied. Scheme A external
CA and the original three external validation groups remain unchanged.

## Upstream and preservation

All343 applied upstream and20 management pins remain exact. API-manifest overlap
is explicit in `upstream-candidate-overlaps-template-v3.json`; never rewrite old
342/343 seals. `template-preservation-v3.json` records18 original HEAD/branch/
staged/tracked fingerprints outside the prior UI delta,141 exact credential inputs
plus five prior overlays, ca9 eight artifacts, f07 and retained runtimes.

All11 advertised fork tips and relocated CloudCat tip match the last reviewed
pins. Smaller tested functional seals get a tip check; coherent R013 and
pre-release gates repeat full18-repository logical-delta/consumer review. Seven
nonforks skip history merges while retaining source-origin/compile/OSGi/assembly/
runtime analysis. Seven deliberate API2 migrations remain internal pending.
