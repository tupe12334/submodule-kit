use super::*;

#[test]
fn err_read_gitmodules_contains_error() {
    let msg = err_read_gitmodules(&"disk error");
    assert!(msg.contains("disk error"));
    assert!(msg.contains(".gitmodules"));
}

#[test]
fn err_missing_path_contains_name() {
    let msg = err_missing_path("mymod");
    assert!(msg.contains("mymod"));
    assert!(msg.contains("path"));
}

#[test]
fn err_missing_url_contains_name() {
    let msg = err_missing_url("mymod");
    assert!(msg.contains("mymod"));
    assert!(msg.contains("url"));
}

#[test]
fn err_missing_branch_contains_path() {
    let msg = err_missing_branch("mypath");
    assert!(msg.contains("mypath"));
    assert!(msg.contains("branch"));
}

#[test]
fn err_open_index_contains_error() {
    let msg = err_open_index(&"io error");
    assert!(msg.contains("io error"));
    assert!(msg.contains("index"));
}

#[test]
fn err_not_in_index_contains_path() {
    let msg = err_not_in_index("mypath");
    assert!(msg.contains("mypath"));
    assert!(msg.contains("index"));
}

#[test]
fn err_create_remote_contains_url_and_error() {
    let msg = err_create_remote("https://example.com", &"net error");
    assert!(msg.contains("https://example.com"));
    assert!(msg.contains("net error"));
}

#[test]
fn err_connect_remote_contains_url_and_error() {
    let msg = err_connect_remote("https://example.com", &"timeout");
    assert!(msg.contains("https://example.com"));
    assert!(msg.contains("timeout"));
}

#[test]
fn err_list_refs_contains_url_and_error() {
    let msg = err_list_refs("https://example.com", &"refused");
    assert!(msg.contains("https://example.com"));
    assert!(msg.contains("refused"));
}

#[test]
fn err_ref_not_found_contains_refspec_and_url() {
    let msg = err_ref_not_found("refs/heads/main", "https://example.com");
    assert!(msg.contains("refs/heads/main"));
    assert!(msg.contains("https://example.com"));
}

#[test]
fn err_open_repo_contains_error() {
    let msg = err_open_repo(&"not a repo");
    assert!(msg.contains("not a repo"));
    assert!(msg.contains("repository"));
}

#[test]
fn err_open_submodule_contains_path_and_error() {
    let msg = err_open_submodule("mymod", &"missing");
    assert!(msg.contains("mymod"));
    assert!(msg.contains("missing"));
}

#[test]
fn err_get_status_contains_path_and_error() {
    let msg = err_get_status("mymod", &"locked");
    assert!(msg.contains("mymod"));
    assert!(msg.contains("locked"));
}

#[test]
fn err_read_head_contains_path_and_error() {
    let msg = err_read_head("mymod", &"unborn");
    assert!(msg.contains("mymod"));
    assert!(msg.contains("unborn"));
}
