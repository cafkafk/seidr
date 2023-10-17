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
//! Git repositories

use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::collections::HashMap;
use std::fs::canonicalize;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::{fmt, fs, process::Command};

use crate::settings;
use crate::utils::strings::{failure_str, success_str};

/// An enum containing flags that change behaviour of repos and categories
#[derive(PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum RepoFlags {
    /// If clone is set, the repository should respond to the clone subcommand
    Clone,
    /// If pull is set, the repository should respond to the pull subcommand
    Pull,
    /// If add is set, the repository should respond to the add subcommand
    Add,
    /// If commit is set, the repository should respond to the commit subcommand
    Commit,
    /// If push is set, the repository should respond to the push subcommand
    Push,
    /// If push is set, the repository should respond to the Qucik subcommand
    ///
    /// This is a shortcut for Add, Commit, Push
    Quick,
    /// If push is set, the repository should respond to the Fast and Qucik  subcommand
    ///
    /// This is a shortcut for Pull, Add, Commit, Push
    Fast,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum RepoKinds {
    GitRepo,
    GitHubRepo,
    GitLabRepo,
    GiteaRepo,
    UrlRepo,
    Link,
}

/// Represents the config.toml file.
///
/// For diagrams of the underlying architecture, consult ARCHITECHTURE.md
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Config {
    /// map of all categories
    ///
    /// Key should conceptually be seen as the name of the category.
    pub categories: HashMap<String, Category>,
}

/// Represents a category of repositories
///
/// This allows you to organize your repositories into categories
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<RepoFlags>>, // FIXME: not implemented
    /// map of all repos in category
    ///
    /// Key should conceptually be seen as the name of the category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos: Option<HashMap<String, Repo>>,

    /// map of all links in category
    ///
    /// Key should conceptually be seen as the name of the category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, Link>>,
}

/// Contain fields for a single link.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The name of the link
    pub name: String,
    pub rx: String,
    pub tx: String,
}

/// Holds a single git repository and related fields.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Repo {
    pub name: Option<String>,
    pub path: Option<String>,
    pub url: Option<String>,
    // TODO: make default a standard GitRepo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<RepoKinds>, // FIXME: not implemented
    pub flags: Option<Vec<RepoFlags>>,
}

/// Represents a single operation on a repository
pub struct SeriesItem<'series> {
    /// The string to be displayed to the user
    pub operation: &'series str,
    /// The closure representing the actual operation
    pub closure: Box<dyn Fn(&Repo) -> (bool)>,
}

#[derive(Debug)]
pub enum LinkError {
    AlreadyLinked(String, String),
    DifferentLink(String, String),
    FileExists(String, String),
    BrokenSymlinkExists(String, String),
    FailedCreatingLink(String, String),
    IoError(std::io::Error),
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::AlreadyLinked(tx, rx) => {
                write!(f, "Linking {tx} -> {rx} failed: file already linked")
            }
            LinkError::DifferentLink(tx, rx) => write!(
                f,
                "Linking {tx} -> {rx} failed: link to different file exists"
            ),
            LinkError::FileExists(tx, rx) => write!(f, "Linking {tx} -> {rx} failed: file exists"),
            LinkError::BrokenSymlinkExists(tx, rx) => {
                write!(f, "Linking {tx} -> {rx} failed: broken symlink")
            }
            LinkError::FailedCreatingLink(tx, rx) => write!(f, "Linking {tx} -> {rx} failed"),
            LinkError::IoError(err) => write!(f, "IO Error: {err}"),
        }
    }
}

impl std::error::Error for LinkError {
    // TODO: I'm too tired to implement soucce... yawn, eepy
    // fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    //     match self {
    //         LinkError::AlreadyLinked(tx, rx) => Some(rx),
    //         LinkError::DifferentLink(tx, rx) => Some(rx),
    //         LinkError::FileExists(tx, rx) => Some(rx),
    //     }
    // }
}

impl From<std::io::Error> for LinkError {
    fn from(err: std::io::Error) -> LinkError {
        LinkError::IoError(err)
    }
}

fn handle_file_exists(selff: &Link, tx_path: &Path, rx_path: &Path) -> Result<bool, LinkError> {
    match rx_path.read_link() {
        Ok(file)
            if file.canonicalize().expect("failed to canonicalize file")
                == tx_path.canonicalize().expect("failed to canonicalize path") =>
        {
            Err(LinkError::AlreadyLinked(
                tx_path.to_string_lossy().to_string(),
                rx_path.to_string_lossy().to_string(),
            ))
        }
        Ok(file) => Err(LinkError::DifferentLink(
            tx_path.to_string_lossy().to_string(),
            rx_path.to_string_lossy().to_string(),
        )),
        Err(error) => Err(LinkError::FileExists(
            tx_path.to_string_lossy().to_string(),
            rx_path.to_string_lossy().to_string(),
        )),
    }
}

impl Link {
    /// Creates the link from the link struct
    pub fn link(&self) -> Result<bool, LinkError> {
        let tx_path: &Path = std::path::Path::new(&self.tx);
        let rx_path: &Path = std::path::Path::new(&self.rx);
        match rx_path.try_exists() {
            // TODO: unwrap defeats the purpose here.
            Ok(true) => handle_file_exists(self, tx_path, rx_path),
            Ok(false) if rx_path.is_symlink() => Err(LinkError::FileExists(
                tx_path.to_string_lossy().to_string(),
                rx_path.to_string_lossy().to_string(),
            )),
            Ok(false) => {
                symlink(&self.tx, &self.rx)?;
                Ok(true)
            }
            Err(error) => Err(LinkError::FailedCreatingLink(
                tx_path.to_string_lossy().to_string(),
                rx_path.to_string_lossy().to_string(),
            )),
        }
    }
}

impl Repo {
    /// Clones the repository to its specified folder.
    pub fn clone(&self) -> bool {
        if self
            .flags
            .as_ref()
            .expect("failed to unwrap flags")
            .contains(&RepoFlags::Clone)
        {
            // TODO: check if &self.name.as_ref() already exists in dir
            let output = Command::new("git")
                .current_dir(self.path.as_ref().unwrap())
                .arg("clone")
                .arg(self.url.as_ref().unwrap())
                .arg(self.name.as_ref().unwrap())
                .output()
                .unwrap_or_else(|_| panic!("git repo failed to clone: {:?}", &self,));
            output.status.success()
        } else {
            info!(
                "{} has clone set to false, not cloned",
                &self.name.as_ref().unwrap()
            );
            false
        }
    }
    /// Pulls the repository if able.
    pub fn pull(&self) -> bool {
        if self
            .flags
            .as_ref()
            .expect("failed to unwrap flags")
            .iter()
            .any(|s| s == &RepoFlags::Pull || s == &RepoFlags::Fast)
        {
            let output = Command::new("git")
                .current_dir(format!(
                    "{}{}",
                    &self.path.as_ref().unwrap(),
                    &self.name.as_ref().unwrap()
                ))
                .arg("pull")
                .output()
                .unwrap_or_else(|_| panic!("git repo failed to pull: {:?}", &self,));
            output.status.success()
        } else {
            info!(
                "{} has clone set to false, not pulled",
                &self.name.as_ref().unwrap()
            );
            false
        }
    }
    /// Adds all files in the repository.
    pub fn add_all(&self) -> bool {
        if self
            .flags
            .as_ref()
            .expect("failed to unwrap flags")
            .iter()
            .any(|s| s == &RepoFlags::Add || s == &RepoFlags::Quick || s == &RepoFlags::Fast)
        {
            let output = Command::new("git")
                .current_dir(format!(
                    "{}{}",
                    &self.path.as_ref().unwrap(),
                    &self.name.as_ref().unwrap()
                ))
                .arg("add")
                .arg(".")
                .output()
                .unwrap_or_else(|_| panic!("git repo failed to add: {:?}", &self,));
            output.status.success()
        } else {
            info!(
                "{} has clone set to false, not cloned",
                &self.name.as_ref().unwrap()
            );
            false
        }
    }
    /// Tries to commit changes in the repository.
    ///
    /// # Development
    ///
    /// - FIXME: this prints extra information to terminal this is because we
    /// use status() instead of output(), as that makes using the native editor
    /// easy
    #[allow(dead_code)]
    pub fn commit(&self) -> bool {
        if self
            .flags
            .as_ref()
            .expect("failed to unwrap flags")
            .iter()
            .any(|s| s == &RepoFlags::Commit || s == &RepoFlags::Quick || s == &RepoFlags::Fast)
        {
            let status = Command::new("git")
                .current_dir(format!(
                    "{}{}",
                    &self.path.as_ref().unwrap(),
                    &self.name.as_ref().unwrap()
                ))
                .arg("commit")
                .status()
                .unwrap_or_else(|_| panic!("git repo failed to commit: {:?}", &self,));
            status.success()
        } else {
            info!(
                "{} has push set to false, not cloned",
                &self.name.as_ref().unwrap()
            );
            false
        }
    }
    /// Tries to commit changes with a message argument.
    pub fn commit_with_msg(&self, msg: &str) -> bool {
        if self
            .flags
            .as_ref()
            .expect("failed to unwrap flags")
            .iter()
            .any(|s| s == &RepoFlags::Commit || s == &RepoFlags::Quick || s == &RepoFlags::Fast)
        {
            let output = Command::new("git")
                .current_dir(format!(
                    "{}{}",
                    &self.path.as_ref().unwrap(),
                    &self.name.as_ref().unwrap()
                ))
                .arg("commit")
                .arg("-m")
                .arg(msg)
                .output()
                .unwrap_or_else(|_| panic!("git repo failed to commit: {:?}", &self,));
            output.status.success()
        } else {
            info!(
                "{} has clone set to false, not cloned",
                &self.name.as_ref().unwrap()
            );
            false
        }
    }
    /// Attempts to push the repository.
    pub fn push(&self) -> bool {
        if self
            .flags
            .as_ref()
            .expect("failed to unwrap flags")
            .iter()
            .any(|s| s == &RepoFlags::Push || s == &RepoFlags::Quick || s == &RepoFlags::Fast)
        {
            let output = Command::new("git")
                .current_dir(format!(
                    "{}{}",
                    &self.path.as_ref().unwrap(),
                    &self.name.as_ref().unwrap()
                ))
                .arg("push")
                .output()
                .unwrap_or_else(|_| panic!("git repo failed to push: {:?}", &self,));
            output.status.success()
        } else {
            info!(
                "{} has clone set to false, not cloned",
                &self.name.as_ref().unwrap()
            );
            false
        }
    }
    /// Removes a repository (not implemented)
    ///
    /// Kept here as a reminder that we probably shouldn't do this
    fn remove() -> Result<(), std::io::Error> {
        // https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
        unimplemented!("This seems to easy to missuse/exploit");
        // fs::remove_dir_all(format!("{}{}", &self.path.as_ref(), &self.name.as_ref()))
    }
    fn check_is_valid_gitrepo(&self) -> bool {
        if (self.name.is_none()) {
            eprintln!("{:?} must have name: <string>", self.kind);
            return false;
        }
        if (self.path.is_none()) {
            eprintln!("{:?} must have path: <string>", self.kind);
            return false;
        }
        if (self.url.is_none()) {
            eprintln!("{:?} must have url: <string>", self.kind);
            return false;
        }
        assert!(self.name.is_some());
        assert!(self.path.is_some());
        assert!(self.url.is_some());
        true
    }
    fn check_is_valid_githubrepo(&self) -> bool {
        todo!();
    }
    fn check_is_valid_gitlabrepo(&self) -> bool {
        todo!();
    }
    fn check_is_valid_gitearepo(&self) -> bool {
        todo!();
    }
    fn check_is_valid_urlrepo(&self) -> bool {
        todo!();
    }
    fn check_is_valid_link(&self) -> bool {
        todo!();
    }
    /// Check if Repo is a valid instance of its kind
    pub fn is_valid_kind(&self) -> bool {
        use RepoKinds::*;
        match &self.kind {
            Some(GitRepo) => self.check_is_valid_gitrepo(),
            Some(GitHubRepo) => self.check_is_valid_githubrepo(),
            Some(GitLabRepo) => self.check_is_valid_gitlabrepo(),
            Some(GiteaRepo) => self.check_is_valid_gitearepo(),
            Some(UrlRepo) => self.check_is_valid_urlrepo(),
            Some(Link) => self.check_is_valid_link(),
            Some(kind) => {
                panic!("kind {kind:?} not implemented");
                false
            }
            None => {
                println!("unknown kind {:?}", self.kind);
                false
            }
        }
    }
}

/// run_series runs a closure series on a Config struct
///
/// # Examples
///
///
/// ```
/// use seidr::git;
/// use seidr::git::Repo;
/// use seidr::git::Config;
/// use std::env::current_dir;
/// use seidr::git::SeriesItem;
/// use relative_path::RelativePath;
///
/// let root = current_dir().expect("failed to get current dir");
/// let config = Config::new(
///     &RelativePath::new("./src/test/config.yaml")
///         .to_logical_path(root)
///         .into_os_string()
///         .into_string()
///         .expect("failed to turnn config into string"),
/// );
///
/// let series: Vec<SeriesItem> = vec![
///     SeriesItem {
///         operation: "pull",
///         closure: Box::new(move |repo: &Repo| repo.pull()),
///     },
///     SeriesItem {
///         operation: "add",
///         closure: Box::new(move |repo: &Repo| repo.add_all()),
///     },
///     SeriesItem {
///         operation: "commit",
///         closure: Box::new(move |repo: &Repo| repo.commit()),
///     },
///     SeriesItem {
///         operation: "push",
///         closure: Box::new(move |repo: &Repo| repo.push()),
///     },
/// ];
///
/// // If we don't care if the series steps fail
/// run_series!(config, series);
///
/// // If we want to skip repo as soon as a step fails
/// run_series!(config, series, true);
/// ```
#[macro_export]
macro_rules! run_series {
    ($conf:ident, $closures:ident) => {
        $conf.all_on_all($closures, false);
    };
    ($conf:ident, $closures:ident, $stop_on_err:tt) => {
        $conf.all_on_all($closures, $stop_on_err);
    };
}

impl Config {
    /// Loads the configuration toml from a path in to the Config struct.
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
    /// NOTE: currently unused
    fn on_all<F>(&self, f: F)
    where
        F: Fn(&Repo),
    {
        for category in self.categories.values() {
            for (_, repo) in category.repos.as_ref().expect("failed to get repos").iter() {
                f(repo);
            }
        }
    }
    // /// Runs associated function on all repos in config
    // fn on_all_spinner<F>(&self, op: &str, f: F)
    // where
    //     F: Fn(&Repo) -> bool,
    // {
    //     for category in self.categories.values() {
    //         for (_, repo) in category.repos.as_ref().expect("failed to get repos").iter() {
    //             if !settings::QUIET.load(std::sync::atomic::Ordering::Relaxed) {
    //                 let mut sp = Spinner::new(Spinners::Dots10, format!("{}: {}", repo.name.as_ref(), op));
    //                 if f(repo) {
    //                     sp.stop_and_persist(success_str(), format!("{}: {}", repo.name.as_ref(), op));
    //                 } else {
    //                     sp.stop_and_persist(failure_str(), format!("{}: {}", repo.name.as_ref(), op));
    //                 }
    //             } else {
    //                 f(repo);
    //             }
    //         }
    //     }
    // }
    /// Runs associated function on all repos in config
    fn on_all_repos_spinner<F>(&self, op: &str, f: F)
    where
        F: Fn(&Repo) -> bool,
    {
        for category in self.categories.values() {
            match category.repos.as_ref() {
                Some(repos) => {
                    for repo in repos.values() {
                        if !settings::QUIET.load(std::sync::atomic::Ordering::Relaxed) {
                            let mut sp = Spinner::new(
                                Spinners::Dots10,
                                format!("{}: {}", repo.name.as_ref().unwrap(), op),
                            );
                            if f(repo) {
                                sp.stop_and_persist(
                                    success_str(),
                                    format!("{}: {}", repo.name.as_ref().unwrap(), op),
                                );
                            } else {
                                sp.stop_and_persist(
                                    failure_str(),
                                    format!("{}: {}", repo.name.as_ref().unwrap(), op),
                                );
                            }
                        } else {
                            f(repo);
                        }
                    }
                }
                None => continue,
            };
        }
    }
    /// Runs associated function on all links in config
    fn on_all_links_spinner<F>(&self, op: &str, f: F)
    where
        F: Fn(&Link) -> Result<bool, LinkError>,
    {
        for category in self.categories.values() {
            match category.links.as_ref() {
                Some(links) => {
                    for link in links.values() {
                        if !settings::QUIET.load(std::sync::atomic::Ordering::Relaxed) {
                            let mut sp =
                                Spinner::new(Spinners::Dots10, format!("{}: {}", link.name, op));
                            match f(link) {
                                Err(e @ LinkError::AlreadyLinked(_, _)) => {
                                    sp.stop_and_persist(success_str(), format!("{e}"))
                                }
                                Err(e @ LinkError::DifferentLink(_, _)) => {
                                    sp.stop_and_persist(failure_str(), format!("{e}"))
                                }
                                Err(e @ LinkError::FileExists(_, _)) => {
                                    sp.stop_and_persist(failure_str(), format!("{e}"))
                                }
                                Err(e @ LinkError::BrokenSymlinkExists(_, _)) => {
                                    sp.stop_and_persist(failure_str(), format!("{e}"))
                                }
                                Err(e @ LinkError::FailedCreatingLink(_, _)) => {
                                    sp.stop_and_persist(failure_str(), format!("{e}"))
                                }
                                Err(e @ LinkError::IoError(_)) => {
                                    sp.stop_and_persist(failure_str(), format!("{e}"))
                                }
                                Err(e) => sp.stop_and_persist(failure_str(), format!("{e}")),
                                _ => sp.stop_and_persist(
                                    failure_str(),
                                    format!("{}: {}", link.name, op),
                                ),
                            }
                        } else {
                            f(link);
                        }
                    }
                }
                None => continue,
            };
        }
    }
    /// Runs associated function on all repos in config
    ///
    /// Unlike `series_on_all`, this does not stop if it encounters an error
    ///
    /// # Usage
    ///
    /// Here is an example of how an associated method could use this function.
    ///
    /// ```
    /// # use seidr::git::Repo;
    /// # use seidr::git::SeriesItem;
    ///
    /// let series: Vec<SeriesItem> = vec![
    ///     SeriesItem {
    ///         operation: "pull",
    ///         closure: Box::new(move |repo: &Repo| repo.pull()),
    ///     },
    ///     SeriesItem {
    ///         operation: "add",
    ///         closure: Box::new(move |repo: &Repo| repo.add_all()),
    ///     },
    ///     SeriesItem {
    ///         operation: "commit",
    ///         closure: Box::new(move |repo: &Repo| repo.commit()),
    ///     },
    ///     SeriesItem {
    ///         operation: "push",
    ///         closure: Box::new(move |repo: &Repo| repo.push()),
    ///     },
    /// ];
    /// ```
    pub fn all_on_all(&self, closures: Vec<SeriesItem>, break_on_err: bool) {
        // HACK: creates a empty repo, that is used if a category doesn't have
        // any repos or don't define the repo field
        let tmp: HashMap<String, Repo> = HashMap::new();

        for category in self.categories.values() {
            // HACK: if the repo doesn't exist here, we inject tmp
            for (_, repo) in category.repos.as_ref().unwrap_or(&tmp).iter() {
                use RepoKinds::*;
                match &repo.kind {
                    Some(GitRepo) => {
                        for instruction in &closures {
                            let f = &instruction.closure;
                            let op = instruction.operation;
                            if !settings::QUIET.load(std::sync::atomic::Ordering::Relaxed) {
                                let mut sp = Spinner::new(
                                    Spinners::Dots10,
                                    format!("{}: {}", repo.name.as_ref().unwrap(), op),
                                );
                                if f(repo) {
                                    sp.stop_and_persist(
                                        success_str(),
                                        format!("{}: {}", repo.name.as_ref().unwrap(), op),
                                    );
                                } else {
                                    sp.stop_and_persist(
                                        failure_str(),
                                        format!("{}: {}", repo.name.as_ref().unwrap(), op),
                                    );
                                    if break_on_err {
                                        break;
                                    }
                                }
                            } else {
                                f(repo);
                            }
                        }
                    }
                    None => {
                        println!("unknown kind {:?}", repo.kind);
                    }
                    Some(kind) => {
                        println!("unknown kind {kind:?}");
                    }
                }
            }
        }
    }
    pub fn get_repo<F>(&self, cat_name: &str, repo_name: &str, f: F)
    where
        F: FnOnce(&Repo),
    {
        f(self
            .categories
            .get(cat_name)
            .expect("failed to get category")
            .repos
            .as_ref()
            .expect("failed to get repo")
            .get(repo_name)
            .expect("failed to get category"));
    }
    pub fn get_link<F>(&self, cat_name: &str, link_name: &str, f: F)
    where
        F: FnOnce(&Link),
    {
        f(self
            .categories
            .get(cat_name)
            .expect("failed to get category")
            .links
            .as_ref()
            .expect("failed to get repo")
            .get(link_name)
            .expect("failed to get category"));
    }
    /// Tries to pull all repositories, skips if fail.
    pub fn pull_all(&self) {
        debug!("exectuting pull_all");
        self.on_all_repos_spinner("pull", Repo::pull);
    }
    /// Tries to clone all repossitories, skips if fail.
    pub fn clone_all(&self) {
        debug!("exectuting clone_all");
        self.on_all_repos_spinner("clone", Repo::clone);
    }
    /// Tries to add all work in all repossitories, skips if fail.
    pub fn add_all(&self) {
        debug!("exectuting clone_all");
        self.on_all_repos_spinner("add", Repo::add_all);
    }
    /// Tries to commit all repossitories one at a time, skips if fail.
    pub fn commit_all(&self) {
        debug!("exectuting clone_all");
        self.on_all_repos_spinner("commit", Repo::commit);
    }
    /// Tries to commit all repossitories with msg, skips if fail.
    pub fn commit_all_msg(&self, msg: &str) {
        debug!("exectuting clone_all");
        self.on_all_repos_spinner("commit", |repo| repo.commit_with_msg(msg));
    }
    /// Tries to pull, add all, commit with msg "quick commit", and push all
    /// repositories, skips if fail.
    pub fn quick(&self, msg: &'static str) {
        debug!("exectuting quick");
        let series: Vec<SeriesItem> = vec![
            SeriesItem {
                operation: "pull",
                closure: Box::new(Repo::pull),
            },
            SeriesItem {
                operation: "add",
                closure: Box::new(Repo::add_all),
            },
            SeriesItem {
                operation: "commit",
                closure: Box::new(move |repo: &Repo| repo.commit_with_msg(msg)),
            },
            SeriesItem {
                operation: "push",
                closure: Box::new(Repo::push),
            },
        ];
        run_series!(self, series);
    }
    /// Tries to pull, add all, commit with msg "quick commit", and push all
    /// repositories, skips if fail.
    pub fn fast(&self, msg: &'static str) {
        debug!("exectuting fast");
        let series: Vec<SeriesItem> = vec![
            SeriesItem {
                operation: "pull",
                closure: Box::new(Repo::pull),
            },
            SeriesItem {
                operation: "add",
                closure: Box::new(Repo::add_all),
            },
            SeriesItem {
                operation: "commit",
                closure: Box::new(move |repo: &Repo| repo.commit_with_msg(msg)),
            },
            SeriesItem {
                operation: "push",
                closure: Box::new(Repo::push),
            },
        ];
        run_series!(self, series, true);
    }
    /// Tries to link all repositories, skips if fail.
    pub fn link_all(&self) {
        debug!("exectuting link_all");
        self.on_all_links_spinner("link", Link::link);
    }
}
