# UI provisioning and organizational CA custody review

This review responds to the user's latest direction: deployment and ongoing
configuration should be performed through UI controls rather than mandatory
environment variables or scripts. Certificate generation may use scripts behind
a UI, or a download/sign/upload workflow. CA custody is a security decision. The
user specifically raised private keys being retained by one person.

The earlier selection of Scheme A remains the implemented contract while this
custody decision is reassessed. This document recommends a direction; it does not
claim that managed signing, local agent setup UI or two-person controls exist.

## Separate the decisions

- UI operation is required for either signing mode. It is not a reason to place
  a root CA private key in the console.
- CA ownership belongs to the organization. Keeping a root key on one person's
  computer creates a continuity risk. Central storage alone does not establish
  shared organizational control either.
- Gateway key ownership is separate from CA ownership. Prefer local key/CSR
  creation: Kura's Java identity service for a Kura gateway; an explicit local
  setup UI for a standalone agent. A center-generated one-time key bundle remains
  a compatibility option, with a wider exposure boundary.

## A and B

| Decision | A: external signing, no CA private key at console | B: organizational managed signing |
| --- | --- | --- |
| Routine enrollment/renewal | UI requests/downloads CSR, external CA signs, UI uploads result | UI submits a CSR to a controlled signer and receives the certificate |
| Console compromise | Does not by itself disclose an independently held CA key; console identity/binding and stored gateway bundles can still be compromised | An attacker may misuse available signing authority; file encryption alone does not prevent this |
| Personnel continuity | Requires organizational CA custody and documented recovery; personal offline storage is insufficient | Can use organizational accounts, multiple custodians and recoverable encrypted backups |
| Operations | Extra external signing handoff unless an existing CA API/UI is integrated | Simpler routine operation and renewal after managed signer setup |
| Engineering | Complete the UI enrollment/upload/recovery path | Also implement signer policy, authorization, encrypted key lifecycle and recovery |

Recommended B variant: retain the root CA under independent organizational
custody; online signing uses a dedicated subordinate issuing CA restricted to
gateway client certificates. An existing organizational CA service can provide
this signer without exposing its private key to the console. A local managed
signer is an optional deployment profile. This is distinct from copying an
unrestricted root key into the web service's data directory.

Root CA compromise affects the hierarchy. An offline root and subordinate issuing
CAs reduce the root's online exposure; the issuing key must be available when
issuing certificates. See [Microsoft CA architecture](https://learn.microsoft.com/en-us/windows-server/identity/ad-cs/certification-authority-role#root-and-subordinate-certification-authorities).
Client-side CSR creation is supported by the enrollment model described in
[RFC7030, section4.2.1](https://www.rfc-editor.org/rfc/rfc7030.html#section-4.2.1).
The recommendation is an application design proposal, not a claim of EST support.

## Controls addressing a single custodian

1. Use named organizational accounts, with more than one authorized administrator
   and no shared personal account as the only recovery path.
2. Separate routine certificate issuance from CA enablement, replacement,
   decommissioning and key-backup recovery. High-impact CA changes require a
   second authorized person; routine approved gateway enrollment should remain
   a short UI flow.
3. Do not expose plaintext CA private-key download through ordinary management
   APIs. Provide an encrypted backup/export and a documented recovery exercise.
   Recovery material must not depend on one person's memory or workstation.
   A threshold recovery option such as two-of-three can address both sole-person
   control and one unavailable custodian; its actual implementation and validation
   are separate deliverables, not assumed complete.
4. A person controlling the host root account, console database, encrypted backups
   and decryption authority can bypass application-only approval controls. Strong
   protection against that administrator requires independent signer/key-service
   custody and separated operating privileges. HSM/KMS integration is an optional
   security profile, not a prerequisite imposed on every deployment.

## Required UI outcome, common to both modes

- First launch has a setup wizard for the center address, certificate/trust
  settings and organizational administrator setup. Runtime configuration is
  saved durably by the application; environment variables are advanced overrides.
- Gateway onboarding supports installed Kura's existing local GWT entry and a
  standalone agent setup entry. A first unregistered gateway cannot rely on an
  already authenticated remote channel for its initial identity installation.
- Generate/download a CSR, upload a signed chain or a one-time identity package,
  verify fingerprints, bind the gateway and activate the remote channel from UI.
- Remote enablement, allowed interfaces, capability switches and supported NM/
  firewall settings are UI operations backed by the same authority and journal.
- Certificate renewal, CA transitions, encrypted backup/restore and maintenance
  activation have explicit UI actions and durable result receipts. A mandatory
  `docker restart`, `setup.py` command or environment edit does not satisfy this.
- First software installation/initial process launch remains a deployment-platform
  step. An installer or container-platform UI must make it concrete; the not-yet-
  running center cannot serve its own first installer.

## Current implementation gaps and next action

The old v1 P1/P2/P3 labels do not establish full UI completion: standalone identity
import/CSR activation and remote/capability switches still require CLI or env
edits, and TLS activation lacks the requested complete UI maintenance flow.
These are internal implementation work, not external deferred validation.

R019 is independent of the final custody choice. Current Compose definitions
accidentally mount the entire demo TLS directory, including CA and gateway
private keys. Restrict bootstrap mounts and make client-CA activation work from
the private state directory. A future managed issuing key must enter through an
explicit managed-signer configuration with its own lifecycle, not that directory
mount. Preserve the current scene and all old evidence while implementing this.
