use super::super::{git_rev_parse_submodule, parse_gitmodules, short};
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
        let sub_path = Path::new(&sub.path);
        if !sub_path.join(".git").exists() {
            println!(
                "{:<col_width$}  {}",
                sub.path,
                strings::STATUS_NOT_POPULATED_SKIPPED
            );
            continue;
        }

        let recorded_sha = match git_rev_parse_submodule(&repo, &sub.path) {
            Ok(sha) => sha,
            Err(e) => {
                eprintln!("error: {e}");
                exit(2);
            }
        };

        let sub_repo = match git2::Repository::open(sub_path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("error: {}", strings::err_open_submodule(&sub.path, &e));
                exit(2);
            }
        };

        let head_sha = match sub_repo.head() {
            Ok(head) => head
                .peel_to_commit()
                .map(|c| c.id().to_string())
                .unwrap_or_default(),
            Err(e) => {
                eprintln!("error: {}", strings::err_read_head(&sub.path, &e));
                exit(2);
            }
        };

        if recorded_sha == head_sha {
            println!(
                "{:<col_width$}  {}      {}",
                sub.path,
                strings::STATUS_SYNCED,
                short(&recorded_sha)
            );
        } else {
            println!(
                "{:<col_width$}  {}  {}={}  {}={}",
                sub.path,
                strings::STATUS_OUT_OF_SYNC,
                strings::LABEL_RECORDED,
                short(&recorded_sha),
                strings::LABEL_LOCAL,
                short(&head_sha)
            );
            all_ok = false;
        }
    }

    if !all_ok {
        exit(1);
    }
}
