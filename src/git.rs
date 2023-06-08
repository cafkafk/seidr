use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::os::unix::fs::symlink;
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

impl Links {
    /// Creates a link from a file
    fn link(&self) {
        let rx_exists: bool = std::path::Path::new(&self.rx).exists();
        let tx_exists: bool = std::path::Path::new(&self.tx).exists();
        // NOTE If the file exists
        // NOTE And is not a link
        // NOTE We avoid using fs::read_link by returning early
        // FIXME This should be refactored
        if rx_exists {
            let rx_link = fs::read_link(&self.rx);
            match rx_link {
                Ok(_) => (),
                Err(error) => {
                    error!("Linking {} -> {} failed: {}", &self.tx, &self.rx, error);
                    return;
                }
            }
        }
        match (tx_exists, rx_exists) {
            (true, false) => symlink(&self.tx, &self.rx).expect("failed to create link"),
            (true, true)
                if fs::read_link(&self.rx)
                    .expect("failed to read link")
                    .to_str()
                    .unwrap()
                    != &self.tx =>
            {
                error!(
                    "link {}: can't link rx {}, file already exists",
                    &self.name, &self.rx
                )
            }
            (true, true)
                if fs::read_link(&self.rx)
                    .expect("failed to read link")
                    .to_str()
                    .unwrap()
                    == &self.tx =>
            {
                warn!(
                    "link {}: can't link rx {}, file already exists",
                    &self.name, &self.rx
                )
            }
            (false, _) => error!("link {}: could not find tx at {}", &self.name, &self.rx),
            (_, _) => unreachable!("Should never happen!"),
        }
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
            .current_dir(&(self.path.to_owned() + &self.name))
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
            .current_dir(&(self.path.to_owned() + &self.name))
            .arg("commit")
            .status()
            .expect("failed to commit");
        info!("{out}");
    }
    /// Tries to commit changes with a message argument.
    fn commit_with_msg(&self, msg: &String) {
        let out = Command::new("git")
            .current_dir(&(self.path.to_owned() + &self.name))
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
            .current_dir(&(self.path.to_owned() + &self.name))
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
