use super::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn make_sub(path: &str, branch: Option<&str>) -> SubmoduleInfo {
    SubmoduleInfo {
        path: path.to_string(),
        url: "https://example.com/repo.git".to_string(),
        branch: branch.map(str::to_string),
    }
}

fn init_repo_with_commit(path: &Path) -> git2::Repository {
    let repo = git2::Repository::init(path).unwrap();
    {
        let sig = git2::Signature::now("Test", "test@test.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
            .unwrap();
    }
    repo
}

#[test]
fn empty_submodules_returns_true() {
    let dir = tempdir().unwrap();
    assert!(check(&[], dir.path()).unwrap());
}

#[test]
fn not_populated_is_skipped() {
    let dir = tempdir().unwrap();
    let subs = [make_sub("sub", Some("main"))];
    assert!(check(&subs, dir.path()).unwrap());
}

#[test]
fn on_correct_branch_returns_true() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    let repo = init_repo_with_commit(&sub_path);
    let head_branch = repo.head().unwrap().shorthand().unwrap().to_string();

    let subs = [make_sub("sub", Some(&head_branch))];
    assert!(check(&subs, dir.path()).unwrap());
}

#[test]
fn on_branch_no_expected_returns_true() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    init_repo_with_commit(&sub_path);

    let subs = [make_sub("sub", None)];
    assert!(check(&subs, dir.path()).unwrap());
}

#[test]
fn wrong_branch_returns_false() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    init_repo_with_commit(&sub_path);

    let subs = [make_sub("sub", Some("nonexistent-branch"))];
    assert!(!check(&subs, dir.path()).unwrap());
}

#[test]
fn detached_head_returns_false() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    let repo = init_repo_with_commit(&sub_path);
    let oid = repo.head().unwrap().target().unwrap();
    repo.set_head_detached(oid).unwrap();

    let subs = [make_sub("sub", Some("main"))];
    assert!(!check(&subs, dir.path()).unwrap());
}

#[test]
fn invalid_repo_returns_error() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    fs::create_dir(sub_path.join(".git")).unwrap();

    let subs = [make_sub("sub", Some("main"))];
    assert!(check(&subs, dir.path()).is_err());
}
