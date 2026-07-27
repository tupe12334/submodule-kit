use crate::strings;
use crate::submodule::submodules;

/// Print every configured submodule (path, url, and optional branch).
///
/// # Errors
///
/// Returns an error if the git repository cannot be opened or its submodule
/// list cannot be read (see [`submodules`]).
pub fn run() -> Result<(), String> {
    let repo = git2::Repository::open(".").map_err(|e| strings::err_open_repo(&e))?;
    let submodules = submodules(&repo)?;
    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);
    for sub in &submodules {
        match &sub.branch {
            Some(branch) => println!("{:<col_width$}  {}  (branch: {branch})", sub.path, sub.url),
            None => println!("{:<col_width$}  {}", sub.path, sub.url),
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "list_tests.rs"]
mod tests;
