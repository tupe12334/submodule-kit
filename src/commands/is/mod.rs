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
}

pub fn run(condition: IsCondition) {
    match condition {
        IsCondition::AllUpToDate => all_up_to_date(),
        IsCondition::Populated => populated(),
    }
}

struct SubmoduleInfo {
    path: String,
    url: String,
    branch: Option<String>,
}

fn parse_gitmodules() -> Result<Vec<SubmoduleInfo>, String> {
    let content = fs::read_to_string(".gitmodules")
        .map_err(|e| format!("Failed to read .gitmodules: {e}"))?;

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
        let path =
            path.ok_or_else(|| format!("submodule '{name}' is missing 'path =' in .gitmodules"))?;
        let url =
            url.ok_or_else(|| format!("submodule '{name}' is missing 'url =' in .gitmodules"))?;
        out.push(SubmoduleInfo { path, url, branch });
        Ok(())
    };

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("[submodule ") && line.ends_with(']') {
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
                line.trim_start_matches("[submodule \"")
                    .trim_end_matches("\"]")
                    .to_string(),
            );
        } else if let Some(v) = line.strip_prefix("path = ") {
            current_path = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("url = ") {
            current_url = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("branch = ") {
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
    let index = repo
        .index()
        .map_err(|e| format!("failed to open index: {e}"))?;
    let entry = index
        .get_path(Path::new(path), 0)
        .ok_or_else(|| format!("submodule '{path}' not found in index"))?;
    Ok(entry.id.to_string())
}

fn git_ls_remote(repo: &git2::Repository, url: &str, branch: &str) -> Result<String, String> {
    let refspec = format!("refs/heads/{branch}");
    let mut remote = repo
        .remote_anonymous(url)
        .map_err(|e| format!("failed to create remote for {url}: {e}"))?;
    remote
        .connect(git2::Direction::Fetch)
        .map_err(|e| format!("failed to connect to {url}: {e}"))?;
    let list = remote
        .list()
        .map_err(|e| format!("failed to list refs at {url}: {e}"))?;
    list.iter()
        .find(|r| r.name() == refspec)
        .map(|r| r.oid().to_string())
        .ok_or_else(|| format!("ref {refspec} not found at {url}"))
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
            eprintln!(
                "error: submodule '{}' is missing 'branch =' in .gitmodules",
                sub.path
            );
            exit(2);
        }
    }

    let repo = match git2::Repository::open(".") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: failed to open git repository: {e}");
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
                "{:<col_width$}  up-to-date  {}",
                sub.path,
                short(&parent_sha)
            );
        } else {
            println!(
                "{:<col_width$}  behind      parent={}  remote={}",
                sub.path,
                short(&parent_sha),
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
            println!("{:<col_width$}  populated", sub.path);
        } else {
            println!("{:<col_width$}  missing", sub.path);
            all_ok = false;
        }
    }

    if !all_ok {
        exit(1);
    }
}
