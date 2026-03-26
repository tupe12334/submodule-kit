use super::super::parse_gitmodules;
use crate::strings;
use std::path::Path;
use std::process::exit;

pub fn run() -> bool {
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
        let sub_path = Path::new(&sub.path);
        if !sub_path.join(".git").exists() {
            println!(
                "{:<col_width$}  {}",
                sub.path,
                strings::STATUS_NOT_POPULATED_SKIPPED
            );
            continue;
        }

        let sub_repo = match git2::Repository::open(sub_path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("error: {}", strings::err_open_submodule(&sub.path, &e));
                exit(2);
            }
        };

        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(false).exclude_submodules(true);

        let statuses = match sub_repo.statuses(Some(&mut opts)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: {}", strings::err_get_status(&sub.path, &e));
                exit(2);
            }
        };

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

    all_ok
}
