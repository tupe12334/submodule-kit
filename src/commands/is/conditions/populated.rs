use crate::strings;
use crate::submodule::{SubmoduleInfo, submodules};
use std::path::Path;

pub fn run() -> Result<bool, String> {
    let repo = git2::Repository::open(".").map_err(|e| strings::err_open_repo(&e))?;
    let submodules = submodules(&repo)?;
    Ok(check(&submodules, Path::new(".")))
}

pub(crate) fn check(submodules: &[SubmoduleInfo], base_path: &Path) -> bool {
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
