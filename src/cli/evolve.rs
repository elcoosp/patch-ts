use anyhow::Result;
use crate::evolve::EvolutionConfig;
use super::types::*;
use std::path::Path;
pub fn handle_evolve(args: EvolveArgs) -> Result<()> {
    let file_path = Path::new(&args.file);
    let original = std::fs::read_to_string(file_path)?;
    let config = EvolutionConfig { population_size: args.population_size, max_generations: 3, timeout_secs: args.evolve_timeout };
    let best = crate::evolve::run_evolution(file_path, &original, &args.old, &args.new, &config);
    if args.json { println!("{}", serde_json::to_string(&best)?); }
    else {
        println!("Best candidate (fitness: {:.2}):", best.fitness);
        println!("  Replace: `{}`", best.old);
        println!("  With:    `{}`", best.new);
        println!("  Fuzz: {}, Confidence: {:.2}, Uniqueness: {:.2}", best.fuzz, best.confidence, best.uniqueness_weight);
    }
    if args.apply {
        let patch_args = PatchArgs {
            file: Some(args.file.clone()), files: None, line: Some(1),
            fuzz: best.fuzz, old: Some(best.old), new: Some(best.new),
            confidence: best.confidence, fix_indent: false, diff: false,
            delete: None, expect: None, after: None, content: None,
            dry_run: false, force: false, no_backup: false, json: false,
            no_auto_repair: false, marker: None, serial: false, plugin: None,
            allow_all_paths: false, symbol: None, url: None, git_commit: None,
            no_strip_fence: false, no_compile_check: false, compile_timeout: 30,
            no_sanitize: false, no_ellipsis: false, uniqueness_weight: best.uniqueness_weight,
            strict_whitespace: false, cross_file: false, agent: None, model: None,
            no_provenance: true,
            fix_headers: false,
            verify: false,
            verify_test: None,
        validate_first: false,
        };
        super::patch::apply_patch_to_file(file_path, &patch_args)?;
        println!("Evolutionary patch applied to {}", args.file);
    }
    Ok(())
}
