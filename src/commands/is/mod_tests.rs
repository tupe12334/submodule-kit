use super::*;
use std::fs;
use tempfile::tempdir;

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
fn parse_empty_content() {
    let result = parse_gitmodules_str("").unwrap();
    assert!(result.is_empty());
}

#[test]
fn parse_single_submodule_with_branch() {
    let content = r#"
[submodule "foo"]
    path = foo
    url = https://example.com/foo.git
    branch = main
"#;
    let subs = parse_gitmodules_str(content).unwrap();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].path, "foo");
    assert_eq!(subs[0].url, "https://example.com/foo.git");
    assert_eq!(subs[0].branch.as_deref(), Some("main"));
}

#[test]
fn parse_single_submodule_without_branch() {
    let content = r#"
[submodule "bar"]
    path = bar
    url = https://example.com/bar.git
"#;
    let subs = parse_gitmodules_str(content).unwrap();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].path, "bar");
    assert_eq!(subs[0].url, "https://example.com/bar.git");
    assert!(subs[0].branch.is_none());
}

#[test]
fn parse_multiple_submodules() {
    let content = r#"
[submodule "a"]
    path = a
    url = https://example.com/a.git
    branch = main
[submodule "b"]
    path = b
    url = https://example.com/b.git
"#;
    let subs = parse_gitmodules_str(content).unwrap();
    assert_eq!(subs.len(), 2);
    assert_eq!(subs[0].path, "a");
    assert_eq!(subs[1].path, "b");
}

#[test]
fn parse_missing_path_returns_error() {
    let content = r#"
[submodule "bad"]
    url = https://example.com/bad.git
"#;
    let err = parse_gitmodules_str(content).unwrap_err();
    assert!(err.contains("bad"));
    assert!(err.contains("path"));
}

#[test]
fn parse_missing_url_returns_error() {
    let content = r#"
[submodule "bad"]
    path = bad
"#;
    let err = parse_gitmodules_str(content).unwrap_err();
    assert!(err.contains("bad"));
    assert!(err.contains("url"));
}

#[test]
fn parse_ignores_non_submodule_lines() {
    let content = r#"
[core]
    repositoryformatversion = 0
[submodule "x"]
    path = x
    url = https://example.com/x.git
"#;
    let subs = parse_gitmodules_str(content).unwrap();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].path, "x");
}

// ── CWD-based tests (serialized via CWD_MUTEX) ────────────────────────────

#[test]
fn list_with_submodules_branch_and_no_branch() {
    let _guard = CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    let original = std::env::current_dir().unwrap();
    let gitmodules = concat!(
        "[submodule \"with\"]\n",
        "    path = with\n",
        "    url = https://example.com/with.git\n",
        "    branch = main\n",
        "[submodule \"without\"]\n",
        "    path = without\n",
        "    url = https://example.com/without.git\n",
    );
    fs::write(dir.path().join(crate::strings::GITMODULES_FILE), gitmodules).unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let result = list();
    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_ok());
}

#[test]
fn list_fails_when_no_gitmodules_file() {
    let _guard = CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let result = list();
    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_err());
}

#[test]
fn run_all_conditions_with_empty_submodules() {
    let _guard = CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    let original = std::env::current_dir().unwrap();
    fs::write(dir.path().join(crate::strings::GITMODULES_FILE), "").unwrap();
    git2::Repository::init(dir.path()).unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let conditions = vec![
        IsCondition::Populated,
        IsCondition::Clean,
        IsCondition::OnBranch,
        IsCondition::Synced,
        IsCondition::AllUpToDate,
    ];
    let result = run(conditions);
    std::env::set_current_dir(&original).unwrap();
    assert_eq!(result.unwrap(), true);
}

#[test]
fn run_returns_error_when_no_gitmodules_file() {
    let _guard = CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    // Call each condition so all err_read_gitmodules closures are exercised.
    for cond in [
        IsCondition::Populated,
        IsCondition::Clean,
        IsCondition::OnBranch,
        IsCondition::Synced,
        IsCondition::AllUpToDate,
    ] {
        assert!(run(vec![cond]).is_err());
    }
    std::env::set_current_dir(&original).unwrap();
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
