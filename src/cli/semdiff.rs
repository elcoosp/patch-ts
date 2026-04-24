use anyhow::Result;
use crate::semdiff::ChangeType;
use super::types::*;
pub fn handle_sem_diff(args: SemDiffArgs) -> Result<()> {
    let old_content = std::fs::read_to_string(&args.old)?;
    let new_content = std::fs::read_to_string(&args.new)?;
    let lang = if let Some(ref file) = args.file {
        std::path::Path::new(file).extension().and_then(|e| e.to_str()).unwrap_or("rs")
    } else { std::path::Path::new(&args.old).extension().and_then(|e| e.to_str()).unwrap_or("rs") };
    let old_entities = crate::semdiff::extract_entities(&old_content, lang).map_err(|e| anyhow::anyhow!("{}", e))?;
    let new_entities = crate::semdiff::extract_entities(&new_content, lang).map_err(|e| anyhow::anyhow!("{}", e))?;
    let changes = crate::semdiff::diff_entities(&old_entities, &new_entities);
    if args.json { println!("{}", serde_json::to_string(&changes)?); }
    else { for change in &changes { let symbol = match change.change_type { ChangeType::Added => "⊕", ChangeType::Removed => "⊖", ChangeType::Modified => "∆", ChangeType::Moved => "⇢" }; println!("{} {} {}", symbol, change.entity.kind, change.entity.name); } }
    Ok(())
}
