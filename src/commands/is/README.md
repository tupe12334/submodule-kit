# Command: `is`

Returns exit 0 (true) or exit 1 (false). Designed for scripting and CI.

```sh
submodule-kit is <condition>
```

---

## Conditions

### `all-up-to-date`

Checks whether every submodule's parent-recorded commit matches the tip of its remote branch.

```sh
submodule-kit is all-up-to-date
```

**Algorithm:**

1. Parse `.gitmodules`
   - If any submodule is missing `branch =` → **error** (do not silently skip or default)
2. For each submodule:
   - Get the commit the parent repo records: `git rev-parse :<path>`
   - Get the remote tip: `git ls-remote <url> refs/heads/<branch>`
   - Compare the two SHAs
3. Print all submodules regardless of status
4. Exit 0 if all match, exit 1 if any differ

**Output format:**

```text
libs/foo  up-to-date  abc1234
libs/bar  behind      parent=def5678  remote=9abcdef
libs/baz  up-to-date  789abcd
```

**Error cases** (distinct from exit 1 / "behind"):

| Situation | Behavior |
| --- | --- |
| Submodule missing `branch =` in `.gitmodules` | Error — abort |
| `git ls-remote` fails (network, bad URL, auth) | Error — abort |

**Open questions:**

- If `ls-remote` fails for one submodule: abort immediately, or collect all errors and print them together?
- Output format: tabular (above), JSON, or colored?
- SHA display length: full 40-char or short 7-char?

---

### `populated` _(planned)_

Checks whether all submodules have been initialized and cloned locally.

---

## Notes

- `is` is a namespace — each condition is a subcommand (`clap`)
- `all-up-to-date` does **not** require submodules to be locally cloned (uses `git rev-parse` on the parent tree and `git ls-remote` on the remote URL directly)
- Network is required for `all-up-to-date`; `populated` will be local-only
