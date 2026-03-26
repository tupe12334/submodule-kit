use crate::strings;
use clap::ValueEnum;
use std::fs;
use std::path::Path;

mod conditions;

#[derive(ValueEnum, Clone)]
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

pub fn list() {
    let submodules = match parse_gitmodules() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    };

    let col_width = submodules.iter().map(|s| s.path.len()).max().unwrap_or(0);
    for sub in &submodules {
        match &sub.branch {
            Some(branch) => println!("{:<col_width$}  {}  (branch: {branch})", sub.path, sub.url),
            None => println!("{:<col_width$}  {}", sub.path, sub.url),
        }
    }
}

pub fn run(conditions: Vec<IsCondition>) {
    let all_ok = conditions.into_iter().fold(true, |ok, condition| {
        let passed = match condition {
            IsCondition::AllUpToDate => conditions::all_up_to_date(),
            IsCondition::Populated => conditions::populated(),
            IsCondition::Clean => conditions::clean(),
            IsCondition::Synced => conditions::synced(),
            IsCondition::OnBranch => conditions::on_branch(),
        };
        ok && passed
    });
    if !all_ok {
        std::process::exit(1);
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
