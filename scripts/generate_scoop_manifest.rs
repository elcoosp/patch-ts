//! Generate a Scoop manifest for patch-ts.
//! Run with: cargo run --example generate_scoop_manifest

use serde_json::json;
use std::env;
use std::fs;

fn main() {
    let version = if let Ok(tag) = env::var("GITHUB_REF") {
        tag.trim_start_matches("refs/tags/v").to_string()
    } else {
        "1.0.0".to_string()
    };

    let url64 = format!(
        "https://github.com/elcoosp/patch-ts/releases/download/v{}/patch-ts-x86_64-pc-windows-msvc.zip",
        version
    );
    let url32 = format!(
        "https://github.com/elcoosp/patch-ts/releases/download/v{}/patch-ts-i686-pc-windows-msvc.zip",
        version
    );

    let manifest = json!({
        "version": version,
        "description": "Tree-sitter-backed universal patching CLI for AI agents",
        "homepage": "https://github.com/elcoosp/patch-ts",
        "license": "MIT",
        "architecture": {
            "64bit": {
                "url": url64,
                "hash": "REPLACE_WITH_SHA256_64",
                "bin": "patch-ts.exe"
            },
            "32bit": {
                "url": url32,
                "hash": "REPLACE_WITH_SHA256_32",
                "bin": "patch-ts.exe"
            }
        }
    });

    fs::write(
        "patch-ts.json",
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();
    println!("Generated patch-ts.json");
}
