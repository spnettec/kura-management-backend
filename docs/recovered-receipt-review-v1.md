# Explicit review of recovered generic operation receipts

2026-11-08, c720b842. Extends generic GWT receipts and persisted read-only identities; full native recovery remains pending.

RECOVERED retains its own state and never means the original request succeeded. Independent inspection accepts only the matching original generic operation shape: a single simple PID and expected factory/readiness for CREATE, or proven absence for DELETE. It performs no mutation.

The ordinary canAcknowledge predicate and original-reply callback still reject RECOVERED. A distinct explicit recovered-review action requires matched RECOVERED plus READY for CREATE or ABSENT for DELETE. It deletes only the browser's value-free correlation and reloads; it does not release Core writes, change a receipt, retry an operation, recreate an instance, or declare full health. Unknown, pending, rejected observation, wrong identity, missing target or mismatched factory do not enable this action.

Web2 229 tests pass, including separate recovered predicates and actual server inspection. Native browser proof of this new action remains pending because the new forward coordination attempt failed its strict CM health comparison before release. Do not record the implementation tests as an end-to-end RECOVERED acceptance.

Identity supplement: adding an already persisted role returns null, removing an absent role returns null at the store interface and false through Felix repository. These calls cause no mutation. All actual mutations remain rejected. Existing role type is preserved even if a different type is requested, matching the normal store's duplicate-name contract. Final useradmin18 tests pass; new native proof of this supplement is pending.
