# Automatic NAT ownership and explicit intentions — 2026-11-12

This source contract extends the joint NM coordinator. It does not complete the
original network/management scope or authorize a live ownership handover.

## Intention and compatibility

The console offers Keep current, Disable automatic NAT and Set automatic NAT.
Keep omits `nat`. Set carries the bounded `{wan,lan}` proposal. Disable carries
`{"schemaVersion":1,"action":"disable"}` in `nmChange.nat`. Operators select
the action and interface fields; they do not type this protocol object.

Legacy HTTP/stored `nat:null` still means unspecified. Its reconstructed Protobuf
bytes remain identical to the previous empty `nat_json`; queued or UNKNOWN tasks
must not acquire a different intention/request hash after an upgrade. The new
versioned object retains its own JSON bytes across HTTP/storage/wire roundtrips.
Only the agent's coordinator adapter turns that explicit action into a local
`nat:null` removal. The local administrative API distinguishes omission (keep)
and explicit null (remove). A wrong version/extra fields do not match the action
and fail the existing typed NAT validation before effects.

## Owned policy and shared source

`nm/nat-confirmed.json` is a private provenance pointer, not a second network
configuration authority. It binds an operation, terminal receipt hash and NAT
policy. Validation follows the exact active/history journal, main intent and
firewall action, checks hashes/private file metadata, COMMITTED identity and the
immutable effective NAT target. The kernel table/chain/rules must independently
match the full policy semantics. Name/comment alone never creates ownership.

The manual firewall writer excludes `inet yofc_nm` from foreign managers only
with that proof. Unknown namesakes, changed proof, changed rules and other tables
still block. NM and manual firewall writes retain the same system-change guard;
a pending NM transaction reserves the other writer. No independent/manual table
or its confirmed policy is rewritten by the NM coordinator.

Writer observations include profiles, automatic NAT, the provenance marker and
manual firewall revision/confirmed digest in one source lease. Both sides of the
profile observation recheck NAT/marker/firewall state under the original deadline.
Read-only observer configuration stays independent of nft. Writer inspect,
preview, prepare, recompile and remote view use the same joint digest. A NAT-only
or manual firewall revision change therefore invalidates an earlier page's lease.

Preparation captures the proven baseline and resolves keep into a concrete
effective target. The immutable main intent binds that target to journal input.
Before effects, and before the single nft dispatch, the baseline is rechecked.
Unchanged policy issues no nft write. Removal and replacement use the original
per-action intent. A resumed PROFILE_APPLIED stage cannot start NAT after its
checkpoint/window has expired. An uncertain dispatch is not repeated.

Confirmation/compensation fence ownership changes. Only an exact terminal receipt
may be reused after interruption; its file and directory are synced again. The
provenance pointer can be published before the final journal update without
causing a backend resend on recovery. Different/unsafe receipts are retained as
uncertain evidence.

## Remaining parent work

The proposed WAN/LAN roles are not yet derived from Kura's authoritative role/NAT
configuration. This provenance pointer is not a role proof. Adoption/retirement
with the Java writer, full ordinary fields and early boot/consistent release
integration remain internal pending. The subsequent forwarding/read-only batch
implements the fixed kernel IPv4 forwarding gate and binds it to the source
lease; it never enables forwarding. See
`reviews/nm-forwarding-readonly-review-20261112.md`. Actual kernel
NAT and real-NM combined acceptance remain open; mock rulesets do not prove them.
Physical NM/cloud-init/hardware, amd64 and disk-full/hard-power remain the original
three external deferred groups. Scheme A/external CA, Kura independence, existing
fixtures and UNKNOWN non-redispatch remain unchanged.
