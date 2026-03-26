use super::*;
use tempfile::tempdir;

fn make_sub(path: &str, url: &str, branch: Option<&str>) -> SubmoduleInfo {
    SubmoduleInfo {
        path: path.to_string(),
        url: url.to_string(),
        branch: branch.map(str::to_string),
    }
}

fn add_submodule_to_index(parent_repo: &git2::Repository, sub_path: &str, oid: git2::Oid) {
    let mut index = parent_repo.index().unwrap();
    let entry = git2::IndexEntry {
        ctime: git2::IndexTime::new(0, 0),
        mtime: git2::IndexTime::new(0, 0),
        dev: 0,
        ino: 0,
        mode: 0o160000,
        uid: 0,
        gid: 0,
        file_size: 0,
        id: oid,
        flags: 0,
        flags_extended: 0,
        path: sub_path.as_bytes().to_vec(),
    };
    index.add(&entry).unwrap();
    index.write().unwrap();
}

fn make_commit_in_bare(repo: &git2::Repository, branch: &str) -> git2::Oid {
    let sig = git2::Signature::now("Test", "test@test.com").unwrap();
    let tree_id = {
        let builder = repo.treebuilder(None).unwrap();
        builder.write().unwrap()
    };
    let tree = repo.find_tree(tree_id).unwrap();
    let refname = format!("refs/heads/{branch}");
    repo.commit(Some(&refname), &sig, &sig, "Initial", &tree, &[])
        .unwrap()
}

#[test]
fn empty_submodules_returns_true() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();
    assert!(check(&[], &repo).unwrap());
}

#[test]
fn missing_branch_returns_error() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();
    let subs = [make_sub("sub", "https://example.com/repo.git", None)];
    let err = check(&subs, &repo).unwrap_err();
    assert!(err.contains("branch"));
}

#[test]
fn up_to_date_returns_true() {
    let remote_dir = tempdir().unwrap();
    let remote_repo = git2::Repository::init_bare(remote_dir.path()).unwrap();
    let commit_oid = make_commit_in_bare(&remote_repo, "main");

    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();
    add_submodule_to_index(&parent_repo, "sub", commit_oid);

    let remote_url = format!("file://{}", remote_dir.path().display());
    let subs = [make_sub("sub", &remote_url, Some("main"))];
    assert!(check(&subs, &parent_repo).unwrap());
}

#[test]
fn behind_returns_false() {
    let remote_dir = tempdir().unwrap();
    let remote_repo = git2::Repository::init_bare(remote_dir.path()).unwrap();
    let commit_oid = make_commit_in_bare(&remote_repo, "main");

    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();

    // Record a different (older) OID in parent index
    let different_oid = {
        let sig = git2::Signature::now("Test", "test@test.com").unwrap();
        let tree_id = remote_repo.treebuilder(None).unwrap().write().unwrap();
        let tree = remote_repo.find_tree(tree_id).unwrap();
        let parent_commit = remote_repo.find_commit(commit_oid).unwrap();
        remote_repo
            .commit(
                Some("refs/heads/main"),
                &sig,
                &sig,
                "Second",
                &tree,
                &[&parent_commit],
            )
            .unwrap()
    };
    // Parent index still points to first commit, remote now has second
    add_submodule_to_index(&parent_repo, "sub", commit_oid);

    let remote_url = format!("file://{}", remote_dir.path().display());
    let subs = [make_sub("sub", &remote_url, Some("main"))];
    // remote tip is different_oid, parent records commit_oid → behind
    let _ = different_oid; // used above to advance remote
    assert!(!check(&subs, &parent_repo).unwrap());
}

#[test]
fn bad_url_returns_error() {
    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();

    // Add a fake OID to the index so git_rev_parse_submodule succeeds
    let fake_oid = git2::Oid::from_str("0000000000000000000000000000000000000000").unwrap();
    add_submodule_to_index(&parent_repo, "sub", fake_oid);

    let subs = [make_sub(
        "sub",
        "file:///nonexistent/path/to/repo",
        Some("main"),
    )];
    assert!(check(&subs, &parent_repo).is_err());
}

#[test]
fn run_fails_when_not_a_git_repo() {
    let _guard = super::super::super::CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join(crate::strings::GITMODULES_FILE), "").unwrap();
    // No git repo — Repository::open(".") should fail
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let result = run();
    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_err());
}

#[test]
fn missing_index_entry_returns_error() {
    let remote_dir = tempdir().unwrap();
    let remote_repo = git2::Repository::init_bare(remote_dir.path()).unwrap();
    make_commit_in_bare(&remote_repo, "main");

    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();
    // Don't add submodule to index

    let remote_url = format!("file://{}", remote_dir.path().display());
    let subs = [make_sub("sub", &remote_url, Some("main"))];
    assert!(check(&subs, &parent_repo).is_err());
}
