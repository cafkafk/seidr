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
use std::collections::HashMap;
use std::fs::canonicalize;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::{fs, process::Command};

// why not make it O(log n) instead of a vec that's /only/ O(n)
// ...because premature optimization is the root of all evil!
#[derive(PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum RepoFlags {
    Push,
    Clone,
    // Pull, FIXME: could be interesting to implement
}

/// Represents the config.toml file.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Config {
    /// map of all categories
    ///
    /// Key should conceptually be seen as the name of the category.
    pub categories: HashMap<String, Category>,
    pub links: Vec<Links>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Category {
    pub flags: Vec<RepoFlags>, // FIXME: not implemented
    /// map of all categories
    ///
    /// Key should conceptually be seen as the name of the category.
    pub repos: HashMap<String, GitRepo>,
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
    pub flags: Vec<RepoFlags>,
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
            Ok(true) => handle_file_exists(self, tx_path, rx_path),
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
        if self.flags.contains(&RepoFlags::Clone) {
            // TODO: check if &self.name already exists in dir
            let out = Command::new("git")
                .current_dir(&self.path)
                .arg("clone")
                .arg(&self.url)
                .arg(&self.name)
                .status()
                .unwrap_or_else(|_| panic!("git repo failed to clone: {:?}", &self,));
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
            .unwrap_or_else(|_| panic!("git repo failed to pull: {:?}", &self,));
        info!("{out}");
    }
    /// Adds all files in the repository.
    fn add_all(&self) {
        if self.flags.contains(&RepoFlags::Push) {
            let out = Command::new("git")
                .current_dir(format!("{}{}", &self.path, &self.name))
                .arg("add")
                .arg(".")
                .status()
                .unwrap_or_else(|_| panic!("git repo failed to add: {:?}", &self,));
            info!("{out}");
        } else {
            info!("{} has clone set to false, not cloned", &self.name);
        }
    }
    /// Tries to commit changes in the repository.
    #[allow(dead_code)]
    fn commit(&self) {
        if self.flags.contains(&RepoFlags::Push) {
            let out = Command::new("git")
                .current_dir(format!("{}{}", &self.path, &self.name))
                .arg("commit")
                .status()
                .unwrap_or_else(|_| panic!("git repo failed to commit: {:?}", &self,));
            info!("{out}");
        } else {
            info!("{} has clone set to false, not cloned", &self.name);
        }
    }
    /// Tries to commit changes with a message argument.
    fn commit_with_msg(&self, msg: &String) {
        if self.flags.contains(&RepoFlags::Push) {
            let out = Command::new("git")
                .current_dir(format!("{}{}", &self.path, &self.name))
                .arg("commit")
                .arg("-m")
                .arg(msg)
                .status()
                .unwrap_or_else(|_| panic!("git repo failed to commit: {:?}", &self,));
            info!("{out}");
        } else {
            info!("{} has clone set to false, not cloned", &self.name);
        }
    }
    /// Attempts to push the repository.
    fn push(&self) {
        if self.flags.contains(&RepoFlags::Push) {
            let out = Command::new("git")
                .current_dir(format!("{}{}", &self.path, &self.name))
                .arg("push")
                .status()
                .unwrap_or_else(|_| panic!("git repo failed to push: {:?}", &self,));
            info!("{out}");
        } else {
            info!("{} has clone set to false, not cloned", &self.name);
        }
    }
    /// Removes repository
    fn remove(&self) -> Result<(), std::io::Error> {
        // https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
        unimplemented!("This seems to easy to missuse/exploit");
        // fs::remove_dir_all(format!("{}{}", &self.path, &self.name))
    }
}

impl Config {
    /* GIT RELATED */
    /// Reads the configuration toml from a path.
    pub fn new(path: &String) -> Self {
        debug!("initializing new Config struct");
        let yaml = fs::read_to_string(path).unwrap_or_else(|_| {
            panic!("Should have been able to read the file: path -> {:?}", path,)
        });
        debug!("deserialized yaml from config file");
        serde_yaml::from_str(&yaml).unwrap_or_else(|_| {
            panic!(
                "Should have been able to deserialize yaml config: path -> {:?}",
                path,
            )
        })
    }
    /// Runs associated function on all repos in config
    ///
    /// TODO: need to be made over a generic repo type
    fn on_all<F>(&self, f: F)
    where
        F: Fn(&GitRepo),
    {
        for (_, category) in self.categories.iter() {
            println!("{category:?}");
            for (_, repo) in category.repos.iter() {
                f(repo);
            }
        }
    }
    /// Tries to pull all repositories, skips if fail.
    pub fn pull_all(&self) {
        debug!("exectuting pull_all");
        self.on_all(|repo| {
            repo.pull();
        });
    }
    /// Tries to clone all repositories, skips if fail.
    pub fn clone_all(&self) {
        debug!("exectuting clone_all");
        self.on_all(|repo| {
            repo.clone();
        });
    }
    /// Tries to add all work in all repositories, skips if fail.
    pub fn add_all(&self) {
        debug!("exectuting clone_all");
        self.on_all(|repo| {
            repo.add_all();
        });
    }
    /// Tries to commit all repositories one at a time, skips if fail.
    pub fn commit_all(&self) {
        debug!("exectuting clone_all");
        self.on_all(|repo| {
            repo.commit();
        });
    }
    /// Tries to commit all repositories with msg, skips if fail.
    pub fn commit_all_msg(&self, msg: &String) {
        debug!("exectuting clone_all");
        self.on_all(|repo| {
            repo.commit_with_msg(msg);
        });
    }
    /// Tries to pull, add all, commit with msg "quick commit", and push all
    /// repositories, skips if fail.
    pub fn quick(&self, msg: &String) {
        debug!("exectuting quick");
        self.on_all(|repo| {
            repo.pull();
            repo.add_all();
            repo.commit_with_msg(msg);
            repo.push();
        });
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
