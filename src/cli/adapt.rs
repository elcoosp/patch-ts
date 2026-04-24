use anyhow::Result;
pub fn handle_adapt_threshold() -> Result<()> {
    let threshold = crate::history_adaptive::suggest_threshold(std::path::Path::new(".patch‑ts"))?;
    println!("{:.2}", threshold);
    Ok(())
}
pub fn handle_adapt_strategy() -> Result<()> { println!("exact,anchor,anchor_pair,ellipsis,similarity,fuzzy"); Ok(()) }
