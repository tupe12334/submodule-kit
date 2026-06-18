use crate::submodule::parse_gitmodules;

/// Print every configured submodule (path, url, and optional branch).
///
/// # Errors
///
/// Returns an error if the repository's `.gitmodules` file cannot be read or
/// parsed (see [`parse_gitmodules`]).
pub fn run() -> Result<(), String> {
    let submodules = parse_gitmodules()?;
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
