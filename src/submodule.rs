use crate::strings;
use std::path::Path;

#[derive(Debug)]
pub struct SubmoduleInfo {
    pub path: String,
    pub url: String,
    pub branch: Option<String>,
}

pub fn parse_gitmodules_str(content: &str) -> Result<Vec<SubmoduleInfo>, String> {
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

pub fn parse_gitmodules() -> Result<Vec<SubmoduleInfo>, String> {
    let content = std::fs::read_to_string(strings::GITMODULES_FILE)
        .map_err(|e| strings::err_read_gitmodules(&e))?;
    parse_gitmodules_str(&content)
}

pub fn git_rev_parse_submodule(repo: &git2::Repository, path: &str) -> Result<String, String> {
    let index = repo.index().map_err(|e| strings::err_open_index(&e))?;
    let entry = index
        .get_path(Path::new(path), 0)
        .ok_or_else(|| strings::err_not_in_index(path))?;
    Ok(entry.id.to_string())
}

pub fn git_ls_remote(_repo: &git2::Repository, url: &str, branch: &str) -> Result<String, String> {
    let refspec = format!("{}{branch}", strings::REFS_HEADS_PREFIX);
    let output = std::process::Command::new("git")
        .args(["ls-remote", url, &refspec])
        .output()
        .map_err(|e| strings::err_connect_remote(url, &e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(strings::err_connect_remote(url, &stderr.trim()));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find(|line| line.ends_with(&refspec))
        .and_then(|line| line.split_whitespace().next())
        .map(ToString::to_string)
        .ok_or_else(|| strings::err_ref_not_found(&refspec, url))
}

pub fn short(sha: &str) -> &str {
    &sha[..sha.len().min(7)]
}

#[cfg(test)]
#[path = "submodule_tests.rs"]
mod tests;
