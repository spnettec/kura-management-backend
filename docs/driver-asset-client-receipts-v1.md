# Driver/Asset retained operation receipts v1

2026-11-08, in-progress batch `fa088421`, following sealed `8c51aa2d`. This extends the generic receipt contract without closing the complete dedicated-domain or management redesign ledger.

## Permission boundary and stable correlation

Driver/Asset creation and deletion now use recorded methods on `GwtDriverAndAssetService`, retaining its existing `WIRES_ADMIN` requirement and XSRF checks. Generic component RPCs remain `ADMIN`. Mutation admission still checks that the factory or live PID provides Driver/Asset. Receipt read/inspect/cancel does not require that live provider: deletion and unavailable Core must not make the independent receipt identity inaccessible. Current instance inspection may itself be unavailable and never changes the original receipt.

The browser retains the random client `op-UUID` and fixed domain `DRIVER_ASSET_V1`. The server maps every mutation/read/inspection/cancellation to a deterministic internal identifier:

1. SHA-256 over UTF-8 bytes of `YOFC Kura GWT DriverAsset operation v1`, one NUL byte, then the complete canonical client ID, including `op-`.
2. Take the first 16 digest bytes. Set byte 6 high nibble to `8`; set byte 8 high two bits to binary `10`.
3. Interpret those bytes as a big-endian UUID and prepend `op-`.

Golden example: browser `op-74e632af-1447-430a-9d78-fd513da24117` maps to Core `op-c5e1c044-d725-8d61-83bb-8081c5786cb5`. This v1 mapping must remain stable across upgrades, relocation and backup restoration. Core request/dispatch/intent/completion/recovery diagnostics use the internal UUID; the browser shows its original external UUID. A returned DTO is rebound to the external ID only after its internal ID matches the derived value.

The v8 internal namespace separates these endpoints from ordinary generic browser v4 operations; the caller cannot supply a raw internal ID or select a namespace. This is a permission-domain partition, not a per-user ownership or confidentiality ACL. Trusted ADMIN and in-process OSGi code retain their existing authority. There is no new mutable mapping journal, key, secret or request payload cache.

## Browser behavior

Codec 3 adds only the fixed domain after the existing correlation fields and cancellation-issued marker. Earlier codec 1/2 records decode as GENERIC; generic encoding stays codec 2. Unknown domains fail closed. The existing per-origin/module storage prefix and Web Lock serialize pending actions across both domains. No properties, revision, token, password, callback or replayable request is persisted.

The three existing Driver/Asset create/delete entry points reserve the reference before the original RPC and never resubmit it. Query/inspect/cancel route to the domain endpoint after refresh. Cancellation still persists its dispatch marker under the lock before the single RPC. A retained reference gates the ordinary Driver/Asset list fetch.

On timely original success, matching CORE_COMMITTED plus current READY/ABSENT permits safe reference removal before a volatile page continuation reads the new descriptor/configuration or refreshes the existing list. Reload, failure, UNKNOWN, REJECTED, CANCELLED and RECOVERED do not restore that original continuation. Explicit review after a lost reply reloads the page. The existing driver-with-assets and Wire-managed-asset UI restrictions remain; this change does not claim a new server-side dependency closure.

## Evidence and remaining checks

Full GWT Java11 locale compilation/install succeeds. Web2 has 241 passing JVM tests, including stable mapping, DTO identity checking, codec compatibility/cancellation marker, continuation gating, XSRF/permission annotations, typed identified plans, independent receipt access, invalid-domain rejection and one-call UNKNOWN behavior. Logs are in `target/driver-asset-receipts-20261108`.

Fresh no-agent native Kura, launched through IDEA, used final Core259/API and this Web2. A no-device-I/O fixture Driver and the actual Asset provider verified normal Driver creation, typed Asset creation, UI refusal to delete a referenced Driver, then Asset and Driver deletion. Four identified operations completed. The Driver default Integer remained typed and visible.

Two further new operations discarded their original HTTP 200 replies through a loopback proxy: Driver CREATE and DELETE each submitted once. The original IDs survived refresh, used dedicated receipt endpoints and required explicit current READY/ABSENT review. CREATE also survived an independent JVM and re-login. Final journal contains six intents and six completions, with corresponding scoped request/dispatch records. The proxy retains metadata only; response payloads are not retained or decoded as acceptance evidence. Native proof summaries and stopped-home copies are retained.

An additional fresh fixture `driverasset-auth-0c592a31` uses a setup-only account `receiptwires`, with exactly `kura.wires.admin`; existing admin is untouched. Its real permission helper denies `kura.admin`, admin menus are absent and the actual browser can create one Driver through the dedicated endpoint. Its original response was discarded once; original ID `op-35d95684-2482-41be-b59d-9335a1af3742` remains retained.

In independent JVMs, the fixture disables the actual ConfigurationService DS component through SCR. Production Core/API jar bytes are unchanged. The same account reauthenticates without that service and reads the original CORE_COMMITTED through the dedicated endpoint. Current inspection is unavailable, so acceptance/cancellation remain disabled. Request/dispatch/intent/completion bytes remain exact. This is service-absence validation, not another actual UNKNOWN recovery scene. No forged generic HTTP request was submitted; generic denial evidence combines the actual permission helper, interface contracts and restricted browser menus.

That browser scene exposed a real startup defect: EntryClassUi restored retained references only inside its ADMIN branch. Initialization now runs for authenticated users with nonempty permissions before the ADMIN-only service list; domain endpoints retain their own authorization. The pre-fix account had to open Drivers/Assets to restore its panel. After rebuilding and upgrading the isolated Web2, a new Core-absent JVM restores it immediately after login, without clicking a menu. Full GWT locale build and all241 Web2 tests pass after this fix.

Two fixture-only class-loading attempts failed before Core absence was established. The retained Equinox cache used the earlier fixture manifest; enabling `osgi.checkConfiguration=true` only for this launch, after preserving its configuration cache, allowed the new imports to resolve. No reset/clean or production cache change was used. All five authentication-scene JVMs and its proxy are stopped; its persisted reference, keys, CM, owned Driver and journals remain. The earlier normal scene's two JVMs/proxy are also stopped.

Still pending: broader providers/plan shapes, dedicated UPDATE/channel/Wires/cloud/import/clone receipts and full install/upgrade/downgrade behavior. Native browser checks use Chinese UI and selected identities; do not generalize them to every role, provider, locale or hardware. External NM/cloud-init/physical, amd64 and disk-full/hard-power validation remain separate deferred groups. No historical UNKNOWN was resent and no old one-time acceptance scene was replayed.
