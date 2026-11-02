use anyhow::Result;
///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn handle_lsp() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(crate::lsp::run_lsp())?;
    Ok(())
}
