use anyhow::Result;
use super::types::*;
use crate::scorecard::render_scorecard;
pub fn handle_score(args: ScoreArgs) -> Result<()> {
    let content = std::fs::read_to_string(&args.file)?;
    let syntax_valid = { let mut lang = crate::ast::detect_language(std::path::Path::new(&args.file))?; let parse_result = lang.parse(&content); lang.is_valid(&parse_result) };
    let compile_success = { let result = crate::compile::compile_check(std::path::Path::new(&args.file), "rs", 30)?; result.success };
    let confidence = args.confidence.unwrap_or(0.9);
    let uniqueness_score = args.uniqueness_score.unwrap_or(0.5);
    let cross_file_impact = args.cross_file_impact.unwrap_or(0);
    let historical_success_rate = crate::score::historical_success_rate();
    let ctx = crate::score::ScoreContext { syntax_valid, compile_success, confidence, uniqueness_score, cross_file_impact, historical_success_rate };
    let score = crate::score::calculate_score(&ctx);
    if args.json { println!("{}", serde_json::to_string(&score)?); }
    else { println!("{}", render_scorecard(&score)); }
    Ok(())
}
