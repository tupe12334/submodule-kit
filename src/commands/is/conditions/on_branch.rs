use super::super::{SubmoduleInfo, parse_gitmodules_str, short};
use crate::strings;
use std::fs;
use std::path::Path;

pub fn run() -> Result<bool, String> {
    let content = fs::read_to_string(strings::GITMODULES_FILE)
        .map_err(|e| strings::err_read_gitmodules(&e))?;
    let submodules = parse_gitmodules_str(&content)?;
    check(&submodules, Path::new("."))
}

pub(crate) fn check(submodules: &[SubmoduleInfo], base_path: &Path) -> Result<bool, String> {
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

        let sub_repo = git2::Repository::open(&sub_path)
            .map_err(|e| strings::err_open_submodule(&sub.path, &e))?;

        let head = sub_repo
            .head()
            .map_err(|e| strings::err_read_head(&sub.path, &e))?;

        if head.is_branch() {
            let current_branch = head.shorthand().unwrap_or(strings::LABEL_UNKNOWN);
            match &sub.branch {
                Some(expected) if current_branch != expected => {
                    println!(
                        "{:<col_width$}  {}  {}={}  {}={}",
                        sub.path,
                        strings::STATUS_WRONG_BRANCH,
                        strings::LABEL_CURRENT,
                        current_branch,
                        strings::LABEL_EXPECTED,
                        expected
                    );
                    all_ok = false;
                }
                _ => {
                    println!(
                        "{:<col_width$}  {}  {}",
                        sub.path,
                        strings::STATUS_ON_BRANCH,
                        current_branch
                    );
                }
            }
        } else {
            let sha = head
                .peel_to_commit()
                .map(|c| {
                    let full = c.id().to_string();
                    short(&full).to_string()
                })
                .unwrap_or_else(|_| strings::LABEL_UNKNOWN.to_string());
            println!(
                "{:<col_width$}  {}  {}",
                sub.path,
                strings::STATUS_DETACHED_HEAD,
                sha
            );
            all_ok = false;
        }
    }

    Ok(all_ok)
}

#[cfg(test)]
#[path = "on_branch_tests.rs"]
mod tests;
