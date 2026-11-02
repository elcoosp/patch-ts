use anyhow::Result;
///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn handle_adapt_threshold() -> Result<()> {
    let threshold = crate::history_adaptive::suggest_threshold(std::path::Path::new(".patch‑ts"))?;
    println!("{threshold:.2}");
    Ok(())
}
///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn handle_adapt_strategy() -> Result<()> {
    println!("exact,anchor,anchor_pair,ellipsis,similarity,fuzzy");
    Ok(())
}
