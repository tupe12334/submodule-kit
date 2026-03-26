use crate::submodule::parse_gitmodules;

pub fn run() {
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
