# R013 original template transfer and preparation — 2026-11-12

Historical v4 checkpoint. Mirror/activation status is superseded by
`nm-confirmed-mirror-activation-review-20261112.md`; its prior seals remain.

This continues the same unapplied `target/nm-role-authority-20261112/workspace/`.
It supersedes only the export/transfer/preparation status of the prior crypto
checkpoint. R013, the original14 management groups and seven API2 migrations
remain open. No deployment, original-source application, commit or push occurred.

## Implemented chain

Java registers optional `kura.network.template.export`. Parameters are exactly
original operation UUID, opaque revision and64-hex deposit nonce. It uses one
fixed-PID paired logical read, the configured nondefault crypto key, and a fresh
paired revision check. Bounded256KiB ciphertext chunks travel once through the
same captured registered session to `network.nm.template.deposit`; the total
export budget is at most20s. Every acknowledgement must match the original
operation/hash/index and exact schema/state. After any attempted deposit, an
error or changed service/session yields OUTCOME_UNKNOWN. No loop retries a chunk
or reencryption. Crypto's DS reference is dynamic0..1. Existing public outbound
request behavior is retained; only the new export uses the session-bound deadline.

Rust admits the original operation/actual local actor and reviewed full baseline
under the shared system-change guard, then persists a private original intent/
nonce before invoking Java. It releases the OS guard during that invocation.
The persistent transfer still inhibits competing writers, allowing the reverse
callback to acquire the guard without a lock cycle. A generic read cannot invoke
export. The bridge clone is checked against the original agent instance/epoch
before queueing; disconnect after queueing remains unknown.

The deposit callback is dispatched from the actual Local.connection task with its
captured epoch, SO_PEERCRED-derived Kura UID and registered status. Generic local
admin calls and caller-supplied epoch/readiness fields cannot invoke it. The
private `nm-template/` store binds original actor/baseline/nonce, manifest, chunk
pre-intent hash, received ciphertext and chunk receipt. Duplicate/reordered/
foreign/late-cancelled deposits refuse. The complete receipt is checked against
every chunk and whole ciphertext hash; READY alone never grants a writer lease.

Local original-operation adjudication can finish receipts from the exact original
received bytes, syncing the same no-follow descriptor. Missing original data
stays unknown and is never fabricated or requested again. A complete original
template can finish preparation even when Java's final reply was lost. The
coordinator rechecks the full original source/profile bindings before publishing
the schema2 immutable ownership plan, which now binds the template receipt/hash.
The ciphertext excludes its eventual plan hash. Legacy schema1 trusted fixtures
remain readable; production preparation requires a real complete transfer.

Local admin endpoints are `network.nm.ownership.prepare`, `.status`, `.original`
and `.cancel`. Actor comes from the actual peer UID. Prepare requires a
caller-known original operation ID and reviewed sourceDigest. Original queries
return a lost preparation token only to that actor; generic status redacts it.
Pre-fence cancellation has an original durable receipt, rejects later chunks,
retains ready proof and reopens other writers only with matching cancellation
evidence. An orphan initial intent can be explicitly cancelled by its actor if
there is no competing/foreign evidence and the controller proves no fence.

## Actual validation and retained failures

- `template-export-java-v3`: API25/crypto30/Core265/bridge91 pass/package together,
  411 total, no failures/skips. New export contracts cover once-only encryption,
  bounded chunks, original identity, CM drift, lost acknowledgements, forged ACKs,
  service/session drift, default-key/old-reader refusal and deadlines. The real
  Java UDS test verifies reverse deposit while waiting on that same connection.
- `real-template-export-crypto-probe-v3`: actual private Core crypto, codec and
  export command interoperate, with typed logical Passwords and ciphertext-only
  receiving callback. Paired revision service is a test double; this is not live
  Kura/OSGi/provider acceptance or Java-to-Rust process integration.
- `template-native-full-v1`:380 pass/0 fail/15 existing ignores at two threads;
  131 NM passes/1 ignore and36 local passes are included. The13 new cases are
  seven transfer, three coordinator and three local registration/export cases.
  Mocked coordinator checks prove no NM checkpoint/profile/nft action or fence,
  released OS guard during reverse deposit, lost-final-reply preparation,
  original query without another export, actor/token privacy and source drift.
- `template-agent-compile-v1`, `template-agent-unit-v1` and
  `template-controller-v1` are earlier same-chain generations. The runner now pins
  both new Rust child files. No warning was suppressed: Source.owner remains
  unused until actual activation is wired. Current strict/Linux/GUI/full live
  acceptance is still pending.

All first attempts remain: Java v1 has an owner-name compile collision; v2 has
nested Mockito stubbing and a fixture that closed its completed connection before
the test's connection poll observed it. The corrected test holds the peer open
with a latch; no product deadline was increased. The v2 real-crypto probe has a
fixture result-variable collision, corrected in v3. Failure reports are retained.

## Current seal and preservation

`candidate-source-pins-export-v4.json`:33 changed existing/15 new files; three
fixture overrides are excluded, leaving45 production/test files. The1320 original
inputs,20 applied management pins,343 upstream pins,141 exact credential inputs
plus five previous overlays, ca9 eight artifacts and f07 remain intact.
`export-preservation-v4.json` compares all18 HEAD/branches/staged/full tracked
changes with `export-start-live-v1.json`. Old342/343 seals remain immutable; the
API-manifest candidate overlap is explicit in `upstream-candidate-overlaps-export-v4.json`.
Private final jars/reports are archived and pinned by `api-core-export-classpath-v5.json`.
No candidate API was installed in the shared Maven cache. Original console/PG
remain healthy, Orb runs, both Parallels VMs and five retained proof containers
stay stopped. No physical owner marker, NM/nft/sysctl mutation or old one-time
acceptance occurred.

At this functional seal, `stage-tip-recheck-r013-template-export-v3.json` finds
all11 advertised fork tips unchanged, preserving all18 Git fingerprints during
the check. Seven nonfork origin/consumer dispositions remain; no new merge or
acceptance replay is needed. The recurring logical-sync gate remains active.

## Next connected work; candidate must remain unapplied

1. Agent confirmed-only metadata/chunk endpoint and actual fixed-PID external
   handler, decrypting only in Kura; never a recursive Core→agent→live Core read.
   FENCED baseline visibility must remain original confirmed data, never a role
   proposal or a caller readiness flag. Match original operation/plan/template,
   actual writer drain, profile/NAT/firewall state before lease publication.
2. Guided controlled startup reservation: Core captures external PIDs from
   `kura.configuration.external.pids` at object creation and refuses unreserved
   handlers. Existing Core bootstrap explicitly avoids reading/replaying those
   PIDs and its handler reference is optional0..n, avoiding a transport startup
   cycle. Preserve this contract. Bind any generated profile/framework reservation
   and necessary controlled restart to the original operation and safe receipt;
   do not ask the operator to set environment variables. Account explicitly for
   the new authenticated registration epoch after a controlled Kura restart.
3. Trusted actual activation, role-confirmed generations, full retirement with
   fences last and archive/history for cancel→new-adoption cycles. Current
   pre-fence cancellation reopens other writers but does not yet implement a new
   adoption cycle in the single-original store. Do not discard old originals to
   work around that gap.
4. Preserve legacy original UNKNOWN/query/compensation, complete joint
   keep/disable/set/NAT-only/multi-WAN/modem/ordinary fields, capability gates and
   GUI. Then strict/Linux/OSGi/current live and guarded application gates, R017
   coherent GUI deployment/TLS hot reload/bulk enrollment/published protocol,
   full14 groups and seven API2 migrations. Scheme A external CA and exactly the
   original three external validation groups remain unchanged.
