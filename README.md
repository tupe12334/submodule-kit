# Command-Line Help for `submodule-kit`

This document contains the help content for the `submodule-kit` command-line program.

**Command Overview:**

* [`submodule-kit`↴](#submodule-kit)
* [`submodule-kit list`↴](#submodule-kit-list)
* [`submodule-kit is`↴](#submodule-kit-is)

## `submodule-kit`

A CLI toolkit for managing git submodules

**Usage:** `submodule-kit <COMMAND>`

###### **Subcommands:**

* `list` — List all submodules
* `is` — Check one or more conditions about submodules; exits 0 (all true) or 1 (any false)



## `submodule-kit list`

List all submodules

**Usage:** `submodule-kit list`



## `submodule-kit is`

Check one or more conditions about submodules; exits 0 (all true) or 1 (any false)

**Usage:** `submodule-kit is [CONDITIONS]...`

###### **Arguments:**

* `<CONDITIONS>`

  Possible values:
  - `all-up-to-date`:
    Check whether every submodule's parent-recorded commit matches the tip of its remote branch
  - `populated`:
    Check whether all submodules have been initialized and cloned locally
  - `clean`:
    Check whether all submodules have no uncommitted changes
  - `synced`:
    Check whether each submodule's locally checked-out commit matches what the parent records
  - `on-branch`:
    Check whether all submodules are checked out on their configured branch (not detached HEAD)




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

## License

MIT — see [LICENSE](LICENSE)
