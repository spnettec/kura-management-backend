# Credential mount and activation refactoring

This is a source checkpoint, not a deployed release or completion of the full
management redesign. Evidence: `target/credential-mount-20261112/`.

## Implemented

- R019: repository and desktop Compose definitions bind only `server.pem`,
  `server.key` and public `ca.pem`, all read-only, with missing-host-path creation
  disabled. The deployment bundle uses a prebuilt image rather than a nonexistent
  parent-directory source build.
- The root entrypoint copies only those bootstrap inputs into a fresh private
  runtime directory under a root-owned parent, then drops privileges. It never
  applies console-writable pending credentials. Desktop entrypoint bytes match.
- Client CA uploads validate parsed CA constraints, signing usage when present,
  validity and rustls trust-anchor decoding. Immutable versions and a single
  durable pointer are activated by the unprivileged server. A changed current
  trust anchor invalidates prior approval. Restart after pointer publication can
  finish without reissuing a CA update. Original files and old versions remain.
- Server/gateway identity forms reject CA certificates, preventing a CA key pair
  being imported accidentally as an ordinary identity. This is not a managed-CA
  signing implementation or a complete certificate-policy audit.
- Settings now accepts local certificate/key/CA files. Selection immediately
  invalidates confirmation; reads block submission. Revision binding prevents a
  stale asynchronous fingerprint from approving another file. Oversized files
  have an explicit bilingual message. Loading buttons retain stable accessible
  labels. Manual PEM entry remains available.
- Deployment docs and skills describe current boundaries, preserve data, and
  stop the locally running agent before identity import/rotation commands.

## Evidence and limits

Final source is `frozen-source-final/source-manifest.json`:146 exact input copies.
The earlier `frozen-source/` is an intermediate test snapshot and is not the final
seal. Before copies cover21 existing files; new public certificate fixtures have
separate pins. Generated private test keys remain only in private review artifacts.

- Native console:46 passed,32 ignored; three prior mTLS cases remain ignored.
- Final private Unix-socket PG HTTP contracts:17 passed. Each uses a fresh schema;
  all owned PG servers stopped and data/logs remain.
- Browser file contracts:5 passed,0 flaky, with all API paths mocked. They prove
  file transfer, explicit confirmation, bilingual controls, size bounds and stale
  fingerprint protection. Real parsing/pairing/admission is covered separately.
- Strict all-target Clippy, React typecheck and entrypoint shell syntax pass.
- Parsed Compose inventories show exactly three read-only bootstrap file binds.
- A fresh network-none Linux container verifies the production entrypoint with a
  stub console: UID999, private owned copies, unchanged pending data, read-only
  originals, no mounted CA/gateway private key. This is a filesystem/privilege
  contract, not acceptance of a new Rust Linux release. Container exited; retained.

Failed attempts remain: the initial extracted helper missed a MetadataExt import;
the first bind-mounted entrypoint lacked the executable bit supplied by Dockerfile
builds; browser assertions encountered duplicate alerts/loading-icon accessible
names. Final tests include explicit digest completion and render settling rather
than accepting a check that could run before the stale result arrived.

## Open work

New image/full Linux and deployment acceptance, legacy pending-settings UI review,
initial center setup, standalone agent local setup, UI maintenance activation and
organizational signing/recovery remain internal work. CA custody is under the
user-requested security reassessment; no CA signing key was imported or generated
for the application. The original14 groups and three external deferred remain.

The user requested logical upstream synchronization before further large changes.
This verified source seal is the boundary before that pass.18 HEAD/branches were
unchanged. GitHub ancestry confirms11 forks and7 nonfork repositories. Nonforks
skip direct parent merging but still require dependency and source-origin impact
analysis. See `upstream-logical-sync-20261112.md`.
