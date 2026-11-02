use criterion::{Criterion, criterion_group, criterion_main};
use patch_ts::ast::RustLanguage;
use patch_ts::patch::{PatchOptions, apply_literal_patch};
use std::fs;
use std::hint::black_box;
use tempfile::tempdir;

fn bench_patch_large_file(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("large.rs");
    let content = "fn main() {\n".to_string() + &"    println!(\"line\");\n".repeat(2000) + "}\n";
    fs::write(&file_path, &content).unwrap();

    let mut lang = RustLanguage::new();
    let mut options = PatchOptions::default();

    c.bench_function("patch 2000-line file", |b| {
        b.iter(|| {
            let result = apply_literal_patch(
                black_box(&file_path),
                black_box(1000),
                black_box("    println!(\"line\");"),
                black_box("    println!(\"patched\");"),
                black_box(&mut options),
                black_box(&mut lang),
            );
            black_box(result).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_patch_large_file,
    bench_preserve_comments_large_file
);
criterion_main!(benches);

fn bench_preserve_comments_large_file(c: &mut Criterion) {
    use patch_ts::ast::RustLanguage;
    use patch_ts::comment_preserve::preserve_comments;
    use std::fmt::Write as _;

    let mut lang = RustLanguage::new();
    let mut content = String::new();
    content.push_str("// File header\n");
    for i in 0..50 {
        let _ = writeln!(content, "/// Doc for function {i}");
        let _ = writeln!(content, "fn func_{i}() {{");
        for j in 0..38 {
            let _ = writeln!(content, "    let x_{i}_{j} = {};", i * j);
        }
        let _ = writeln!(content, "}} // end func_{i}");
    }
    let mut patched = String::new();
    for i in 0..50 {
        let _ = writeln!(patched, "fn func_{i}() {{");
        for j in 0..38 {
            let _ = writeln!(patched, "    let x_{i}_{j} = {};", i * j + 1);
        }
        patched.push_str("}\n");
    }

    c.bench_function("preserve comments 2000-line file", |b| {
        b.iter(|| {
            let result = preserve_comments(&content, &patched, &mut lang);
            black_box(result).unwrap();
        });
    });
}
