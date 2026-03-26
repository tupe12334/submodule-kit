use super::super::{SubmoduleInfo, git_rev_parse_submodule, parse_gitmodules_str, short};
use crate::strings;
use std::fs;
use std::path::Path;

pub fn run() -> Result<bool, String> {
    let content = fs::read_to_string(strings::GITMODULES_FILE)
        .map_err(|e| strings::err_read_gitmodules(&e))?;
    let submodules = parse_gitmodules_str(&content)?;
    let repo = git2::Repository::open(".").map_err(|e| strings::err_open_repo(&e))?;
    check(&submodules, &repo, Path::new("."))
}

pub(crate) fn check(
    submodules: &[SubmoduleInfo],
    repo: &git2::Repository,
    base_path: &Path,
) -> Result<bool, String> {
    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);
    let mut all_ok = true;

    for sub in submodules {
        let sub_path = base_path.join(&sub.path);
        if !sub_path.join(".git").exists() {
            println!(
                "{:<col_width$}  {}",
                sub.path,
                strings::STATUS_NOT_POPULATED_SKIPPED
            );
            continue;
        }

        let recorded_sha = git_rev_parse_submodule(repo, &sub.path)?;

        let sub_repo = git2::Repository::open(&sub_path)
            .map_err(|e| strings::err_open_submodule(&sub.path, &e))?;

        let head_sha = sub_repo
            .head()
            .map_err(|e| strings::err_read_head(&sub.path, &e))?
            .peel_to_commit()
            .map(|c| c.id().to_string())
            .unwrap_or_default();

        if recorded_sha == head_sha {
            println!(
                "{:<col_width$}  {}      {}",
                sub.path,
                strings::STATUS_SYNCED,
                short(&recorded_sha)
            );
        } else {
            println!(
                "{:<col_width$}  {}  {}={}  {}={}",
                sub.path,
                strings::STATUS_OUT_OF_SYNC,
                strings::LABEL_RECORDED,
                short(&recorded_sha),
                strings::LABEL_LOCAL,
                short(&head_sha)
            );
            all_ok = false;
        }
    }

    Ok(all_ok)
}

#[cfg(test)]
#[path = "synced_tests.rs"]
mod tests;
