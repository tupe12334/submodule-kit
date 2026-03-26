use crate::strings;
use clap::Subcommand;
use std::fs;
use std::path::Path;
use std::process::exit;

#[derive(Subcommand)]
pub enum IsCondition {
    /// Check whether every submodule's parent-recorded commit matches the tip of its remote branch
    AllUpToDate,
    /// Check whether all submodules have been initialized and cloned locally
    Populated,
    /// Check whether all submodules have no uncommitted changes
    Clean,
    /// Check whether each submodule's locally checked-out commit matches what the parent records
    Synced,
    /// Check whether all submodules are checked out on their configured branch (not detached HEAD)
    OnBranch,
}

pub fn run(condition: IsCondition) {
    match condition {
        IsCondition::AllUpToDate => all_up_to_date(),
        IsCondition::Populated => populated(),
        IsCondition::Clean => clean(),
        IsCondition::Synced => synced(),
        IsCondition::OnBranch => on_branch(),
    }
}

struct SubmoduleInfo {
    path: String,
    url: String,
    branch: Option<String>,
}

fn parse_gitmodules() -> Result<Vec<SubmoduleInfo>, String> {
    let content = fs::read_to_string(strings::GITMODULES_FILE)
        .map_err(|e| strings::err_read_gitmodules(&e))?;

    let mut submodules: Vec<SubmoduleInfo> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_path: Option<String> = None;
    let mut current_url: Option<String> = None;
    let mut current_branch: Option<String> = None;

    let flush = |name: String,
                 path: Option<String>,
                 url: Option<String>,
                 branch: Option<String>,
                 out: &mut Vec<SubmoduleInfo>|
     -> Result<(), String> {
        let path = path.ok_or_else(|| strings::err_missing_path(&name))?;
        let url = url.ok_or_else(|| strings::err_missing_url(&name))?;
        out.push(SubmoduleInfo { path, url, branch });
        Ok(())
    };

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(strings::SUBMODULE_SECTION_CHECK) && line.ends_with(']') {
            if let Some(name) = current_name.take() {
                flush(
                    name,
                    current_path.take(),
                    current_url.take(),
                    current_branch.take(),
                    &mut submodules,
                )?;
            }
            current_name = Some(
                line.trim_start_matches(strings::SUBMODULE_SECTION_PREFIX)
                    .trim_end_matches(strings::SUBMODULE_SECTION_SUFFIX)
                    .to_string(),
            );
        } else if let Some(v) = line.strip_prefix(strings::KEY_PATH) {
            current_path = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix(strings::KEY_URL) {
            current_url = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix(strings::KEY_BRANCH) {
            current_branch = Some(v.trim().to_string());
        }
    }

    if let Some(name) = current_name.take() {
        flush(
            name,
            current_path.take(),
            current_url.take(),
            current_branch.take(),
            &mut submodules,
        )?;
    }

    Ok(submodules)
}

fn git_rev_parse_submodule(repo: &git2::Repository, path: &str) -> Result<String, String> {
    let index = repo.index().map_err(|e| strings::err_open_index(&e))?;
    let entry = index
        .get_path(Path::new(path), 0)
        .ok_or_else(|| strings::err_not_in_index(path))?;
    Ok(entry.id.to_string())
}

fn git_ls_remote(repo: &git2::Repository, url: &str, branch: &str) -> Result<String, String> {
    let refspec = format!("{}{branch}", strings::REFS_HEADS_PREFIX);
    let mut remote = repo
        .remote_anonymous(url)
        .map_err(|e| strings::err_create_remote(url, &e))?;
    remote
        .connect(git2::Direction::Fetch)
        .map_err(|e| strings::err_connect_remote(url, &e))?;
    let list = remote.list().map_err(|e| strings::err_list_refs(url, &e))?;
    list.iter()
        .find(|r| r.name() == refspec)
        .map(|r| r.oid().to_string())
        .ok_or_else(|| strings::err_ref_not_found(&refspec, url))
}

fn short(sha: &str) -> &str {
    &sha[..sha.len().min(7)]
}

fn all_up_to_date() {
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

    if !all_ok {
        exit(1);
    }
}

fn populated() {
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

fn clean() {
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

    if !all_ok {
        exit(1);
    }
}

fn synced() {
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

fn on_branch() {
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

    if !all_ok {
        exit(1);
    }
}

fn short_owned(sha: &str) -> String {
    sha[..sha.len().min(7)].to_string()
}
