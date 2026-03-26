use super::super::{git_ls_remote, git_rev_parse_submodule, parse_gitmodules, short};
use crate::strings;
use std::process::exit;

pub fn run() -> bool {
    let submodules = match parse_gitmodules() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            exit(2);
        }
    };

    // Validate branch is present for every submodule upfront.
    for sub in &submodules {
        if sub.branch.is_none() {
            eprintln!("error: {}", strings::err_missing_branch(&sub.path));
            exit(2);
        }
    }

    let repo = match git2::Repository::open(".") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {}", strings::err_open_repo(&e));
            exit(2);
        }
    };

    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);
    let mut all_ok = true;

    for sub in &submodules {
        let branch = sub.branch.as_deref().unwrap();

        let parent_sha = match git_rev_parse_submodule(&repo, &sub.path) {
            Ok(sha) => sha,
            Err(e) => {
                eprintln!("error: {e}");
                exit(2);
            }
        };

        let remote_sha = match git_ls_remote(&repo, &sub.url, branch) {
            Ok(sha) => sha,
            Err(e) => {
                eprintln!("error: {e}");
                exit(2);
            }
        };

        if parent_sha == remote_sha {
            println!(
                "{:<col_width$}  {}  {}",
                sub.path,
                strings::STATUS_UP_TO_DATE,
                short(&parent_sha)
            );
        } else {
            println!(
                "{:<col_width$}  {}      {}={}  {}={}",
                sub.path,
                strings::STATUS_BEHIND,
                strings::LABEL_PARENT,
                short(&parent_sha),
                strings::LABEL_REMOTE,
                short(&remote_sha)
            );
            all_ok = false;
        }
    }

    all_ok
}
