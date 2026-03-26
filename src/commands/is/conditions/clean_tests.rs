use super::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn make_sub(path: &str) -> SubmoduleInfo {
    SubmoduleInfo {
        path: path.to_string(),
        url: "https://example.com/repo.git".to_string(),
        branch: None,
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
    let subs = [make_sub("sub")];
    // "sub" has no .git, should be skipped → still true
    assert!(check(&subs, dir.path()).unwrap());
}

#[test]
fn clean_repo_returns_true() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    init_repo_with_commit(&sub_path);

    let subs = [make_sub("sub")];
    assert!(check(&subs, dir.path()).unwrap());
}

#[test]
fn dirty_repo_returns_false() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    init_repo_with_commit(&sub_path);

    // Create a tracked modified file
    let file = sub_path.join("file.txt");
    fs::write(&file, "initial").unwrap();
    {
        let repo = git2::Repository::open(&sub_path).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("file.txt")).unwrap();
        index.write().unwrap();
        let sig = git2::Signature::now("Test", "test@test.com").unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let parent = repo
            .find_commit(repo.head().unwrap().target().unwrap())
            .unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Add file", &tree, &[&parent])
            .unwrap();
    }
    // Modify the tracked file to make it dirty
    fs::write(file, "modified").unwrap();

    let subs = [make_sub("sub")];
    assert!(!check(&subs, dir.path()).unwrap());
}

#[test]
fn invalid_repo_returns_error() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    // Create a .git dir that is NOT a valid git repo
    fs::create_dir(sub_path.join(".git")).unwrap();

    let subs = [make_sub("sub")];
    assert!(check(&subs, dir.path()).is_err());
}
