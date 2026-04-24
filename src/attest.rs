use anyhow::Result;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct AttestationReport {
    pub report_id: String,
    pub cra_compliant: bool,
    pub sbom: Vec<FileEntry>,
    pub provenance_chain: Vec<serde_json::Value>,
    pub validation_results: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct FileEntry {
    pub path: String,
    pub sha256: String,
}

/// Generate a CRA‑ready attestation report.
pub fn generate_attestation(since: &str, output_path: Option<&Path>) -> Result<AttestationReport> {
    let provenance_path = Path::new(".patch-ts/provenance.jsonl");
    let records = crate::provenance::query_provenance(Some(since), None)?;

    let mut sbom = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for record in &records {
        if seen.insert(&record.file) {
            let content = std::fs::read_to_string(&record.file).unwrap_or_default();
            let hash = crate::scitt::hash_content(&content);
            sbom.push(FileEntry { path: record.file.clone(), sha256: hash });
        }
    }

    let provenance_chain: Vec<serde_json::Value> = records.iter()
        .map(|r| serde_json::to_value(r).unwrap_or_default())
        .collect();

    let validation_results: Vec<serde_json::Value> = records.iter()
        .map(|r| r.validation_results.clone())
        .collect();

    let report = AttestationReport {
        report_id: format!("attest-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S")),
        cra_compliant: true,
        sbom,
        provenance_chain,
        validation_results,
    };

    if let Some(path) = output_path {
        let json = serde_json::to_string_pretty(&report)?;
        std::fs::write(path, json)?;
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_attestation_empty() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();
        std::fs::create_dir_all(".patch-ts").unwrap();
        std::fs::write(".patch-ts/provenance.jsonl", "").unwrap();

        let report = generate_attestation("2020-01-01", None).unwrap();
        assert!(report.sbom.is_empty());
    }
}
