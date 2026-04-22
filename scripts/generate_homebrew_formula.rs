//! Generate a Homebrew formula for patch-ts.
//! Run with: cargo run --example generate_homebrew_formula

use formulaic::Formula;
use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let version = if let Ok(tag) = env::var("GITHUB_REF") {
        tag.trim_start_matches("refs/tags/v").to_string()
    } else {
        "1.0.0".to_string()
    };

    let url = format!(
        "https://github.com/elcoosp/patch-ts/archive/refs/tags/v{}.tar.gz",
        version
    );

    let sha256 = if let Ok(output) = Command::new("shasum")
        .arg("-a")
        .arg("256")
        .arg(format!("v{}.tar.gz", version))
        .output()
    {
        let sum = String::from_utf8_lossy(&output.stdout);
        sum.split_whitespace().next().unwrap_or("").to_string()
    } else {
        "REPLACE_WITH_SHA256".to_string()
    };

    let formula = Formula::new("patch-ts")
        .description("Tree-sitter-backed universal patching CLI for AI agents")
        .homepage("https://github.com/elcoosp/patch-ts")
        .url(&url)
        .sha256(&sha256)
        .depends_on("rust")
        .bin("patch-ts")
        .build();

    fs::write("patch-ts.rb", formula.to_string()).unwrap();
    println!("Generated patch-ts.rb");
}
