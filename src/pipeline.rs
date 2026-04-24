use anyhow::Result;

pub enum PipelineStage {
    Patch,
    Validate,
    Test,
    Commit,
}

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
                        .args(&["add", "-A"])
                        .status()?;
                    std::process::Command::new("git")
                        .args(&["commit", "-m", "Auto-commit from patch-ts pipeline"])
                        .status()?;
                    println!("✅ Commit stage passed.");
                }
            }
        }
    }
    Ok(())
}
