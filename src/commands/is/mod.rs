use clap::Subcommand;
use std::fs;
use std::process::{Command, exit};

#[derive(Subcommand)]
pub enum IsCondition {
    /// Check whether every submodule's parent-recorded commit matches the tip of its remote branch
    AllUpToDate,
}

pub fn run(condition: IsCondition) {
    match condition {
        IsCondition::AllUpToDate => all_up_to_date(),
    }
}

struct SubmoduleInfo {
    path: String,
    url: String,
    branch: String,
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
        let branch = branch
            .ok_or_else(|| format!("submodule '{name}' is missing 'branch =' in .gitmodules"))?;
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

fn git_rev_parse_submodule(path: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", &format!(":{path}")])
        .output()
        .map_err(|e| format!("failed to run git rev-parse: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "git rev-parse :{path} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_ls_remote(url: &str, branch: &str) -> Result<String, String> {
    let refspec = format!("refs/heads/{branch}");
    let output = Command::new("git")
        .args(["ls-remote", url, &refspec])
        .output()
        .map_err(|e| format!("failed to run git ls-remote: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "git ls-remote {url} {refspec} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().next())
        .map(str::to_string)
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

    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);

    let mut all_ok = true;

    for sub in &submodules {
        let parent_sha = match git_rev_parse_submodule(&sub.path) {
            Ok(sha) => sha,
            Err(e) => {
                eprintln!("error: {e}");
                exit(2);
            }
        };

        let remote_sha = match git_ls_remote(&sub.url, &sub.branch) {
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
