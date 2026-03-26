    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/submodule-kit generate-docs`
# Command-Line Help for `submodule-kit`

This document contains the help content for the `submodule-kit` command-line program.

**Command Overview:**

* [`submodule-kit`↴](#submodule-kit)
* [`submodule-kit list`↴](#submodule-kit-list)
* [`submodule-kit is`↴](#submodule-kit-is)
* [`submodule-kit is all-up-to-date`↴](#submodule-kit-is-all-up-to-date)
* [`submodule-kit is populated`↴](#submodule-kit-is-populated)
* [`submodule-kit is clean`↴](#submodule-kit-is-clean)
* [`submodule-kit is synced`↴](#submodule-kit-is-synced)
* [`submodule-kit is on-branch`↴](#submodule-kit-is-on-branch)

## `submodule-kit`

A CLI toolkit for managing git submodules

**Usage:** `submodule-kit <COMMAND>`

###### **Subcommands:**

* `list` — List all submodules
* `is` — Check a condition about submodules; exits 0 (true) or 1 (false)



## `submodule-kit list`

List all submodules

**Usage:** `submodule-kit list`



## `submodule-kit is`

Check a condition about submodules; exits 0 (true) or 1 (false)

**Usage:** `submodule-kit is <COMMAND>`

###### **Subcommands:**

* `all-up-to-date` — Check whether every submodule's parent-recorded commit matches the tip of its remote branch
* `populated` — Check whether all submodules have been initialized and cloned locally
* `clean` — Check whether all submodules have no uncommitted changes
* `synced` — Check whether each submodule's locally checked-out commit matches what the parent records
* `on-branch` — Check whether all submodules are checked out on their configured branch (not detached HEAD)



## `submodule-kit is all-up-to-date`

Check whether every submodule's parent-recorded commit matches the tip of its remote branch

**Usage:** `submodule-kit is all-up-to-date`



## `submodule-kit is populated`

Check whether all submodules have been initialized and cloned locally

**Usage:** `submodule-kit is populated`



## `submodule-kit is clean`

Check whether all submodules have no uncommitted changes

**Usage:** `submodule-kit is clean`



## `submodule-kit is synced`

Check whether each submodule's locally checked-out commit matches what the parent records

**Usage:** `submodule-kit is synced`



## `submodule-kit is on-branch`

Check whether all submodules are checked out on their configured branch (not detached HEAD)

**Usage:** `submodule-kit is on-branch`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
