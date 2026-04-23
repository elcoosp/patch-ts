use anyhow::Result;
use std::path::Path;

/// Fetch a unified diff from an HTTP(S) URL.
pub fn fetch_http(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .build()?;
    let resp = client.get(url).send()?;
    if !resp.status().is_success() {
        anyhow::bail!("HTTP error {} fetching {}", resp.status(), url);
    }
    let body = resp.text()?;
    Ok(body)
}

/// Fetch the diff introduced by a specific commit (or range) in a git repository.
/// `repo_path` may be a local directory or a remote URL (we clone to temp dir).
/// `commit` is a git reference like "abc123" or "HEAD~1..HEAD".
pub fn fetch_git_commit(repo_path: &str, commit: &str) -> Result<String> {
    use git2::Repository;
    // If repo_path is a URL, clone to a temp directory first
    let repo = if repo_path.starts_with("http://") || repo_path.starts_with("https://") || repo_path.starts_with("git@") {
        let tmp = tempfile::tempdir()?;
        Repository::clone(repo_path, tmp.path())?;
        Repository::open(tmp.path())?
    } else {
        Repository::open(Path::new(repo_path))?
    };

    let rev = repo.revparse_single(commit)?;
    let commit_obj = rev.peel_to_commit()?;
    let tree = commit_obj.tree()?;

    // Check if the commit has a parent before calling parent(0)
    if commit_obj.parent_count() == 0 {
        anyhow::bail!("Commit has no parent to diff against");
    }
    let parent = commit_obj.parent(0)?;
    let parent_tree = parent.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
    let mut buf = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        buf.extend_from_slice(line.content());
        buf.push(b'\n');
        true
    })?;
    Ok(String::from_utf8(buf)?)
}
