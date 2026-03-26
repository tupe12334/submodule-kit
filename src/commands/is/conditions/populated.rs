use super::super::parse_gitmodules;
use crate::strings;
use std::path::Path;
use std::process::exit;

pub fn run() {
    let submodules = match parse_gitmodules() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            exit(2);
        }
    };

    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);
    let mut all_ok = true;

    for sub in &submodules {
        // A cloned submodule has a .git file (gitdir pointer) at its root.
        if Path::new(&sub.path).join(".git").exists() {
            println!("{:<col_width$}  {}", sub.path, strings::STATUS_POPULATED);
        } else {
            println!("{:<col_width$}  {}", sub.path, strings::STATUS_MISSING);
            all_ok = false;
        }
    }

    if !all_ok {
        exit(1);
    }
}
