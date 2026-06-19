use clap::ValueEnum;

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

/// Evaluate each requested condition and report whether they all hold.
///
/// # Errors
///
/// Returns an error if any individual condition check fails (for example, when
/// git state cannot be inspected).
pub fn run(conditions: Vec<IsCondition>) -> Result<bool, String> {
    let mut all_ok = true;
    for condition in conditions {
        let passed = match condition {
            IsCondition::AllUpToDate => conditions::all_up_to_date()?,
            IsCondition::Populated => conditions::populated()?,
            IsCondition::Clean => conditions::clean()?,
            IsCondition::Synced => conditions::synced()?,
            IsCondition::OnBranch => conditions::on_branch()?,
        };
        all_ok = all_ok && passed;
    }
    Ok(all_ok)
}

#[cfg(test)]
pub(crate) static CWD_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
