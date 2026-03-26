use super::*;
use std::fs;
use tempfile::tempdir;

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
