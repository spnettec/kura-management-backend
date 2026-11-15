# NM fresh connection and CSR output review — 2026-11-12

R015/R016 have scoped source repairs. Evidence lives under
`target/nm-transport-csr-20261112/`; this does not close release deployment or the
original management ledger. Scheme A with external CA issuance remains selected.

## Behavior

R015: the control worker now includes NM_PREPARE among the preparation operations
that request a fresh management connection after delivering WAIT_CONFIRM. The
ordinary reconnect loop can therefore establish a new locally minted connection
witness even when applying the candidate did not interrupt the network. Existing
network/takeover/firewall preparation handling is retained. Read-only and UNKNOWN
responses do not trigger this path. No UNKNOWN command is dispatched again.

R016: CSR export pins the selected output directory before generating the key,
checks its identity again, and creates the actual output through that directory
descriptor with O_EXCL/O_NOFOLLOW. Both writing and chmod use the newly created
file descriptor. A concurrent regular file, symlink or hardlink cannot be
overwritten or chmodded. A replaced parent directory is refused. Failed export
retains the local pending key, CSR and record, rather than generating a second
identity. The key remains private and never becomes part of the exported CSR.

## Validation

- The original transport source fails a new real local mTLS contract after the
  server has received the exact cached NM WAIT_CONFIRM receipt. The final source
  passes: three authenticated TLS connections, three cached receipts delivered,
  NM WAIT_CONFIRM closes its old stream, while the read-only and UNKNOWN controls
  keep theirs open. The SQLite execution table remains empty. Thus this tests the
  real connection worker without performing any NM/system operation. The first
  fixture attempt did not explicitly record receipt delivery before its timeout;
  the retained v2 baseline adds that proof and is authoritative.
- Transport unit tests: 7 pass, 4 ignored, plus the explicitly executed new mTLS
  contract above. Strict all-target native Clippy passes. This is not replay of
  an earlier accepted mTLS/firewall scene.
- Four new CSR race contracts fail before the repair and pass afterward. The
  normal CSR/external-signing/activation control passes on both sources.
- Native packaging suite: 84 run, 2 skip, no failures (82 pass). The first full
  run lacked the copied core preinst/postinst text fixtures; those three errors
  and the corrected run remain retained. Only those two source fixtures were
  copied into the isolated workspace; no package lifecycle script was executed.
- A separate new Linux ARM64 container, without network or system-bus access,
  passes the same four races and normal CSR activation control. Its read-only
  mount contains only the bounded source workspace. It exited successfully and
  remains stopped; original containers and VMs were not restarted.

These checks establish the transport dispatch boundary and file creation contract,
not the complete real-NM remote lifecycle. The closed private-GUID proof and the
previous312-test recovery result are retained without replay. A fresh coherent
release/deployment remains internal pending. External deferred remain exactly
real NM/cloud-init/physical hardware, amd64, and disk-full/hard-power.

The final source manifest and preservation audit record the four applied files,
the preceding six NM source pins, 18 repository HEAD/branches/staged state, tracked
changes restricted to three selected agent files, and all protected upstream,
credential, ca9 and f07 seals. No commit or push occurred.
