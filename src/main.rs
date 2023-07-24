//    A Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow.
//    Copyright (C) 2023  Christina Sørensen <christina@cafkafk.com>
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see https://www.gnu.org/gpl-3.0.html.
//
//! A Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow.
//!
//! # What is?
//!
//! A Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow. Useful for dealing
//! with "dotfiles", and with git support as a first class feature. Configuration is
//! done throug a single yaml file, giving it a paradigm that should bring joy to
//! those that use declarative operating systems and package managers.
//!
//! Although this isn't really a case where it matters *that* much for performance,
//! being written in rust instead of e.g. /janky/ scripting languages does also mean
//! it is snappy and reliable, and the /extensive/ testing helps ensure regressions
//! aren't introduced.
//!
//! That said, we're in 0.0.Z, *here be dragons* for now.
#![feature(unsized_tuple_coercion)]

extern crate log;
extern crate pretty_env_logger;

#[allow(unused)]
mod cli;
#[allow(unused)]
mod git;
#[allow(unused)]
mod settings;
#[allow(unused)]
mod utils;

use cli::{Args, Commands, JumpCommands};
use git::Config;
use utils::strings::{FAST_COMMIT, QUICK_COMMIT};

use clap::Parser;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use std::sync::atomic::Ordering;

/// The main loop of the binary
///
/// Here, we handle parsing the configuration file, as well as matching commands
/// to the relavant operations.
fn main() {
    pretty_env_logger::init();
    let mut args = Args::parse();
    let config = Config::new(&args.config);

    // Input from -m flag is stored here, this is just used to construct the
    // persistent box
    let mut message_input: String = "".to_string();

    match &args {
        args if args.license => println!("{}", utils::strings::INTERACTIVE_LICENSE),
        args if args.warranty => println!("{}", utils::strings::INTERACTIVE_WARRANTY),
        args if args.code_of_conduct => println!("{}", utils::strings::INTERACTIVE_COC),
        args if args.quiet => settings::QUIET.store(true, Ordering::Relaxed),
        args if args.no_emoji => settings::EMOJIS.store(true, Ordering::Relaxed),
        args if args.message.is_some() => message_input = args.message.clone().unwrap(),
        _ => (),
    }

    let message = Box::leak(message_input.into_boxed_str());

    match &mut args.command {
        Some(Commands::Link {}) => {
            config.link_all();
        }
        // NOTE: This implements "sub-subcommand"-like matching on repository,
        // name, and additional data for a subcommand
        // TODO: generalize for reuse by all commands that operate on repo->name->msg
        //
        // What we want:
        // - gg quick
        // - gg quick category
        // - gg quick category repository
        // - gg quick -m "message"
        // - gg quick category -m "message"
        // - gg quick category repo -m "hi"
        //
        // What we are implementing:
        // - [x] gg quick
        // - [ ] gg quick category
        // - [ ] gg quick category repository
        // - [ ] gg quick category repository "stuff"
        //
        // Roadmap:
        // - [-] basic command parsing
        //   - [ ] lacks -m flag
        // - [ ] ability to run command on repos in category
        // - [ ] ability to run command on single repo
        Some(Commands::Quick { category, repo }) => match (&category, &repo) {
            // - gg quick
            (None, None) => {
                config.quick(message);
            }
            // - [ ] gg quick category
            (category, None) => {
                println!("{}", category.as_ref().unwrap());
                todo!();
            }
            (category, repo) => {
                println!("{} {}", category.as_ref().unwrap(), repo.as_ref().unwrap());
                todo!();
            } // // - [ ] gg quick category categorysitory "stuff"
              // (category, repo) => {
              //     println!("{} {}", category.as_ref().unwrap(), repo.as_ref().unwrap(),);
              //     todo!();
              // }
        },
        Some(Commands::Fast {}) => {
            config.fast(message);
        }
        Some(Commands::Clone {}) => {
            config.clone_all();
        }
        Some(Commands::Pull {}) => {
            config.pull_all();
        }
        Some(Commands::Add {}) => {
            config.add_all();
        }
        Some(Commands::Commit {}) => {
            config.commit_all();
        }
        Some(Commands::CommitMsg {}) => {
            config.commit_all_msg(message);
        }
        Some(Commands::Jump(cmd)) => match cmd {
            JumpCommands::Repo { category, name } => {
                config.get_repo(category, name, |repo| {
                    println!(
                        "{}{}",
                        repo.path.as_ref().unwrap(),
                        repo.name.as_ref().unwrap()
                    );
                });
            }
            JumpCommands::Link { category, name } => {
                config.get_link(category, name, |link| println!("{}", link.tx));
            }
        },
        None => (),
    }
    trace!("{:?}", config);
}

#[cfg(test)]
mod config {
    use crate::*;
    use git::RepoFlags::{Clone, Push};
    use git::{Category, Repo};
    use relative_path::RelativePath;
    use std::collections::HashMap;
    use std::env::current_dir;
    use std::fs::File;
    use std::io::prelude::*;
    #[test]
    fn init_config() {
        let _config = Config {
            categories: HashMap::new(),
        };
    }
    #[test]
    fn init_config_populate() {
        let default_category = Category {
            flags: Some(vec![]),
            repos: Some(HashMap::new()),
            links: Some(HashMap::new()),
        };
        let mut config = Config {
            categories: HashMap::new(),
        };
        config
            .categories
            .insert(format!("{}", 0).to_string(), default_category);
        for i in 0..=5 {
            config
                .categories
                .get_mut(&format!("{}", 0).to_string())
                .expect("category not found")
                .repos
                .as_mut()
                .expect("failed to get repo")
                .insert(
                    format!("{}", i).to_string(),
                    Repo {
                        name: Some("test repo".to_string()),
                        path: Some("/tmp".to_string()),
                        url: Some("https://github.com/cafkafk/gg".to_string()),
                        flags: Some(vec![Clone, Push]),
                        kind: None,
                    },
                );
        }
    }
    #[test]
    fn read_config_populate() {
        let _config = Config::new(&RelativePath::new("./src/test/config.yaml").to_string());
    }
    #[test]
    fn write_config() {
        let root = current_dir().expect("failed to get current dir");
        let config = Config::new(
            &RelativePath::new("./src/test/config.yaml")
                .to_logical_path(&root)
                .into_os_string()
                .into_string()
                .expect("failed to turn config into string"),
        );

        let mut test_file = File::create(
            RelativePath::new("./src/test/test.yaml")
                .to_logical_path(&root)
                .into_os_string()
                .into_string()
                .expect("failed to turn config into string"),
        )
        .expect("failed to create test file");
        let contents = serde_yaml::to_string(&config).expect("failed to turn config into string");
        test_file
            .write_all(contents.as_bytes())
            .expect("failed to write contents of config into file");

        let test_config = Config::new(&RelativePath::new("./src/test/test.yaml").to_string());
        assert_eq!(config, test_config);
    }
    #[allow(dead_code)]
    fn get_category<'cat>(config: &'cat Config, name: &'cat str) -> &'cat Category {
        config.categories.get(name).expect("failed to get category")
    }
    #[test]
    fn is_config_readable() {
        let root = current_dir().expect("failed to get current dir");
        let config = Config::new(
            &RelativePath::new("./src/test/config.yaml")
                .to_logical_path(root)
                .into_os_string()
                .into_string()
                .expect("failed to turnn config into string"),
        );

        let _flags = vec![Clone, Push];
        // NOTE not very extensive
        #[allow(clippy::bool_assert_comparison)]
        {
            (&config).get_repo("config", "qmk_firmware", |repo| {
                assert_eq!(repo.name.as_ref().unwrap(), "qmk_firmware");
                assert_eq!(repo.path.as_ref().unwrap(), "/home/ces/org/src/git/");
                assert_eq!(
                    repo.url.as_ref().unwrap(),
                    "git@github.com:cafkafk/qmk_firmware.git"
                );
            });
            (&config).get_link("stuff", "gg", |link| {
                assert_eq!(link.name, "gg");
                assert_eq!(link.tx, "/home/ces/.dots/gg");
                assert_eq!(link.rx, "/home/ces/.config/gg");
            });
        }
    }
}

/* FIXME Unable to test with networking inside flake
#[cfg(test)]
mod repo_actions {
    use crate::*;
    use git::Repo;
    use relative_path::RelativePath;
    use std::env::current_dir;
    use std::process::Command;
    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_repo_actions() {
        let test_repo_name: String = "test".to_string();
        let root = current_dir().unwrap();
        let test_repo_dir: String = RelativePath::new("./src/test")
            .to_logical_path(&root)
            .into_os_string()
            .into_string()
            .unwrap();
        let test_repo_url: String = "git@github.com:cafkafk/test.git".to_string();
        println!("{}", test_repo_dir);
        let mut config = Config {
            repos: vec![],
            links: vec![],
        };
        let repo = Repo {
            name: test_repo_name.to_owned(),
            path: test_repo_dir.to_owned(),
            url: test_repo_url.to_owned(),
            clone: true,
        };
        config.repos.push(repo);
        // BUG FIXME can't do this in flake
        // should have a good alternative
        // config.clone_all();
        // config.pull_all();
        for r in config.repos.iter() {
            Command::new("touch")
                .current_dir(&(r.path.to_owned() + &r.name))
                .arg("test")
                .status()
                .expect("failed to create test file");
        }
        config.add_all();
        config.commit_all_msg(&"test".to_string());
    }
}
*/
