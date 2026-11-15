//! Scheme-A provisioning: the console never signs or generates key material.
//! It stores an administrator-uploaded client private key only long enough to
//! hand it back once inside a one-time identity bundle; the CA private key
//! never reaches the console.
use anyhow::{Result, bail, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use rand_core::{OsRng, RngCore};

/// Reject anything that is not exactly one PEM private key block (PKCS#8,
/// SEC1 or RSA) and return it unchanged; the console performs no key parsing
/// beyond structure, and cert/key pairing is verified at import time by the
/// agent.
pub fn validate_private_key_pem(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty() && value.len() <= 32768,
        "private key size out of bounds"
    );
    let trimmed = value.trim();
    let (begin, end) = match (
        trimmed.split_once('\n').map(|(head, _)| head.trim()),
        trimmed.rsplit_once('\n').map(|(_, tail)| tail.trim()),
    ) {
        (Some(head), Some(tail)) => (head, tail),
        _ => bail!("private key must be a PEM block"),
    };
    ensure!(
        (begin.starts_with("-----BEGIN ") && begin.ends_with(" PRIVATE KEY-----"))
            && (end.starts_with("-----END ") && end.ends_with(" PRIVATE KEY-----")),
        "private key must be a PEM PRIVATE KEY block"
    );
    let encoded: String = trimmed
        .lines()
        .skip(1)
        .take_while(|line| !line.starts_with("-----END "))
        .collect::<Vec<_>>()
        .join("");
    ensure!(
        !encoded.is_empty() && encoded.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=')),
        "private key body is not base64"
    );
    STANDARD.decode(&encoded)?;
    ensure!(
        STANDARD.encode(&STANDARD.decode(&encoded)?) == encoded,
        "noncanonical private key encoding"
    );
    Ok(())
}

pub fn bundle_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn token_matches(stored_hash: &str, provided: &str) -> bool {
    provided.len() == 64
        && provided.chars().all(|c| c.is_ascii_hexdigit())
        && crate::auth::hash(provided.as_bytes()) == stored_hash
}

/// One uncompressd ustar archive; members are fixed, so no path or size input
/// can smuggle traversal entries into the tar stream. `kind` selects the
/// manifest: an identity bundle binds endpoint+generation for import-bundle, a
/// rotation bundle carries the previous fingerprint for rotate-bundle.
pub fn build_identity_bundle(
    gateway: &str,
    kind: BundleKind,
    endpoint: &str,
    ca_pem: &str,
    cert_pem: &str,
    key_pem: &str,
    previous_certificate_sha256: Option<&str>,
) -> Result<Vec<u8>> {
    let mut manifest = format!(
        "{{\"schemaVersion\":1,\"gatewayId\":{},\"purpose\":\"{}\",\"caFingerprint\":{},\"certificateFingerprint\":{},\"bindingGeneration\":1",
        json_string(gateway),
        match kind {
            BundleKind::Identity => "identity",
            BundleKind::Rotation => "rotation",
        },
        json_string(&certificate_fingerprint(ca_pem)?),
        json_string(&certificate_fingerprint(cert_pem)?),
    );
    match kind {
        BundleKind::Identity => {
            manifest.push_str(&format!(",\"endpoint\":{}", json_string(endpoint)));
        }
        BundleKind::Rotation => {
            let previous = previous_certificate_sha256
                .filter(|value| value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit()))
                .ok_or_else(|| anyhow::anyhow!("rotation bundle needs the previous fingerprint"))?;
            manifest.push_str(&format!(",\"previousCertificateSha256\":{}", json_string(previous)));
        }
    }
    manifest.push_str("}\n");
    let members = [
        ("manifest.json", manifest.as_bytes()),
        ("ca.pem", ca_pem.as_bytes()),
        ("agent.pem", cert_pem.as_bytes()),
        ("agent.key", key_pem.as_bytes()),
    ];
    let mut archive: Vec<u8> = Vec::new();
    for (name, content) in members {
        archive.extend_from_slice(&ustar_header(name, content.len())?);
        archive.extend_from_slice(content);
        let padding = (512 - content.len() % 512) % 512;
        archive.extend(std::iter::repeat_n(0u8, padding));
    }
    archive.extend_from_slice(&[0u8; 1024]);
    Ok(archive)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BundleKind {
    Identity,
    Rotation,
}

fn certificate_fingerprint(pem: &str) -> Result<String> {
    let trimmed = pem.trim();
    let encoded = trimmed
        .strip_prefix("-----BEGIN CERTIFICATE-----")
        .and_then(|v| v.strip_suffix("-----END CERTIFICATE-----"))
        .ok_or_else(|| anyhow::anyhow!("certificate PEM required"))?;
    let encoded: String = encoded.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let der = STANDARD.decode(&encoded)?;
    Ok(crate::auth::hash(&der))
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn ustar_header(name: &str, size: usize) -> Result<[u8; 512]> {
    ensure!(
        name.len() <= 100 && size <= 1024 * 1024,
        "bundle member out of bounds"
    );
    let mut header = [0u8; 512];
    header[..name.len()].copy_from_slice(name.as_bytes());
    header[100..108].copy_from_slice(b"0000644\0");
    header[108..116].copy_from_slice(b"0000000\0");
    header[116..124].copy_from_slice(b"0000000\0");
    header[124..136].copy_from_slice(&octal(size, 11)?);
    header[136..148].copy_from_slice(&octal(0, 11)?);
    header[156] = b'0';
    header[257..262].copy_from_slice(b"ustar");
    header[263..265].copy_from_slice(b"00");
    header[148..156].fill(b' ');
    let checksum: u32 = header.iter().map(|b| u32::from(*b)).sum();
    // GNU checksum layout: 6 octal digits, NUL, space.
    let digits = format!("{:06o}", checksum);
    ensure!(digits.len() == 6, "ustar checksum overflow");
    header[148..154].copy_from_slice(digits.as_bytes());
    header[154] = 0;
    header[155] = b' ';
    Ok(header)
}

fn octal(value: usize, digits: usize) -> Result<Vec<u8>> {
    let text = format!("{:0digits$o}", value, digits = digits);
    ensure!(text.len() == digits, "ustar octal field overflow");
    let mut out = text.into_bytes();
    out.push(0);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_key_structure_only() {
        let generated = "-----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgmLoJkPp1MmH9N43Q\nHLw+pWYAD9AXPa6QHopXWMlan3qhRANCAAS3w45DagOxnmgVqmoHrfkbv3yGCfFo\nrV/RBQTiPF0QndRC6seZei8QkRv3LnOWO1wb2FZMefOx6MuIqURFOxjp\n-----END PRIVATE KEY-----\n";
        assert!(validate_private_key_pem(generated).is_ok());
        // Any non-ASCII or non-base64 body byte must be refused outright.
        let corrupted = generated.replace("MmH9N43Q", "MmH9N43四");
        assert!(validate_private_key_pem(&corrupted).is_err());
        let garbage = "-----BEGIN PRIVATE KEY-----\nAAAA*bad\n-----END PRIVATE KEY-----\n";
        assert!(validate_private_key_pem(garbage).is_err());
        assert!(validate_private_key_pem("").is_err());
        assert!(validate_private_key_pem("-----BEGIN CERTIFICATE-----\nAAAA\n-----END CERTIFICATE-----").is_err());
        assert!(validate_private_key_pem("-----BEGIN PRIVATE KEY-----\nAAAA*bad\n-----END PRIVATE KEY-----").is_err());
    }

    #[test]
    fn token_is_random_and_hashed_comparison() {
        let first = bundle_token();
        let second = bundle_token();
        assert_ne!(first, second);
        assert_eq!(first.len(), 64);
        assert!(token_matches(&crate::auth::hash(first.as_bytes()), &first));
        assert!(!token_matches(&crate::auth::hash(first.as_bytes()), "0".repeat(64).as_str()));
        assert!(!token_matches(&crate::auth::hash(first.as_bytes()), "short"));
    }

    #[test]
    fn bundle_members_match_manifest_and_tar_round_trips() {
        let cert = include_str!("../tests/fixtures/registration-client.pem");
        let key = "-----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQKnYUnT+mMAi6mg\nPwX7sVkA/rtfBxyaAiRlpk9XcDe4AKgCgYIKoZIzj0DAQc=\n-----END PRIVATE KEY-----\n";
        let bundle = build_identity_bundle("gw-001", BundleKind::Identity, "https://center:18444", cert, cert, key, None)
            .expect("bundle builds");
        assert_eq!(bundle.len() % 512, 0);
        let header = &bundle[..512];
        assert_eq!(&header[..10], b"manifest.j");
        assert_eq!(header[257..262], *b"ustar");
        let text = String::from_utf8(bundle).unwrap();
        assert!(text.contains("\"schemaVersion\":1"));
        assert!(text.contains("\"gatewayId\":\"gw-001\""));
        assert!(text.contains("BEGIN PRIVATE KEY"));
        // A quote inside the gateway id must not break the manifest JSON.
        let bundle = build_identity_bundle("gw\"x", BundleKind::Identity, "https://center:18444", cert, cert, key, None).unwrap();
        let manifest_start = 512;
        let manifest_end = manifest_start
            + usize::from_str_radix(
                String::from_utf8_lossy(&bundle[124..136]).trim_matches(|c| c == '0' || c == ' ' || c == '\0'),
                8,
            )
            .unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(&bundle[manifest_start..manifest_end]).unwrap();
        assert_eq!(manifest["gatewayId"], "gw\"x");
        assert_eq!(manifest["certificateFingerprint"], manifest["caFingerprint"]);
    }
}
