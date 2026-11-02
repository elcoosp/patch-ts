use super::types::GenerateTestsArgs;
use anyhow::Result;
///
/// # Errors
///
/// Returns an error if the content cannot be generated from the input.
pub fn handle_generate_tests(args: &GenerateTestsArgs) -> Result<()> {
    let tests = crate::curiosity::generate_tests(&args.old, &args.new, "function")?;
    if args.json {
        println!("{}", serde_json::to_string(&tests)?);
    } else {
        for test in &tests.tests {
            println!("{test}");
        }
    }
    if let Some(ref out) = args.output {
        std::fs::write(out, tests.tests.join("\n"))?;
    }
    Ok(())
}
