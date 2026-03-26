use crate::strings;
use crate::submodule::{SubmoduleInfo, parse_gitmodules_str};
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

        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(false).exclude_submodules(true);

        let statuses = sub_repo
            .statuses(Some(&mut opts))
            .map_err(|e| strings::err_get_status(&sub.path, &e))?;

        if statuses.is_empty() {
            println!("{:<col_width$}  {}", sub.path, strings::STATUS_CLEAN);
        } else {
            println!(
                "{:<col_width$}  {} ({} changed file(s))",
                sub.path,
                strings::STATUS_DIRTY,
                statuses.len()
            );
            all_ok = false;
        }
    }

    Ok(all_ok)
}

#[cfg(test)]
#[path = "clean_tests.rs"]
mod tests;
