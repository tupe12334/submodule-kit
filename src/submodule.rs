use crate::strings;
use std::path::Path;

#[derive(Debug)]
pub struct SubmoduleInfo {
    pub path: String,
    pub url: String,
    pub branch: Option<String>,
}

/// List the repository's configured submodules via libgit2.
///
/// This delegates to libgit2's `Repository::submodules`, which parses
/// `.gitmodules` with the real git-config grammar (combined with the
/// repository config) instead of a hand-rolled string scan. As a result it
/// correctly handles whitespace around `=`, case-insensitive key names,
/// comment lines and quoted values that the previous parser mishandled.
///
/// # Errors
///
/// Returns an error if libgit2 cannot read the repository's submodule list.
pub fn submodules(repo: &git2::Repository) -> Result<Vec<SubmoduleInfo>, String> {
    let entries = repo
        .submodules()
        .map_err(|e| strings::err_read_submodules(&e))?;
    Ok(entries
        .iter()
        .map(|sm| SubmoduleInfo {
            path: sm.path().to_string_lossy().into_owned(),
            url: sm.url().unwrap_or_default().to_string(),
            branch: sm.branch().map(ToString::to_string),
        })
        .collect())
}

/// Resolve the commit id the parent repository's index records for `path`.
///
/// # Errors
///
/// Returns an error if the repository index cannot be opened, or if `path` has
/// no entry in the index.
pub fn git_rev_parse_submodule(repo: &git2::Repository, path: &str) -> Result<String, String> {
    let index = repo.index().map_err(|e| strings::err_open_index(&e))?;
    let entry = index
        .get_path(Path::new(path), 0)
        .ok_or_else(|| strings::err_not_in_index(path))?;
    Ok(entry.id.to_string())
}

/// Look up the remote commit id for `branch` at `url` via `git ls-remote`.
///
/// # Errors
///
/// Returns an error if the `git` process cannot be spawned, exits unsuccessfully
/// (for example, the remote is unreachable), or the requested branch ref is not
/// present on the remote.
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

#[must_use]
pub fn short(sha: &str) -> &str {
    &sha[..sha.len().min(7)]
}

#[cfg(test)]
#[path = "submodule_tests.rs"]
mod tests;
