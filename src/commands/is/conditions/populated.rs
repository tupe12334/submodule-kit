use crate::strings;
use crate::submodule::{SubmoduleInfo, parse_gitmodules_str};
use std::fs;
use std::path::Path;

pub fn run() -> Result<bool, String> {
    let content = fs::read_to_string(strings::GITMODULES_FILE)
        .map_err(|e| strings::err_read_gitmodules(&e))?;
    let submodules = parse_gitmodules_str(&content)?;
    Ok(check(&submodules, Path::new(".")))
}

pub fn check(submodules: &[SubmoduleInfo], base_path: &Path) -> bool {
    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);
    let mut all_ok = true;

    for sub in submodules {
        if base_path.join(&sub.path).join(".git").exists() {
            println!("{:<col_width$}  {}", sub.path, strings::STATUS_POPULATED);
        } else {
            println!("{:<col_width$}  {}", sub.path, strings::STATUS_MISSING);
            all_ok = false;
        }
    }

    all_ok
}

#[cfg(test)]
#[path = "populated_tests.rs"]
mod tests;
