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
