use anyhow::Result;
///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn handle_undo() -> Result<()> {
    let manager = crate::history::HistoryManager::new();
    match manager.undo_last()? {
        Some(record) => {
            std::fs::write(&record.file, &record.original_content)?;
            println!("Undo applied: file {} restored.", record.file);
        }
        None => {
            eprintln!("No history to undo.");
        }
    }
    Ok(())
}
///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn handle_redo() -> Result<()> {
    eprintln!("Redo not yet implemented.");
    Ok(())
}
///
/// # Errors
///
/// Returns an error if the file or record cannot be read.
pub fn handle_history() -> Result<()> {
    let manager = crate::history::HistoryManager::new();
    let records = manager.list()?;
    if records.is_empty() {
        println!("No history found.");
    } else {
        for record in &records {
            println!("{} - {}", record.timestamp, record.file);
        }
    }
    Ok(())
}
