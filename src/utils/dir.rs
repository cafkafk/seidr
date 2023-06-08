#![feature(stmt_expr_attributes)]
use log::{debug, error, info, trace, warn};

use std::env;
use std::path::Path;

pub fn current_dir() -> String {
    #[allow(deprecated)] // NOTE we don't care about windows , we don't support it
    env::current_dir()
        .expect("Failed to get current_dir")
        .into_os_string()
        .into_string()
        .expect("Failed to turn home_dir into a valid string")
}

pub fn home_dir() -> String {
    #[allow(deprecated)] // NOTE we don't care about windows , we don't support it
    env::home_dir()
        .expect("Failed to get home_dir")
        .into_os_string()
        .into_string()
        .expect("Failed to turn home_dir into a valid string")
}

/// Changes working directory into a repository.
///
/// WARNING: NOT THREAD SAFE
fn change_dir_repo(path: &String, name: &String) {
    let mut full_path: String = "".to_owned();
    full_path.push_str(path);
    full_path.push_str(name);
    let root = Path::new(&full_path);
    println!("{}", root.display());
    assert!(env::set_current_dir(&root).is_ok());
    debug!(
        "Successfully changed working directory to {}!",
        root.display()
    );
}

/// Changes working directory to outside of the repo.
///
/// WARNING: NOT THREAD SAFE
fn change_dir(path: &String) {
    let root = Path::new(path);
    assert!(env::set_current_dir(&root).is_ok());
    debug!(
        "Successfully changed working directory to {}!",
        root.display()
    );
}

/// Returns the users home directory (on unix like only)
macro_rules! current_dir {
    () => {
        env::current_dir()
            .expect("Failed to get current_dir")
            .into_os_string()
            .into_string()
            .expect("Failed to turn home_dir into a valid string")
    };
}

/// Returns the users home directory (on unix like only)
macro_rules! home_dir {
    () => {
        #[allow(deprecated)] // NOTE we don't care about windows , we don't support it
        env::home_dir()
            .expect("Failed to get home_dir")
            .into_os_string()
            .into_string()
            .expect("Failed to turn home_dir into a valid string")
    };
}
