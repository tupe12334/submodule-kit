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

#[test]
fn empty_submodules_returns_true() {
    let dir = tempdir().unwrap();
    assert!(check(&[], dir.path()));
}

#[test]
fn populated_submodule_returns_true() {
    let dir = tempdir().unwrap();
    let sub_path = dir.path().join("sub");
    fs::create_dir(&sub_path).unwrap();
    fs::create_dir(sub_path.join(".git")).unwrap();

    let subs = [make_sub("sub")];
    assert!(check(&subs, dir.path()));
}

#[test]
fn missing_submodule_returns_false() {
    let dir = tempdir().unwrap();

    let subs = [make_sub("sub")];
    assert!(!check(&subs, dir.path()));
}

#[test]
fn mixed_returns_false() {
    let dir = tempdir().unwrap();
    let sub_a = dir.path().join("a");
    fs::create_dir(&sub_a).unwrap();
    fs::create_dir(sub_a.join(".git")).unwrap();

    let subs = [make_sub("a"), make_sub("b")];
    assert!(!check(&subs, dir.path()));
}

#[test]
fn col_width_aligns_correctly() {
    let dir = tempdir().unwrap();
    let long_name = dir.path().join("longname");
    fs::create_dir(&long_name).unwrap();
    fs::create_dir(long_name.join(".git")).unwrap();

    let subs = [make_sub("longname"), make_sub("s")];
    // just ensure it doesn't panic
    let result = check(&subs, dir.path());
    assert!(!result); // "s" is missing
}
