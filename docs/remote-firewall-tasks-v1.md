# Remote firewall tasks — implementation in progress

2026-11-08, f07d2c1a. This is the remote task contract under implementation;
center HTTP admission/dispatch and the agent remote dispatcher are implemented;
the React workflow is implemented, with selected real host/Linux workflow evidence described below. Broader delivery remains in the parent ledger. The agent
requires an explicit default-off grant. Local/GWT contracts remain
unchanged. The previous complete evidence seal is cc91d577.

The additive protocol reserves operations30–33 for firewall view, prepare,
confirm and rollback, with new ReadCommand fields15/16. Existing operations and
wire fields retain their numbers. View alone is explicitly read-only; unknown
future operations remain conservatively classified as side effects.

Prepare carries expected revision, source digest, confirmation seconds and
structured V3 candidate JSON bytes. These bytes are an internal wire encoding,
not a shell or nft expression. The shared typed grammar preserves both family
policies, ICMP flags, input/source port ranges, interface exclusions/wildcards,
MAC restrictions, port forwarding, and nullable/numeric NAT protocols and rule
types. Unknown fields are rejected at every structured level. The wire boundary
limits candidate bytes to16384, total rules to256, ports to nonzero ordered u16
ranges and the confirmation window to30–300 seconds. Address/interface/family
and NAT semantic checks remain the existing agent V3 compiler's responsibility
before any mutation. The original compiler is not replaced by the wire validator.

Actions use the existing canonical `fw-UUID` transaction identity and32 lowercase
hex token. Tokens alone cannot prove a new affected management connection. The
remote implementation must bind authority to the authenticated center/gateway
generation, require an actual new authenticated control connection after apply
before confirm, retain explicit compensation, and keep local/Kura/remote grants
independent and default off. Client-supplied connection claims are insufficient.

The center must use its existing AES-GCM task input/result storage, tenant/role/
CSRF checks, bounded capability admission and immutable original command IDs.
The agent must durably claim before execution, preserve original business outcome
bindings and answer UNKNOWN through read-only original command evidence. Missing
or inconsistent proof stays UNKNOWN; queries cannot apply, confirm, replay or
repair. Durable pending states and shared system-change ownership remain gates.

The agent now requires `AGENT_FIREWALL_REMOTE=1` for remote dispatch and capability
advertisement. Prepare/confirm additionally require `AGENT_FIREWALL_WRITE=1`;
the Kura UID grant is independent. System-write revocation retains explicit
rollback while remote dispatch remains authorized. Remote shutdown leaves
local compensation and expiry available.

Before execution admission, a value-free SQLite record binds command UUID to
authenticated endpoint/gateway/binding generation and full wire request hash.
It survives receipt pruning. Terminal token-bearing receipt queries also check
this authority before returning a result. The domain journal binds the prepare
and each action before external mutation, and retains immutable settled history
when another local/Kura/remote transaction replaces it. Remote actions cannot
confirm Kura-owned transactions; local/Kura confirmation cannot bypass the
remote connection gate. Root-local explicit rollback remains available.

After actual mTLS stream/welcome completion, transport mints a connection witness
with local establishment time and random UUID. It is fixed for the stream and
absent from client action payloads. Confirm requires a different witness
established strictly after recorded application completion. Successful prepare
requests a new control connection after its durable reply. Existing network
operations retain their original session format.

Interrupted entered actions stay UNKNOWN. Evidence queries read only the domain
journal/history and never invoke nft or repair. A retained applied WAIT_CONFIRM
can prove original prepare acceptance; it does not mean confirmed business
success. Exact committed confirmation and rolled-back compensation can prove
their original actions. If an entered confirmation failed before its decision
and explicit rollback completed, that confirmation resolves FAILED. Missing or
inconsistent proof stays UNKNOWN. Resolution uses the existing encrypted journal
and atomic audit. New history, authority table and encrypted execution records
join the coordinated backup/restore set.

The center now accepts `firewall.view`, `firewall.prepare`, `firewall.confirm`
and `firewall.rollback` through the existing task API. All require administrator,
tenant/session/password/CSRF checks and a fresh `firewall.remote.v1` capability.
Only prepare accepts `firewallChange`; only confirm/rollback accept
`firewallAction`. Inputs/results use the existing AEAD task context and key.
Migration0020 extends operation admission to33 without reclassifying writes as
reads. All new fields retain the existing total task-input bound.

Queued writes are bound to the currently live connection epoch and definitively
expire if unleased when that connection changes. Delivered writes become UNKNOWN
and enter original receipt queries, never a new execution lease. A global UNKNOWN
blocks new writes except explicit compensation; rollback still needs the exact
remote owner/action at the agent. Read operation30 follows the read lease policy.

Receipt validation reconstructs the exact original wire command from encrypted
input and database identity/deadline. A confirmation must match its transaction
and report COMMITTED plus verified new management connection. Original proofs
must match command ID, operation, transaction ID, full request SHA and journal
SHA. WAIT_CONFIRM prepare acceptance remains separate from final confirmation;
explicitly rolled-back failed confirmation cannot become success. Diagnostics
cannot smuggle tokens into failure/UNKNOWN payloads. Non-admin task responses
strip the prepare token. Successful proof resolution gets its own once-only
`task.reconcile.firewall` audit.

New HTTP/PostgreSQL validation covers actual center admission, encryption,
permissions, UNKNOWN/reconnect metadata and token filtering, with no agent or
system action. It does not establish actual mTLS reconnection or nft traffic.

The React gateway drawer now has a bilingual, admin/capability-gated editor with
six family/category views, both policies and ICMP flags. It preserves source and
destination ranges, MAC/interface restrictions, nullable/numeric protocols and
automatic NAT. Automatic rules remain read-only. A new empty configuration needs
explicit initialization; unprojectable confirmed rules cannot become defaults.
The confirmation window is30–300 seconds and every edit invalidates approval.

Draft baseline revision/digest never follows background gateway refresh. Remote
digest now additionally binds the current journal, atomically with confirmed
values; intervening apply/rollback invalidates old drafts even at the same
revision. This is remote-only and does not change the Kura/GWT digest contract.
The defect was reproduced in a new pure unit contract before the fix.

Prepare receipts now carry the exact applied structured candidate. The center
compares it to the original typed request before accepting the result. The UI
can restore this candidate from the original task after refresh and presents it
read-only, rather than presenting old confirmed values as the pending candidate.
Only operation/key/task correlations enter session storage; rule values and tokens
are not cached. Readiness from the current authenticated view additionally gates
the confirm control; the agent still makes the final new-connection check.
Uncertainty permits original intent/task lookup only, never replacement writes.
Final states remove the confirmation countdown and require fresh confirmed reads
before another draft. Task details suppress tokens and distinguish prepare
acceptance, final commit/rollback and exact durable proof.

Selected actual mTLS new-connection/lost-result product workflows and fresh
browser/IDEA/Linux validation are recorded below, with exact source/copy limits.
NM automatic NAT, ownership adoption/retirement, confirmed-only early boot,
remaining domain failure matrices and formal deployment remain in the parent
ledger. Do not replay any closed one-time scenes.

Selected real integration now proves production mTLS reconnection, explicit confirmation,30s expiry rollback and browser acknowledgement loss with original-task restoration and explicit rollback on isolated Linux/nft, preceded by actual host/full-native Kura reads. Agent-stream lost result and UNKNOWN proof remain required in a fresh scene. Existing closed selected scenes are retained and must not be replayed. Later idle-network-expiry correction has separate source/build evidence.

A distinct real mTLS/nft confirmation lost its agent receipt at an explicit test-only process exit. The original center command became UNKNOWN and was never resent. After a test-entry permissions failure was preserved and corrected, a production reader with writes disabled read an independent byte-identical state copy with private permissions, resolved the original domain proof after its deadline, and produced one center audit. Kernel handles/rules and domain bytes remained unchanged. The source fault copy remains frozen with RUNNING execution metadata. Corrected no-task entry smoke proves private SQLite creation; full matrices/formal release remain parent requirements. See the review and selected outcome manifest; do not replay that original command.
