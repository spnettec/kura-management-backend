# R004: stop ambiguous NM writes from being reissued

The original profile writer retries Update2/Reapply for every generic backend
error and whenever the stored version fails to advance. Its durable ProfileIntent
can also be re-entered by a later apply invocation. Five new private unit tests
reproduce these issues against the original implementation: opaque Update2 failure
is dispatched three times; lost Reapply/no-op and acknowledged no-op issue three
Update2 calls; a retained profile intent dispatches a fourth call on re-entry;
and a committed update with a lost reply is incorrectly reported as CONFLICT.

Evidence: `target/nm-dispatch-recovery-20261112/before-dispatch.log`. All five
assertions fail for their intended reason; this is not a compilation/setup error.

The candidate removes the mutation retry loop. Each profile receives at most one
Update2 and one Reapply during the invocation that published its fresh action
intent. Opaque method failures and unavailable/mismatching post-write readback
return OUTCOME_UNKNOWN. Existing identity, version/CAS and source-content checks
before dispatch remain. Success requires both acknowledgements and matching
edited fields; a nondecreasing stored version is accepted because a genuine no-op
need not increment it. The no-op positive control reaches WAIT_CONFIRM with one
call of each method and unchanged version7.

A follow-up full-cycle counterexample found that confirm still required a version
increment. `before-noop-confirm.log` fails against the first candidate with that
specific CONFLICT. Confirm now also accepts a nondecreasing version while keeping
the edited-content check. The complete no-op prepare/apply/confirm path reaches
COMMITTED; two negative-control scenarios refuse a regressed version or changed
stored content without destroying the checkpoint. This stage adds seven unit
functions in total. The first candidate's narrower green result is retained and
is not substituted for final-source acceptance.

On entry, apply refuses retained CHECKPOINT_INTENT, PROFILE_INTENT and
FIREWALL_INTENT rather than forwarding them again. The original durable intent
and operation identity remain available for adjudication. Newly published intents
may perform their first dispatch within the same invocation. The R003 legacy
empty-intent test now expects UNKNOWN on re-entry, while retaining its strict
zero-dispatch and unchanged-journal assertions.

The [official Update2 contract](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Settings.Connection.html)
supports version checks; the [Reapply contract](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Device.html)
has a separate applied-connection version. The current busctl adapter converts
nonzero process exits into opaque errors and does not carry an authenticated,
structured definitive-CAS-rejection result. It cannot safely use a generic error
as proof of non-dispatch. This change does not claim such classification or add
an exception that retries ambiguous errors.

The final full agent run passes293 with14 ignored and0 failures; it contains51
passing NM cases and1 ignored NM environment test. The run captures the actual
coordinator source SHA at spawn. Strict all-target Clippy passes for the same
final source. After145 original build-input checks, only the coordinator file is
applied to the agent worktree. The R003 pin is superseded explicitly; both its
historical snapshot and this stage's matching before copy remain sealed. The
shadow Cargo workspace adaptation is not applied.

All342 upstream source pins,146 credential inputs,8 ca9 artifacts and the f07
seal match. All18 repository HEAD/branches, tracked and staged diffs match the
application baseline (the Rust file is among the retained untracked sources).
Original targets/runtime and live deployments are preserved. All build handles
are terminal; no commit/push or reset/clean occurred. This is a
private source test stage: no real NM/nft writes, live deployment, old one-time
acceptance replay or UNKNOWN redispatch. R005 ownership/compensation and R006
terminal recovery remain separate open findings, as do the original14 management
groups, release/TLS/bulk-enrollment work and original3 external deferred groups.
