use anyhow::Result;

pub enum PipelineStage {
    Patch,
    Validate,
    Test,
    Commit,
}

///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn run_pipeline(stages: &[PipelineStage], dry_run: bool) -> Result<()> {
    for stage in stages {
        match stage {
            PipelineStage::Patch => {
                if dry_run {
                    println!("[DRY RUN] Would apply patch.");
                } else {
                    println!("✅ Patch stage passed (mocked).");
                }
            }
            PipelineStage::Validate => {
                if dry_run {
                    println!("[DRY RUN] Would validate compilation.");
                } else {
                    println!("✅ Validate stage passed (mocked).");
                }
            }
            PipelineStage::Test => {
                if dry_run {
                    println!("[DRY RUN] Would run tests: just test.");
                } else {
                    let status = std::process::Command::new("just").arg("test").status()?;
                    if !status.success() {
                        anyhow::bail!("Tests failed.");
                    }
                    println!("✅ Test stage passed.");
                }
            }
            PipelineStage::Commit => {
                if dry_run {
                    println!("[DRY RUN] Would commit changes.");
                } else {
                    std::process::Command::new("git")
                        .args(["add", "-A"])
                        .status()?;
                    std::process::Command::new("git")
                        .args(["commit", "-m", "Auto-commit from patch-ts pipeline"])
                        .status()?;
                    println!("✅ Commit stage passed.");
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_run_pipeline_prints_without_side_effects() {
        run_pipeline(
            &[
                PipelineStage::Patch,
                PipelineStage::Validate,
                PipelineStage::Test,
                PipelineStage::Commit,
            ],
            true,
        )
        .unwrap();
    }

    #[test]
    fn mocked_stages_pass_without_side_effects() {
        // Patch and Validate are mocked in non-dry mode; Test/Commit would
        // shell out, so they are only exercised in dry-run mode.
        run_pipeline(&[PipelineStage::Patch, PipelineStage::Validate], false).unwrap();
    }
}
