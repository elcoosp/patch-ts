use anyhow::Result;
use chrono::Utc;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScittRecord {
    pub content_hash: String,
    pub timestamp: String,
    pub previous_hash: String,
    pub tool: String,
    pub agent: Option<String>,
    pub model: Option<String>,
    pub signature: Option<String>,
}

/// Generate a new Ed25519 keypair using OS randomness.
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Sign a message and return the signature bytes as hex.
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> String {
    let signature: Signature = signing_key.sign(message);
    hex::encode(signature.to_bytes())
}

/// Compute SHA‑256 hash of data.
pub fn hash_content(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

/// Emit a SCITT‑compliant provenance record, maintaining a hash chain.
pub fn emit_scitt_record(
    log_path: &Path,
    content: &str,
    agent: Option<&str>,
    model: Option<&str>,
    signing_key: Option<&SigningKey>,
) -> Result<ScittRecord> {
    let content_hash = hash_content(content);
    let timestamp = Utc::now().to_rfc3339();

    // Read the last record's hash to form the chain
    let previous_hash = if let Ok(prev_content) = fs::read_to_string(log_path) {
        prev_content
            .lines()
            .last()
            .and_then(|line| serde_json::from_str::<ScittRecord>(line).ok())
            .map(|r| r.content_hash)
            .unwrap_or_else(|| "genesis".to_string())
    } else {
        "genesis".to_string()
    };

    let mut record = ScittRecord {
        content_hash: content_hash.clone(),
        timestamp: timestamp.clone(),
        previous_hash,
        tool: "patch-ts".to_string(),
        agent: agent.map(|s| s.to_string()),
        model: model.map(|s| s.to_string()),
        signature: None,
    };

    // Optionally sign the record
    if let Some(key) = signing_key {
        let message = format!("{}:{}:{}", content_hash, timestamp, record.previous_hash);
        record.signature = Some(sign_message(key, message.as_bytes()));
    }

    // Append JSON line
    let mut log_content = fs::read_to_string(log_path).unwrap_or_default();
    log_content.push_str(&serde_json::to_string(&record)?);
    log_content.push('\n');
    fs::write(log_path, log_content)?;

    Ok(record)
}

/// Verify the integrity of the SCITT log.
/// Returns a list of tampered records (timestamps of records that failed verification).
pub fn verify_log(log_path: &Path, verifying_key: Option<&VerifyingKey>) -> Result<Vec<String>> {
    let content = fs::read_to_string(log_path).unwrap_or_default();
    let mut tampered = Vec::new();
    let mut prev_hash: Option<String> = None;

    for line in content.lines() {
        if let Ok(record) = serde_json::from_str::<ScittRecord>(line) {
            // Check hash chain
            if let Some(ref expected_prev) = prev_hash {
                if record.previous_hash != *expected_prev {
                    tampered.push(record.timestamp.clone());
                }
            }

            // Verify signature if key provided
            if let Some(key) = verifying_key {
                if let Some(ref sig_hex) = record.signature {
                    let message = format!("{}:{}:{}", record.content_hash, record.timestamp, record.previous_hash);
                    if let Ok(sig_bytes) = hex::decode(sig_hex) {
                        let signature = Signature::from_slice(&sig_bytes)?;
                        if key.verify(message.as_bytes(), &signature).is_err() {
                            tampered.push(record.timestamp.clone());
                        }
                    }
                }
            }

            prev_hash = Some(record.content_hash.clone());
        }
    }

    Ok(tampered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_emit_and_verify() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("provenance.jsonl");
        let (signing_key, verifying_key) = generate_keypair();

        // Emit two records
        let r1 = emit_scitt_record(&log_path, "patch content 1", Some("Claude Code"), Some("sonnet"), Some(&signing_key)).unwrap();
        let r2 = emit_scitt_record(&log_path, "patch content 2", Some("Claude Code"), Some("sonnet"), Some(&signing_key)).unwrap();

        assert_eq!(r1.previous_hash, "genesis");
        assert_eq!(r2.previous_hash, r1.content_hash);

        // Verify intact log
        let tampered = verify_log(&log_path, Some(&verifying_key)).unwrap();
        assert!(tampered.is_empty());
    }

    #[test]
    fn test_detect_tampered() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("provenance.jsonl");
        let (signing_key, _) = generate_keypair();

        emit_scitt_record(&log_path, "patch content 1", None, None, Some(&signing_key)).unwrap();

        // Tamper with the log
        let mut content = fs::read_to_string(&log_path).unwrap();
        content = content.replace("patch content 1", "TAMPERED");
        fs::write(&log_path, content).unwrap();

        let tampered = verify_log(&log_path, None).unwrap();
        assert!(!tampered.is_empty());
    }
}
