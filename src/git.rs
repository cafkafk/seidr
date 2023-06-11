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

use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::fs::canonicalize;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::{fs, process::Command};

/// Represents the config.toml file.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Config {
    pub repos: Vec<GitRepo>,
    pub links: Vec<Links>,
}

/// Contain fields for a single link.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Links {
    pub name: String,
    pub rx: String,
    pub tx: String,
}

/// Holds a single git repository and related fields.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct GitRepo {
    pub name: String,
    pub path: String,
    pub url: String,
    pub clone: bool,
}

fn handle_file_exists(selff: &Links, tx_path: &Path, rx_path: &Path) {
    match rx_path.read_link() {
        Ok(file) if file.canonicalize().unwrap() == tx_path.canonicalize().unwrap() => {
            debug!(
                "Linking {} -> {} failed: file already linked",
                &selff.tx, &selff.rx
            );
        }
        Ok(file) => {
            error!(
                "Linking {} -> {} failed: link to different file exists",
                &selff.tx, &selff.rx
            );
        }
        Err(error) => {
            error!("Linking {} -> {} failed: file exists", &selff.tx, &selff.rx);
        }
    }
}

impl Links {
    /// Creates a link from a file
    fn link(&self) {
        let tx_path: &Path = std::path::Path::new(&self.tx);
        let rx_path: &Path = std::path::Path::new(&self.rx);
        match rx_path.try_exists() {
            Ok(true) => handle_file_exists(&self, &tx_path, &rx_path),
            Ok(false) if rx_path.is_symlink() => {
                error!(
                    "Linking {} -> {} failed: broken symlink",
                    &self.tx, &self.rx
                );
            }
            Ok(false) => {
                symlink(&self.tx, &self.rx).expect("failed to create link");
            }
            Err(error) => {
                error!("Linking {} -> {} failed: {}", &self.tx, &self.rx, error);
            }
        };
    }
}

impl GitRepo {
    /// Clones the repository to its specified folder.
    fn clone(&self) {
        if self.clone {
            // TODO: check if &self.name already exists in dir
            let out = Command::new("git")
                .current_dir(&self.path)
                .arg("clone")
                .arg(&self.url)
                .arg(&self.name)
                .status()
                .expect("failed to add");
            info!("{out}");
        } else {
            info!("{} has clone set to false, not cloned", &self.name);
        }
    }
    /// Pulls the repository if able.
    fn pull(&self) {
        let out = Command::new("git")
            .current_dir(format!("{}{}", &self.path, &self.name))
            .arg("pull")
            .status()
            .expect("failed to pull");
        info!("{out}");
    }
    /// Adds all files in the repository.
    fn add_all(&self) {
        let out = Command::new("git")
            .current_dir(format!("{}{}", &self.path, &self.name))
            .arg("add")
            .arg(".")
            .status()
            .expect("failed to add");
        info!("{out}");
    }
    /// Tries to commit changes in the repository.
    #[allow(dead_code)]
    fn commit(&self) {
        let out = Command::new("git")
            .current_dir(format!("{}{}", &self.path, &self.name))
            .arg("commit")
            .status()
            .expect("failed to commit");
        info!("{out}");
    }
    /// Tries to commit changes with a message argument.
    fn commit_with_msg(&self, msg: &String) {
        let out = Command::new("git")
            .current_dir(format!("{}{}", &self.path, &self.name))
            .arg("commit")
            .arg("-m")
            .arg(msg)
            .status()
            .expect("failed to commit");
        info!("{out}");
    }
    /// Attempts to push the repository.
    fn push(&self) {
        let out = Command::new("git")
            .current_dir(format!("{}{}", &self.path, &self.name))
            .arg("push")
            .status()
            .expect("failed to push");
        info!("{out}");
    }
    /// Removes repository
    fn remove(&self) -> Result<(), std::io::Error> {
        // https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
        unimplemented!("This seems to easy to missuse/exploit");
        fs::remove_dir_all(format!("{}{}", &self.path, &self.name))
    }
}

impl Config {
    /* GIT RELATED */
    /// Reads the configuration toml from a path.
    pub fn new(path: &String) -> Self {
        debug!("initializing new Config struct");
        let yaml = fs::read_to_string(path).expect("Should have been able to read the file");
        debug!("deserialized yaml from config file");
        serde_yaml::from_str(&yaml).expect("Should have been able to deserialize yaml config")
    }
    /// Tries to pull all repositories, skips if fail.
    pub fn pull_all(&self) {
        debug!("exectuting pull_all");
        for r in self.repos.iter() {
            r.pull();
        }
    }
    /// Tries to clone all repositories, skips if fail.
    pub fn clone_all(&self) {
        debug!("exectuting clone_all");
        for r in self.repos.iter() {
            r.clone();
        }
    }
    /// Tries to add all work in all repositories, skips if fail.
    pub fn add_all(&self) {
        debug!("exectuting clone_all");
        for r in self.repos.iter() {
            r.add_all();
        }
    }
    /// Tries to commit all repositories one at a time, skips if fail.
    pub fn commit_all(&self) {
        debug!("exectuting clone_all");
        for r in self.repos.iter() {
            r.commit();
        }
    }
    /// Tries to commit all repositories with msg, skips if fail.
    pub fn commit_all_msg(&self, msg: &String) {
        debug!("exectuting clone_all");
        for r in self.repos.iter() {
            r.commit_with_msg(msg);
        }
    }
    /// Tries to pull, add all, commit with msg "quick commit", and push all
    /// repositories, skips if fail.
    pub fn quick(&self, msg: &String) {
        debug!("exectuting quick");
        for r in self.repos.iter() {
            r.pull();
            r.add_all();
            r.commit_with_msg(msg);
            r.push();
        }
    }

    /* LINK RELATED */
    /// Tries to link all repositories, skips if fail.
    pub fn link_all(&self) {
        debug!("exectuting link_all");
        for link in self.links.iter() {
            link.link();
        }
    }
}
