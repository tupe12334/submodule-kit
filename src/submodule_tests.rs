use super::*;
use tempfile::{TempDir, tempdir};

/// Create a temporary git repository whose working tree contains a
/// `.gitmodules` file with the given contents. libgit2 reads this file when
/// `Repository::submodules` is called, so it exercises the same path the CLI
/// uses at runtime.
fn repo_with_gitmodules(content: &str) -> (TempDir, git2::Repository) {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();
    std::fs::write(dir.path().join(".gitmodules"), content).unwrap();
    (dir, repo)
}

fn find<'a>(subs: &'a [SubmoduleInfo], path: &str) -> &'a SubmoduleInfo {
    subs.iter().find(|s| s.path == path).unwrap()
}

#[test]
fn short_truncates_long_sha() {
    assert_eq!(short("abcdef1234567890"), "abcdef1");
}

#[test]
fn short_preserves_short_string() {
    assert_eq!(short("abc"), "abc");
}

#[test]
fn short_empty_string() {
    assert_eq!(short(""), "");
}

#[test]
fn submodules_empty_when_no_gitmodules() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();
    let subs = submodules(&repo).unwrap();
    assert!(subs.is_empty());
}

#[test]
fn submodules_single_with_branch() {
    let (_dir, repo) = repo_with_gitmodules(
        r#"
[submodule "foo"]
    path = foo
    url = https://example.com/foo.git
    branch = main
"#,
    );
    let subs = submodules(&repo).unwrap();
    assert_eq!(subs.len(), 1);
    let foo = find(&subs, "foo");
    assert_eq!(foo.url, "https://example.com/foo.git");
    assert_eq!(foo.branch.as_deref(), Some("main"));
}

#[test]
fn submodules_single_without_branch() {
    let (_dir, repo) = repo_with_gitmodules(
        r#"
[submodule "bar"]
    path = bar
    url = https://example.com/bar.git
"#,
    );
    let subs = submodules(&repo).unwrap();
    assert_eq!(subs.len(), 1);
    let bar = find(&subs, "bar");
    assert_eq!(bar.url, "https://example.com/bar.git");
    assert!(bar.branch.is_none());
}

#[test]
fn submodules_multiple() {
    let (_dir, repo) = repo_with_gitmodules(
        r#"
[submodule "a"]
    path = a
    url = https://example.com/a.git
    branch = main
[submodule "b"]
    path = b
    url = https://example.com/b.git
"#,
    );
    let subs = submodules(&repo).unwrap();
    assert_eq!(subs.len(), 2);
    find(&subs, "a");
    find(&subs, "b");
}

/// The previous hand-rolled parser matched the exact byte sequences
/// `"path = "` / `"url = "`, so a `.gitmodules` written without spaces around
/// `=` (which `git` itself accepts) was mis-parsed. libgit2 uses the real
/// git-config grammar, so these files now parse correctly.
#[test]
fn submodules_handles_no_spaces_around_equals() {
    let (_dir, repo) = repo_with_gitmodules(
        "[submodule \"tight\"]\n\tpath=tight\n\turl=https://example.com/tight.git\n",
    );
    let subs = submodules(&repo).unwrap();
    assert_eq!(subs.len(), 1);
    let tight = find(&subs, "tight");
    assert_eq!(tight.url, "https://example.com/tight.git");
}

#[test]
fn git_ls_remote_returns_error_when_branch_not_found() {
    let remote_dir = tempdir().unwrap();
    let remote_repo = git2::Repository::init_bare(remote_dir.path()).unwrap();
    let sig = git2::Signature::now("Test", "test@test.com").unwrap();
    let tree_id = remote_repo.treebuilder(None).unwrap().write().unwrap();
    let tree = remote_repo.find_tree(tree_id).unwrap();
    remote_repo
        .commit(Some("refs/heads/other"), &sig, &sig, "Init", &tree, &[])
        .unwrap();

    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();
    let remote_url = format!("file://{}", remote_dir.path().display());
    let err = git_ls_remote(&parent_repo, &remote_url, "main").unwrap_err();
    assert!(err.contains("main"));
}

#[test]
fn git_rev_parse_submodule_on_bare_repo_returns_error() {
    let dir = tempdir().unwrap();
    let bare_repo = git2::Repository::init_bare(dir.path()).unwrap();
    let result = git_rev_parse_submodule(&bare_repo, "sub");
    assert!(result.is_err());
}
