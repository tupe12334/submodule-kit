use crate::strings;
use crate::submodule::{SubmoduleInfo, git_ls_remote, git_rev_parse_submodule, short, submodules};

pub fn run() -> Result<bool, String> {
    let repo = git2::Repository::open(".").map_err(|e| strings::err_open_repo(&e))?;
    let submodules = submodules(&repo)?;
    check(&submodules, &repo)
}

pub fn check(submodules: &[SubmoduleInfo], repo: &git2::Repository) -> Result<bool, String> {
    // Validate branch is present for every submodule upfront.
    for sub in submodules {
        if sub.branch.is_none() {
            return Err(strings::err_missing_branch(&sub.path));
        }
    }

    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);
    let mut all_ok = true;

    for sub in submodules {
        // The loop above guarantees `branch` is `Some`, but propagate an error
        // instead of unwrapping so a future change can never turn this into a panic.
        let branch = sub
            .branch
            .as_deref()
            .ok_or_else(|| strings::err_missing_branch(&sub.path))?;

        let parent_sha = git_rev_parse_submodule(repo, &sub.path)?;
        let remote_sha = git_ls_remote(repo, &sub.url, branch)?;

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

    Ok(all_ok)
}

#[cfg(test)]
#[path = "all_up_to_date_tests.rs"]
mod tests;
