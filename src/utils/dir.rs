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
fn change_dir_repo(path: &str, name: &str) {
    let mut full_path: String = "".to_owned();
    full_path.push_str(path);
    full_path.push_str(name);
    let root = Path::new(&full_path);
    println!("{}", root.display());
    assert!(env::set_current_dir(root).is_ok());
    debug!(
        "Successfully changed working directory to {}!",
        root.display()
    );
}

/// Changes working directory to outside of the repo.
///
/// WARNING: NOT THREAD SAFE
fn change_dir(path: &str) {
    let root = Path::new(path);
    assert!(env::set_current_dir(root).is_ok());
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
