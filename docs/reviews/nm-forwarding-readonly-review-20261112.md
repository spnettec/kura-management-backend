# NM forwarding admission and read-only views — 2026-11-12

Source applied in `target/nm-forwarding-readonly-20261112/`: nine selected files,
eight existing and one new browser spec. This repairs the IPv4 forwarding subset
of R013 and the complete read-only admission path of R018. R013 remains open for
authoritative Kura roles and shared lifecycle; this is not deployment acceptance.

## Forwarding

Four original-source counterexamples fail and the no-NAT positive control passes:
disabled forwarding was accepted into a durable NAT candidate; a prepared NAT
could create a checkpoint after forwarding changed; confirmation could destroy
the checkpoint after forwarding dropped; the source lease did not change.

The coordinator now reads the fixed kernel IPv4 forwarding state, binds it on
both sides of joint source observation, and requires it for an effective NAT
candidate before intent/checkpoint/NAT dispatch and candidate confirmation.
It never enables forwarding or changes sysctls. Ordinary no-NAT transactions and
explicit NAT removal remain supported with forwarding off. Restoration keeps its
original-baseline semantics rather than changing the system owner's setting.
An external sysctl change is not made atomic by this check; checkpoint/readback
and UNKNOWN rules still apply. This does not prove authoritative WAN/LAN roles.

## Read-only path

The agent's remote view no longer requires a write engine. Existing remote grant,
command identity/payload and authenticated authority checks still apply. Reads
use the observer's bounded profile capture independently of nft. With a state
root, a shared observation lock and the retained transaction reader validate and
project original pending evidence without tokens or any NM/nft write. Corrupt
evidence is an error. Local/remote mutations remain unsupported without a writer.

Center receipt admission and the browser require an actual Boolean writeEnabled,
accept false for reads and reject contradictory confirmation readiness. Views
have a separate bounded token-free transaction shape: pre-checkpoint/other
non-WAIT_CONFIRM states may have no deadline, and the UI invents no countdown.
WAIT_CONFIRM still requires a positive deadline. The browser independently gates
all write controls and handlers on current write enablement, including rollback
with an older saved prepare lookup. Saved correlation does not grant permission.

## Evidence and limits

- Native agent337 pass/15 ignored; console NM receipt8 pass; strict all-target
  agent and console Clippy pass. Agent Clippy's compiled inputs are unchanged;
  an unrelated console before-source pin captured during that run is explicitly
  outside its compiled scope.
- Linux ARM64 NM96 pass/2 ignored, firewall32 pass/1 ignored, system-change2 pass,
  protocol18 pass, strict agent/protocol Clippy pass. One new case reads actual
  `/proc/sys/net/ipv4/ip_forward` and proves its bytes unchanged. No system bus
  is mounted and no NM/nft mutation is performed. Container exits0 and is retained.
- Five new mocked browser cases pass, including Chinese/English read-only display,
  saved-prepare disablement, missing pre-checkpoint deadline and malformed
  readiness. Chinese screenshot visually checked. React typecheck/production
  build passes for the exact final UI source. Unrelated transfer panels are not
  accepted by these mocks.
- `before-forwarding-v2`, `before-agent-readonly`, corrected
  `before-console-readonly-v2` and `browser-before-v2` establish the defects.
  Initial wrong package selection and missing SQL migration fixture are retained
  setup failures, not defect proof. Restoring the console candidate with preserved
  mtime caused Cargo to reuse the old artifact; `final-console-nm-v2` is rejected.
  Only the shadow file mtime was refreshed; `final-console-nm-v3` shows an actual
  recompile and eight passes. No original target was cleaned or changed.
- The original input seal has223 files, including22 SQL migrations needed by
  console test macros. Linux freezes202 earlier inputs; all compiled Linux
  sources match final source. Its only later shadow difference is the browser
  spec's default evidence directory, separately rerun as `browser-final-v2` with
  explicit source pins. That UI test is not a compiled Linux input.

Current active management overlay has20 pins.342 upstream pins,141 original
credential pins plus five explicit current overlays, ca9 eight artifacts and f07
remain protected; historical manifests are not rewritten.18 repository HEADs,
branches, staged states and tracked changes outside selected paths match the
preceding checkpoint. Original runtime stays healthy and all five owned proof
containers are stopped. No commit/push, old one-time scene replay or UNKNOWN resend.

Next: Kura CM-authoritative WAN/LAN/NAT roles and revision binding, durable Java
adoption/retirement and ordinary fields/multi-WAN; then real kernel/NM integration
and coherent release. R017 deployment/docs, TLS hot reload, bulk enrollment and
protocol publishing remain internal pending, alongside the complete14-group
ledger and seven API2 migrations. Original three external deferred groups and
Scheme A/external CA remain unchanged.
