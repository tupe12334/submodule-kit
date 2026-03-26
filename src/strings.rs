#![allow(dead_code)]

// ── file paths ────────────────────────────────────────────────────────────────
pub const GITMODULES_FILE: &str = ".gitmodules";

// ── .gitmodules parsing tokens ────────────────────────────────────────────────
pub const SUBMODULE_SECTION_PREFIX: &str = "[submodule \"";
pub const SUBMODULE_SECTION_SUFFIX: &str = "\"]";
pub const KEY_PATH: &str = "path = ";
pub const KEY_URL: &str = "url = ";
pub const KEY_BRANCH: &str = "branch = ";

// ── git refs ──────────────────────────────────────────────────────────────────
pub const REFS_HEADS_PREFIX: &str = "refs/heads/";

// ── status labels ─────────────────────────────────────────────────────────────
pub const STATUS_UP_TO_DATE: &str = "up-to-date";
pub const STATUS_BEHIND: &str = "behind";
pub const STATUS_POPULATED: &str = "populated";
pub const STATUS_MISSING: &str = "missing";
pub const LABEL_PARENT: &str = "parent";
pub const LABEL_REMOTE: &str = "remote";

// ── progress messages ─────────────────────────────────────────────────────────
pub const MSG_LISTING_SUBMODULES: &str = "Listing submodules...";

// ── error messages ────────────────────────────────────────────────────────────
pub fn err_read_gitmodules(e: &impl std::fmt::Display) -> String {
    format!("Failed to read .gitmodules: {e}")
}

pub fn err_missing_path(name: &str) -> String {
    format!("submodule '{name}' is missing 'path =' in .gitmodules")
}

pub fn err_missing_url(name: &str) -> String {
    format!("submodule '{name}' is missing 'url =' in .gitmodules")
}

pub fn err_missing_branch(path: &str) -> String {
    format!("submodule '{path}' is missing 'branch =' in .gitmodules")
}

pub fn err_open_index(e: &impl std::fmt::Display) -> String {
    format!("failed to open index: {e}")
}

pub fn err_not_in_index(path: &str) -> String {
    format!("submodule '{path}' not found in index")
}

pub fn err_create_remote(url: &str, e: &impl std::fmt::Display) -> String {
    format!("failed to create remote for {url}: {e}")
}

pub fn err_connect_remote(url: &str, e: &impl std::fmt::Display) -> String {
    format!("failed to connect to {url}: {e}")
}

pub fn err_list_refs(url: &str, e: &impl std::fmt::Display) -> String {
    format!("failed to list refs at {url}: {e}")
}

pub fn err_ref_not_found(refspec: &str, url: &str) -> String {
    format!("ref {refspec} not found at {url}")
}

pub fn err_open_repo(e: &impl std::fmt::Display) -> String {
    format!("failed to open git repository: {e}")
}
