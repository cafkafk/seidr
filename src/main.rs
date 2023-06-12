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

extern crate log;
extern crate pretty_env_logger;

#[allow(unused)]
mod cli;
#[allow(unused)]
mod git;
#[allow(unused)]
mod utils;

use cli::{Args, Commands};
use git::Config;

use clap::Parser;
#[allow(unused)]
use log::{debug, error, info, trace, warn};

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    let config = Config::new(&args.config);
    match &args {
        args if args.license == true => println!("{}", utils::strings::INTERACTIVE_LICENSE),
        args if args.warranty == true => println!("{}", utils::strings::INTERACTIVE_WARRANTY),
        args if args.code_of_conduct == true => unimplemented!(),
        _ => (),
    }
    match &args.command {
        Some(Commands::Link { msg: _ }) => {
            config.link_all();
        }
        Some(Commands::Quick { msg }) => {
            config.quick(&msg.as_ref().get_or_insert(&"gg: quick commit".to_string()));
        }
        Some(Commands::Clone { msg: _ }) => {
            config.clone_all();
        }
        Some(Commands::Pull { msg: _ }) => {
            config.pull_all();
        }
        Some(Commands::Add { msg: _ }) => {
            config.add_all();
        }
        Some(Commands::Commit { msg: _ }) => {
            config.commit_all();
        }
        Some(Commands::CommitMsg { msg }) => {
            config.commit_all_msg(&msg.as_ref().unwrap());
        }
        None => (),
    }
    trace!("{:?}", config);
}

#[cfg(test)]
mod config {
    use crate::*;
    use git::GitRepo;
    use std::fs::File;
    use std::io::prelude::*;
    use utils::dir::current_dir;
    #[test]
    fn init_config() {
        let _config = Config {
            repos: vec![],
            links: vec![],
        };
    }
    #[test]
    fn init_config_populate() {
        let mut config = Config {
            repos: vec![],
            links: vec![],
        };
        for _ in 0..=5 {
            let repo = GitRepo {
                name: "test repo".to_string(),
                path: "/tmp".to_string(),
                url: "https://github.com/cafkafk/gg".to_string(),
                clone: false,
            };
            config.repos.push(repo);
        }
        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("{}", yaml);
    }
    #[test]
    fn read_config_populate() {
        const CONFIG_FILE: &str = "/tst/config.yaml";
        let config_path = current_dir() + CONFIG_FILE;
        let _config = Config::new(&config_path);
    }
    #[test]
    fn write_config() {
        const CONFIG_FILE: &str = "/tst/config.yaml";
        const CONFIG_TEST: &str = "/tst/test.yaml";
        let config_path = current_dir() + CONFIG_FILE;
        let config = Config::new(&config_path);

        let test_path = current_dir() + CONFIG_TEST;
        let mut file = File::create(&test_path).unwrap();
        let contents = serde_yaml::to_string(&config).unwrap();
        file.write(&contents.as_bytes()).unwrap();

        let test_config = Config::new(&test_path);
        assert_eq!(config, test_config);
    }
    #[test]
    fn read_and_verify_config() {
        const CONFIG_FILE: &str = "/tst/config.yaml";
        let config_path = current_dir() + CONFIG_FILE;
        let config = Config::new(&config_path);
        // FIXME This is unnecessarily terse
        {
            assert_eq!(config.repos[0].name, "gg");
            assert_eq!(config.repos[0].path, "/home/ces/.dots/");
            assert_eq!(config.repos[0].url, "git@github.com:cafkafk/gg.git");
            assert_eq!(config.repos[0].clone, true);
            assert_eq!(config.repos[1].name, "li");
            assert_eq!(config.repos[1].path, "/home/ces/org/src/git/");
            assert_eq!(config.repos[1].url, "git@github.com:cafkafk/li.git");
            assert_eq!(config.repos[1].clone, true);
            assert_eq!(config.repos[2].name, "qmk_firmware");
            assert_eq!(config.repos[2].path, "/home/ces/org/src/git/");
            assert_eq!(
                config.repos[2].url,
                "git@github.com:cafkafk/qmk_firmware.git"
            );
            assert_eq!(config.repos[2].clone, true);
            assert_eq!(config.repos[3].name, "starship");
            assert_eq!(config.repos[3].path, "/home/ces/org/src/git/");
            assert_eq!(
                config.repos[3].url,
                "https://github.com/starship/starship.git"
            );
            assert_eq!(config.repos[3].clone, true);
        }
        {
            assert_eq!(config.links[0].name, "gg");
            assert_eq!(config.links[0].rx, "/home/ces/.config/gg");
            assert_eq!(config.links[0].tx, "/home/ces/.dots/gg");
            assert_eq!(config.links[1].name, "starship");
            assert_eq!(config.links[1].rx, "/home/ces/.config/starship.toml");
            assert_eq!(config.links[1].tx, "/home/ces/.dots/starship.toml");
        }
    }
}

#[cfg(test)]
mod repo_actions {
    use crate::*;
    use git::GitRepo;
    use std::process::Command;
    use utils::dir::current_dir;
    #[test]
    fn test_repo_actions() {
        let test_repo_name: String = "test".to_string();
        let test_repo_dir: String = (current_dir() + "/tst/").to_string();
        let test_repo_url: String = "git@github.com:cafkafk/test.git".to_string();
        println!("{}", test_repo_dir);
        let mut config = Config {
            repos: vec![],
            links: vec![],
        };
        let repo = GitRepo {
            name: test_repo_name.to_owned(),
            path: test_repo_dir.to_owned(),
            url: test_repo_url.to_owned(),
            clone: true,
        };
        config.repos.push(repo);
        config.clone_all();
        config.pull_all();
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
