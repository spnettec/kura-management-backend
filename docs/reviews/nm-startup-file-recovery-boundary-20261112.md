# Original startup file recovery: implementation boundary

Status: execution serialization, explicit recovery review/cancel/once-only file
compensation and receipt-bound restored material are implemented in the same
**unapplied** R013 candidate at `target/nm-role-authority-20261112/workspace/`.
Current native/Linux contracts and actual normal-agent bootstrap/original queries
at six fresh recovery prefixes pass. GUI and full retirement/re-adoption remain
pending; this is not actual Kura/OSGi/NM system adoption. This document grants no live
system authority and does not mark the full management overhaul complete.

## Implemented prerequisite

Stop, the six-step file publication and start now own one non-cloneable,
per-original execution lease from admission through their last receipt/pointer
write. The private `startup-execution.lock` is bound to the operation and startup
plan digest. A create-only `startup-execution-identity.json` binds its physical
inode. Both files are retained; closing the descriptor releases the OS lock,
without deleting or recreating either file. No-follow descriptors, private file
metadata, single-link checks and directory/file identity checks apply.

The lease is checked before each effect and receipt. Missing, partial, foreign,
linked or replaced bindings refuse admission; there is no automatic repair or
new command reconstructed from an old intent. Lost publication/start receipt
pointer acknowledgement also takes this lease. The read-only original query
stays available and never takes an effect lease. The global ownership fence
continues to deny other writer domains; this lease does not grant ownership.

This handles live cooperating agent commands. After process death, an external
service job might outlive its caller: absence of a held lease is insufficient
proof that restoration is safe. Future recovery must separately reject an
active service job and any original start intent. An administrator deleting
both authority files is not a supported lock-repair procedure.

## Current agent implementation, in the same original chain

1. Introduce an explicit recovery review with a fresh UUID/token/expiry, bound
   to the original actor, parent, startup plan, acknowledged stop, complete
   original publication evidence prefix and freshly observed physical inputs.
   Keep schema 1–3 query-only. Link new evidence from the schema-4 original
   journal without rewriting its immutable plan or historical command records.
2. Admit file recovery only with an acknowledged original stop, an original
   publication intent, no original start intent/receipt on disk or in pointers,
   the fixed unit freshly stopped with PID 0 and empty cgroup, no active job,
   unchanged enablement/policy/source grant/other input pins and owner marker.
   Take the same execution lease before inspecting, and again for execution.
3. Inspect directories and file identities through bound descriptors. For an
   absent arguments file, require the retained original inode/hash/metadata.
   For planned arguments, require the same trusted backup and either the exact
   acknowledged image or a freshly reviewed unacknowledged planned image.
   Foreign material, links, a missing preimage, and a same-body replacement of
   the original backup are refusals. Rename may change ctime, but must preserve
   the original inode, hash, size and ownership/mode.
4. Preserve the original backup. Quarantine a present planned arguments file
   with a create-only same-parent rename to a fresh recovery-specific name;
   then create-only publish the original bytes with original metadata. Retain
   fences and generated grants. JVM argument/key/password bodies stay in the
   Kura-owned directory and transient memory, never in agent evidence or API
   JSON. Bind the quarantine destination and every observed preimage at review.
5. Publish the new recovery intent before physical effects, then its own
   per-step and aggregate receipts. A repeated request with an intent only
   observes its outcome; it never reconstructs a command. A missing receipt
   stays UNKNOWN. A complete retained new receipt can acknowledge a lost
   pointer under the same lease and descriptor checks. Do not create missing
   **original** publication receipts or clear any original UNKNOWN.
6. Allow cancellation only before recovery intent. Keep bounded, validated
   cancelled-review history; reject unlinked or orphan active evidence rather
   than allowing a stripped journal pointer to unblock forward actions. Freeze
   new forward startup actions while recovery is prepared/executing/restored.
   Pure original historical queries remain available. UNKNOWN compensation is
   retained for explicit inspection, not silently cancelled or replayed.
7. A restored ordinary profile is selected only through its new receipt and
   fresh physical checks. It cannot fall back to a matching legacy hash. Keep
   the parent fence and deny ordinary/recovery/system-control writes until a
   separately proven retirement/re-adoption phase completes. Report original
   and recovery outcomes independently, with system adoption still false.

## Validation boundary

Use fresh owned fixtures for absent/planned/original argument cases and each
partial recovery prefix; never replay closed one-time acceptance. Include
concurrent live commands, job/running/enablement/policy drift, replaced parents,
hardlinks/symlinks, missing and substituted backups, foreign arguments, wrong
actor/token, any start intent, missing acknowledgements, cancellation/orphans,
pointer-only acknowledgement, and secret non-disclosure. v17 checks the primary Job renderer and exercises empty/active/missing/duplicate
observations in native/Linux contracts. The earlier read-only gateway Job=
observation remains historical; it is not actual service recovery acceptance. No real Kura/NM mutation was authorized by this document.

The remaining full 14 groups, seven API2 migrations, other-writer and required
endpoint closure, typed generations, legacy NAT/logical modems, retirement and
re-adoption, GUI and live OSGi/system acceptance remain internal pending. Only
the original three external validations remain deferred. Keep external CA A.

Upstream integration remains a recurring stage gate: inspect all 18 repository
relationships, group new upstream changes by function and prerequisites, and
review relevant consumers. Skip direct merging for the seven nonforks while
retaining their source/build/assembly/runtime relationships. Never substitute a
raw upstream merge or a commit/push for the retained functional review.

Primary Job rendering reference: [systemd v258 systemctl-show.c](https://github.com/systemd/systemd/blob/v258/src/systemctl/systemctl-show.c#L1135-L1144). The v17 check uses the explicit empty Job rendering and rejects all other output; no active job is inferred from an absent property.
