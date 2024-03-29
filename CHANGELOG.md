<!--
SPDX-FileCopyrightText: 2023 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: AGPL-3.0-only
-->

# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2023-07-07

### Bug Fixes

- Made categories with only links possible

### Features

- [**breaking**] Put links in categories

### Miscellaneous Tasks

- Filled out Cargo.toml
- Added test.yaml to gitignore
- Fixed up code, roadmap for bump

### Refactor

- Simple code quality changes

### Testing

- Refactored testing, added tests dir

## [0.1.2] - 2023-07-03

### Features

- Implemented quiet flag

### Miscellaneous Tasks

- Bump to v0.1.2

## [0.1.1] - 2023-07-03

### Bug Fixes

- Fixed help formatting

### Documentation

- Added asciinema demo

### Features

- Added no-emoji flag

### Miscellaneous Tasks

- Bump v0.1.0, housekeeping, #8 from cafkafk/dev
- Bump to v0.1.1

## [0.1.0] - 2023-07-03

### Documentation

- Changed roadmap
- Updated roadmap

### Features

- Implemented CoC flag
- Quiet flag
- Made SUCCESS/FAILURE emoji const
- Added flag no-emoji

### Miscellaneous Tasks

- Bump to 0.0.7 #7 from cafkafk/dev
- Bump v0.1.0, housekeeping

### Refactor

- Made code more idiomatic

## [0.0.7] - 2023-07-02

### Bug Fixes

- Changed config.yaml location
- Increased scope of push field
- Remove potentially destructive operaton
- Fixed mini-license typos
- Fixed testing with hashmap arch
- Spinner on all repoactions
- Fixed commit in quick
- [**breaking**] Fixed quick, fast messages
- Fixed commit with editor regression

### Documentation

- Architectural Overview
- Moved charts to doc/img
- Update image locations
- Moved ARCHITECTURE.md to doc/
- Added some documentation
- Added roadmap
- Added git cliff config

### Features

- Started flakification
- Added nix flake #5 
- [**breaking**] Add push field
- [**breaking**] Add repo flags
- [**breaking**] Implemented naive categories
- Started work on using spinners
- Added pull flag
- React to exit code of git
- Started adding multi instruction logic
- Added fast subcommand
- Add Commit, Add flags
- [**breaking**] Added Quick, Fast flags
- Made category flags optional
- Made categories.repo optional
- Made repo flags optional

### Miscellaneous Tasks

- Version bump to v0.0.3
- Moved install scripts to ./bin
- Merge 0.0.6
- Merge 0.0.6 #6 from cafkafk/dev
- Bump to 0.0.7

### Refactor

- Fixed various clippy errors
- Removed unused code from flake
- Improved GitRepo assoc. function debug
- Removed redundant line in Cargo.toml
- Created on_all for config struct
- Naive nested hashmap
- Generic refactor

### Security

- Removed atty dependency
- Removed atty dependency

### Testing

- Removed unused ./test dir

### WIP

- Mvp flake working

<!-- generated by git-cliff -->
