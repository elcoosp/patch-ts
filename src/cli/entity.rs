use anyhow::Result;
use super::types::*;

pub fn handle_entity(args: EntityArgs) -> Result<()> {
    match args.action {
        EntityAction::List { file, json } => {
            let content = std::fs::read_to_string(&file)?;
            let mut lang = crate::ast::detect_language(std::path::Path::new(&file))?;
            let parse_result = lang.parse(&content);
            let entities = lang.find_all_entities(&parse_result);
            if json { println!("{}", serde_json::to_string_pretty(&entities)?); }
            else { for entity in &entities { println!("{} ({}) – bytes {}..{}", entity.name, entity.kind, entity.start_byte, entity.end_byte); } }
        }
        EntityAction::Show { symbol, file } => {
            let content = std::fs::read_to_string(&file)?;
            let mut lang = crate::ast::detect_language(std::path::Path::new(&file))?;
            let parse_result = lang.parse(&content);
            if let Some((start, end)) = lang.find_symbol_node(&parse_result, &symbol) { println!("{}", &content[start..end]); }
            else { anyhow::bail!("Symbol '{}' not found", symbol); }
        }
        EntityAction::Replace { symbol, file, new } => {
            let mut lang = crate::ast::detect_language(std::path::Path::new(&file))?;
            let options = crate::patch::PatchOptions::default();
            crate::patch::apply_symbol_patch(std::path::Path::new(&file), &symbol, &new, &options, &mut *lang)?;
            println!("Entity '{}' replaced.", symbol);
        }
        EntityAction::Body { symbol, file, new } => {
            let content = std::fs::read_to_string(&file)?;
            let mut lang = crate::ast::detect_language(std::path::Path::new(&file))?;
            let parse_result = lang.parse(&content);
            if let Some((start, end)) = lang.find_symbol_node(&parse_result, &symbol) {
                let body_start = content[start..end].find('{').map(|p| start + p + 1).unwrap_or(start);
                let body_end = content[..end].rfind('}').unwrap_or(end);
                let mut patched = content.clone();
                patched.replace_range(body_start..body_end, &new);
                std::fs::write(&file, patched)?;
                println!("Body of '{}' replaced.", symbol);
            } else { anyhow::bail!("Symbol '{}' not found", symbol); }
        }
    }
    Ok(())
}
