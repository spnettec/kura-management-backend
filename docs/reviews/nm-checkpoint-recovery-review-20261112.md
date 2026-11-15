# NM checkpoint recovery and confirmation review — 2026-11-12

R005/R006 have a source repair in six agent files. This is a source and isolated
contract result, not deployment acceptance or completion of the management goal.
Evidence root: `target/nm-checkpoint-recovery-20261112/`.

## Resulting behavior

- Journal and immutable intent bind the original logical D-Bus identity, unique
  NetworkManager owner and prior NAT state. Every mutator rechecks that identity
  and pins busctl to the actual authentication server GUID. GetId is a different
  logical identifier and is not substituted for the authentication GUID.
- An unacknowledged CheckpointCreate never claims checkpoints by set difference.
  Recovery quarantines it. Legacy active journals without original ownership
  proof also remain blocked; existing terminal receipts remain readable.
- Owned rollback validates checkpoint devices and rejects newer overlapping
  foreign checkpoints, because NM rollback also removes those checkpoints.
  Unrelated foreign checkpoints survive. Calls use the Manager object and typed
  checkpoint path. Fresh stored and applied state, profile/device identities,
  counters and the original bus identity must prove restoration.
- Confirmation rereads stored and applied candidate fields and NAT before the
  decision, and again after destroy/observed absence. Expiry after a durable
  decision cannot become COMMITTED from absence alone. Lost destroy replies can
  be adjudicated from matching live state without another destroy dispatch.
- Durable rollback/NAT-restore/destroy dispatch flags prevent reissuing ambiguous
  compensation. Recovery preserves NAT it did not change and restores the
  captured prior NAT when the original operation changed it.

The review also found R021: detached compilation computed the edited applied
configuration and its digest but carried the old applied configuration into
Reapply. The candidate now carries the computed value and retains sourceApplied
separately for rollback proof. A contract asserts the actual Reapply payload.

R013 is only partially repaired here. NAT verification now requires the owned
table, enabled lifetime, NAT postrouting base chain/priority/policy, exact equality
matches, bare masquerade and expected comment, without duplicate or extra rules.
Authoritative WAN/LAN derivation, shared ownership with the firewall writer and
the existing-NAT disable path remain internal pending.

## Evidence

- The unchanged-source adversarial baseline: 2 positive controls pass and 10
  counterexamples fail (`before-recovery.json` and log).
- Final native agent: 312 pass, 14 ignored; strict all-target Clippy passes
  (`native-final-v2.json`, `native-clippy-v3.json`). Their six source pins match.
- Final Linux ARM64 NM contracts: 70 pass, 2 ignored; strict all-target Clippy
  passes (`linux-proof-v3.json`, immutable v2 source reused by v3 manifest).
- A real, newly owned private D-Bus fixture proves two correct-GUID GetId calls
  and zero method dispatches for the wrong GUID (`linux-proof-v1.json`). The
  observed logical GetId and authentication GUID differ. Both private bus
  processes stopped. V3 reuses this closed proof only after matching transport
  file hashes and the dispatch method body; the acceptance was not replayed.
- The first candidate exposed an Option<Value>/JSON null encoding error; the
  baseline is now wrapped as an explicit nat member. Compile/fixture attempts
  remain retained. A later dormant-table counterexample failed before the final
  verifier fix. Linux v2 failed under Docker exec's inherited umask 0000; v3
  uses the same immutable source with a test-only umask 0077. Production directory
  permission checks were preserved, and the failed logs remain.

No actual NetworkManager or kernel nft mutation was used. The private container
has no host/system-bus mount. Real NM/cloud-init/physical hardware, amd64 and
disk-full/hard-power remain exactly the original three external deferred groups.
Fresh release/deployment checks remain internal pending. Concurrent external NM
writers do not share the agent lock; these observations do not claim a global
atomic lock over other clients. Historical R003/R004 snapshots remain retained;
the new applied-source pins explicitly supersede the active R004 coordinator pin.

See the applied-source manifest and final preservation audit for exact source
hashes, 18-repository state, protected evidence and owned-container shutdown.
No commit, push, UNKNOWN redispatch or closed generic acceptance replay occurred.
