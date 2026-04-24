use anyhow::Result;
use crate::diagnostics::{JsonDiagnostic, JsonError};
use super::types::*;
pub fn handle_explain(args: ExplainArgs) -> Result<()> {
    let file_path = std::path::Path::new(&args.file);
    let mut lang = crate::ast::detect_language(file_path)?;
    let diag = crate::repair::explain_error(file_path, args.line, args.json, &mut *lang)?;
    if let Some(diag) = diag {
        if args.json {
            let json_err = JsonError {
                code: "patch_ts::syntax_error".to_string(), message: diag.details.clone(),
                span: crate::diagnostics::JsonSpan { file: args.file.clone(), line: args.line, column: 1 },
                context: String::new(), suggestion: Some("Run `patch-ts balance` to attempt automatic fix".to_string()),
                best_score: None, best_match_line: None, candidates: None, error_code: Some("E005".to_string()),
                retry_prompt: Some(diag.details.clone())
            };
            println!("{}", serde_json::to_string(&JsonDiagnostic::error(json_err))?);
        } else { eprintln!("{:?}", miette::Report::new(diag)); }
    } else if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); }
    Ok(())
}
