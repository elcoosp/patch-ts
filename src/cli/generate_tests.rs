use anyhow::Result;
use super::types::*;
pub fn handle_generate_tests(args: GenerateTestsArgs) -> Result<()> {
    let tests = crate::curiosity::generate_tests(&args.old, &args.new, "function")?;
    if args.json { println!("{}", serde_json::to_string(&tests)?); }
    else { for test in &tests.tests { println!("{}", test); } }
    if let Some(ref out) = args.output { std::fs::write(out, tests.tests.join("\n"))?; }
    Ok(())
}
