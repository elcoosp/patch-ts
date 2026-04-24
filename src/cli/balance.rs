use anyhow::Result;
use std::path::Path;
use super::types::*;
use super::patch::expand_files;
use rayon::prelude::*;

pub fn apply_balance_to_file(file_path: &Path, args: &BalanceArgs) -> Result<()> {
    let mut lang = crate::ast::detect_language(file_path)?;
    let result = crate::repair::balance_file(file_path, args.function.as_deref(), !args.apply, &mut *lang, args.plugin.as_deref(), args.max_cost)?;
    if args.json { println!("{}", serde_json::to_string(&result)?); }
    Ok(())
}
pub fn handle_balance(args: BalanceArgs) -> Result<()> {
    if let Some(pattern) = &args.files {
        let paths = expand_files(pattern)?;
        if args.serial {
            for path in paths { apply_balance_to_file(&path, &args)?; }
        } else {
            paths.par_iter().try_for_each(|path| apply_balance_to_file(path, &args))?;
        }
    } else {
        let file_path = Path::new(args.file.as_deref().unwrap());
        apply_balance_to_file(file_path, &args)?;
    }
    Ok(())
}
