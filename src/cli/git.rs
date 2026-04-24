use anyhow::Result;
use crate::diagnostics::JsonDiagnostic;
use crate::patch::{apply_unified_diff, PatchOptions};
use std::path::Path;
use super::types::*;

pub fn handle_git(args: GitArgs) -> Result<()> {
    match args.action {
        GitAction::Apply(ga) => handle_git_apply(ga),
        GitAction::Diff(gd)  => handle_git_diff(gd),
    }
}
fn handle_git_apply(args: GitApplyArgs) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let rev = repo.revparse_single(&args.commit)?;
    let commit = rev.peel_to_commit()?;
    let tree = commit.tree()?;
    let parent = if commit.parent_count() > 0 { commit.parent(0)?.tree()? } else { tree.clone() };
    let diff = repo.diff_tree_to_tree(Some(&parent), Some(&tree), None)?;
    let mut diff_text = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| { diff_text.extend_from_slice(line.content()); true })?;
    let diff_str = String::from_utf8(diff_text)?;
    if let Some(file_path) = &args.file {
        let options = PatchOptions { fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force, no_backup: args.no_backup, ..Default::default() };
        apply_unified_diff(Path::new(file_path), &diff_str, options)?;
    } else {
        let mut files_to_patch = Vec::new();
        diff.foreach(&mut |delta, _| { if let Some(path) = delta.new_file().path() { files_to_patch.push(path.to_path_buf()); } true }, None, None, None)?;
        for file in &files_to_patch {
            let options = PatchOptions { fuzz_radius: args.fuzz, dry_run: args.dry_run, force: args.force, no_backup: args.no_backup, ..Default::default() };
            apply_unified_diff(file, &diff_str, options.clone())?;
        }
    }
    if args.json { println!("{}", serde_json::to_string(&JsonDiagnostic::success())?); }
    Ok(())
}
fn handle_git_diff(args: GitDiffArgs) -> Result<()> {
    let repo = git2::Repository::open(".")?;
    let target_ref = repo.revparse_single(&args.target)?;
    let target_tree = target_ref.peel_to_tree()?;
    let head_tree = repo.head()?.peel_to_tree()?;
    let diff = repo.diff_tree_to_tree(Some(&target_tree), Some(&head_tree), None)?;
    let mut diff_text = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| { diff_text.extend_from_slice(line.content()); true })?;
    let diff_str = String::from_utf8(diff_text)?;
    if args.apply { /* similar loop */ } else { print!("{}", diff_str); }
    Ok(())
}
