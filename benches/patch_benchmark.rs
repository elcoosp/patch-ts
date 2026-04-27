use criterion::{black_box, criterion_group, criterion_main, Criterion};
use patch_ts::ast::{Language, RustLanguage};
use patch_ts::patch::{apply_literal_patch, PatchOptions};
use std::fs;
use tempfile::tempdir;

fn bench_patch_large_file(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("large.rs");
    let content = "fn main() {\n".to_string() + &"    println!(\"line\");\n".repeat(2000) + "}\n";
    fs::write(&file_path, &content).unwrap();

    let mut lang = RustLanguage::new();
    let options = PatchOptions::default();

    c.bench_function("patch 2000-line file", |b| {
        b.iter(|| {
            let result = apply_literal_patch(
                black_box(&file_path),
                black_box(1000),
                black_box("    println!(\"line\");"),
                black_box("    println!(\"patched\");"),
                black_box(options.clone()),
                black_box(&mut lang),
            );
            black_box(result).unwrap();
        })
    });
}

criterion_group!(benches, bench_patch_large_file);
criterion_main!(benches);

fn bench_preserve_comments_large_file(c: &mut Criterion) {
    use patch_ts::ast::RustLanguage;
    use patch_ts::comment_preserve::preserve_comments;

    let mut lang = RustLanguage::new();
    let mut content = String::new();
    content.push_str("// File header\n");
    for i in 0..50 {
        content.push_str(&format!("/// Doc for function {}\n", i));
        content.push_str(&format!("fn func_{}() {{\n", i));
        for j in 0..38 {
            content.push_str(&format!("    let x_{}_{} = {};\n", i, j, i * j));
        }
        content.push_str(&format!("}} // end func_{}\n", i));
    }
    let mut patched = String::new();
    for i in 0..50 {
        patched.push_str(&format!("fn func_{}() {{\n", i));
        for j in 0..38 {
            patched.push_str(&format!("    let x_{}_{} = {};\n", i, j, i * j + 1));
        }
        patched.push_str("}\n");
    }

    c.bench_function("preserve comments 2000-line file", |b| {
        b.iter(|| {
            let result = preserve_comments(&content, &patched, &mut lang);
            black_box(result).unwrap();
        })
    });
}

// Add this function to the criterion_group! line as well.
// Example: criterion_group!(benches, bench_patch_large_file, bench_preserve_comments_large_file);
