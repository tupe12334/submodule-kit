# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2026-03-26

### Fixed
- `is all-up-to-date`: use `git ls-remote` subprocess for remote ref resolution, fixing hangs with SSH URLs

## [0.2.1] - 2026-03-26

### Fixed
- `is all-up-to-date`: SSH remote connections now use the SSH agent for authentication, fixing "authentication required but no callback set" errors

## [0.2.0] - 2026-03-26

### Changed
- Moved `list` command out of `is` module into its own `commands/list` module

## [0.1.0] - 2026-03-26

### Added
- `is clean` — check if all submodules have no uncommitted changes
- `is synced` — check if all submodules are in sync with their recorded commits
- `is on-branch` — check if all submodules are on the expected branch
- Multiple conditions support in a single `is` command
- `list` command to list all submodules
- MIT license
- Publishing support via `publish.sh` with `.env` token support

### Changed
- Split `is` conditions into a `conditions/` subfolder
- Moved inline tests to separate `_tests.rs` files
- Used shared `short()` helper across modules

### Internal
- Added cspell spell-check to pre-push hook
- Added clippy lints
- Added `generate-docs` command
- Validated README in pre-commit hook
