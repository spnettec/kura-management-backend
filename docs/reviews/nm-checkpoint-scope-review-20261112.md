# R003: restrict the NetworkManager checkpoint device set

The checkpoint review found that the coordinator passes an empty device list.
The [official NetworkManager manager API](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html)
defines this as a checkpoint over every device, which exceeds the proposal's
affected interfaces. This is an internal implementation defect, not a new
external validation condition.

The candidate recompiles against fresh source before publishing the checkpoint
intent, collects only changed device object paths, and sorts/deduplicates them.
The durable action payload carries this explicit set. Dispatch validates a
nonempty set of Device object paths and sends it as the actual `ao` argument.
Empty legacy checkpoint intents fail before a call; the implementation never
substitutes all configured interfaces for an empty set. The original second
source check before profile-intent publication remains.

Three new tests inspect actual fake-wire arguments and mutation counts: an edit
of eth0 must exclude unchanged allowed eth1, changed device resolution must stop
before checkpoint creation, and an empty retained legacy intent must not dispatch.
All three fail against the original production implementation. With the candidate,
the NM unit selection passes44 with1 ignored environment test. Tests explicitly
provide the added read observation; mutation expectations have not been weakened.

Evidence: `target/nm-checkpoint-scope-20261112/`. The source baseline covers104
agent and41 console/protocol build inputs. The shadow agent manifest adds only
an explicit standalone workspace/resolver3 to prevent discovery of its ancestor
console workspace; that build-fixture adaptation is not for production. The
existing native target cache was cloned with APFS copy-on-write; the original
target was not used as an output directory.

Setup failures are retained separately: the first shell had no cargo executable
on PATH, and the next attempt discovered the enclosing console workspace. Neither
ran tests. `before-checkpoint-scope-v2.log` is the authoritative three-failure
counterexample run. The first unbounded source copy also included a historical
retained runtime; that owned copy process was stopped, originals were unchanged,
and the partial copy was retained. Build inputs were then restricted to source,
Cargo manifests and protocol crates. No historical runtime copy is claimed complete.

Full agent units pass286 with14 ignored and0 failures; strict all-target Clippy
passes. After145 original input checks, the coordinator file alone is applied to
the original agent tree. The standalone shadow Cargo-manifest adaptation is not
applied. The selected source before/after hashes, before copy and exact patch are
retained in `applied-to-working-tree.json` and `candidate.patch`.

All342 upstream pins,146 credential inputs,8 ca9 artifacts and the f07 seal match;
all18 HEAD/branches and staged diffs remain unchanged. All owned build handles are
terminal. Original targets/runtime and deployed services are unchanged. R003's
source defect is repaired; fresh release/Linux integration remains internal
pending. R004/R005/R006 and remaining review findings are not closed by this repair.
No real NM/nft/remote mutation, UNKNOWN retry or prior one-time acceptance replay
occurred. The original14 management groups and3 external deferred groups remain.
