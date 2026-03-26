use super::*;
use std::fs;
use tempfile::tempdir;

fn make_sub(path: &str) -> SubmoduleInfo {
    SubmoduleInfo {
        path: path.to_string(),
        url: "https://example.com/repo.git".to_string(),
        branch: None,
    }
}

fn make_commit(repo: &git2::Repository, msg: &str, parents: &[&git2::Commit]) -> git2::Oid {
    let sig = git2::Signature::now("Test", "test@test.com").unwrap();
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, parents)
        .unwrap()
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

#[test]
fn empty_submodules_returns_true() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();
    assert!(check(&[], &repo, dir.path()).unwrap());
}

#[test]
fn not_populated_is_skipped() {
    let dir = tempdir().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();
    let subs = [make_sub("sub")];
    assert!(check(&subs, &repo, dir.path()).unwrap());
}

#[test]
fn synced_returns_true() {
    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();

    let sub_dir = parent_dir.path().join("sub");
    fs::create_dir(&sub_dir).unwrap();
    let sub_repo = git2::Repository::init(&sub_dir).unwrap();
    let commit_oid = make_commit(&sub_repo, "Initial", &[]);

    add_submodule_to_index(&parent_repo, "sub", commit_oid);

    let subs = [make_sub("sub")];
    assert!(check(&subs, &parent_repo, parent_dir.path()).unwrap());
}

#[test]
fn out_of_sync_returns_false() {
    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();

    let sub_dir = parent_dir.path().join("sub");
    fs::create_dir(&sub_dir).unwrap();
    let sub_repo = git2::Repository::init(&sub_dir).unwrap();
    let commit_oid = make_commit(&sub_repo, "Initial", &[]);

    // Record an older (empty tree) OID in the parent index
    let different_oid = {
        let sig = git2::Signature::now("Test", "test@test.com").unwrap();
        let tree_id = sub_repo.index().unwrap().write_tree().unwrap();
        let tree = sub_repo.find_tree(tree_id).unwrap();
        // We'll just store commit_oid offset by creating a second commit in sub
        let parent_commit = sub_repo.find_commit(commit_oid).unwrap();
        sub_repo
            .commit(None, &sig, &sig, "Second", &tree, &[&parent_commit])
            .unwrap()
    };

    add_submodule_to_index(&parent_repo, "sub", different_oid);

    let subs = [make_sub("sub")];
    assert!(!check(&subs, &parent_repo, parent_dir.path()).unwrap());
}

#[test]
fn run_fails_when_not_a_git_repo() {
    let _guard = super::super::super::CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    fs::write(dir.path().join(crate::strings::GITMODULES_FILE), "").unwrap();
    // No git repo — Repository::open(".") should fail
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let result = run();
    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_err());
}

#[test]
fn missing_index_entry_returns_error() {
    let parent_dir = tempdir().unwrap();
    let parent_repo = git2::Repository::init(parent_dir.path()).unwrap();

    let sub_dir = parent_dir.path().join("sub");
    fs::create_dir(&sub_dir).unwrap();
    let sub_repo = git2::Repository::init(&sub_dir).unwrap();
    make_commit(&sub_repo, "Initial", &[]);
    // Create .git so it looks populated but don't add to parent index

    let subs = [make_sub("sub")];
    assert!(check(&subs, &parent_repo, parent_dir.path()).is_err());
}
