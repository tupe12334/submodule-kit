use super::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn list_with_submodules_branch_and_no_branch() {
    let _guard = crate::commands::is::CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    let original = std::env::current_dir().unwrap();
    // `list` now reads submodules through libgit2, so the working directory must
    // be a git repository.
    git2::Repository::init(dir.path()).unwrap();
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
    let result = run();
    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_ok());
}

#[test]
fn list_fails_when_no_gitmodules_file() {
    let _guard = crate::commands::is::CWD_MUTEX.lock().unwrap();
    let dir = tempdir().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let result = run();
    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_err());
}
