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

        let head = match sub_repo.head() {
            Ok(h) => h,
            Err(e) => {
                eprintln!("error: {}", strings::err_read_head(&sub.path, &e));
                exit(2);
            }
        };

        if head.is_branch() {
            let current_branch = head.shorthand().unwrap_or(strings::LABEL_UNKNOWN);
            match &sub.branch {
                Some(expected) if current_branch != expected => {
                    println!(
                        "{:<col_width$}  {}  {}={}  {}={}",
                        sub.path,
                        strings::STATUS_WRONG_BRANCH,
                        strings::LABEL_CURRENT,
                        current_branch,
                        strings::LABEL_EXPECTED,
                        expected
                    );
                    all_ok = false;
                }
                _ => {
                    println!(
                        "{:<col_width$}  {}  {}",
                        sub.path,
                        strings::STATUS_ON_BRANCH,
                        current_branch
                    );
                }
            }
        } else {
            let sha = head
                .peel_to_commit()
                .map(|c| short_owned(&c.id().to_string()))
                .unwrap_or_else(|_| strings::LABEL_UNKNOWN.to_string());
            println!(
                "{:<col_width$}  {}  {}",
                sub.path,
                strings::STATUS_DETACHED_HEAD,
                sha
            );
            all_ok = false;
        }
    }

    all_ok
}

fn short_owned(sha: &str) -> String {
    sha[..sha.len().min(7)].to_string()
}
