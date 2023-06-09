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

use crate::utils::dir::home_dir;

use clap::{ArgAction, CommandFactory, Parser, Subcommand};

const CONFIG_FILE: &str = "/.config/gg/config.yaml";

//#[clap(author, version, about, long_about = None)]
#[derive(Parser, Debug)]
#[clap(
    name="gg - git gut",
    author,
    version,
    long_version=env!("CARGO_PKG_VERSION"),
    about="GitOps for the masses",
    long_about="A Rust GitOps and linkfarm orchestrator inspired by GNU Stow",
    subcommand_required=true,
    arg_required_else_help=true,
    help_template="\
    {before-help}{name} {version}
    {author-with-newline}{about-with-newline}
    {usage-heading} {usage}

    {all-args}{after-help}

    ",
)]
pub struct Args {
    /// The config file to use
    #[allow(deprecated)] // NOTE we don't care about windows , we don't support it
    #[arg(short, long, default_value_t = home_dir() + CONFIG_FILE)]
    pub config: String,

    /// Print license information
    #[arg(long)]
    pub license: bool,

    /// Print warranty information
    #[arg(long)]
    pub warranty: bool,

    /// Print code-of-conduct information (not implemented)
    #[arg(long)]
    pub code_of_conduct: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Link all... links
    #[command(visible_alias = "l")]
    Link { msg: Option<String> },

    /// Do quick pull-commit-push with msg for commit
    #[command(visible_alias = "q")]
    Quick { msg: Option<String> },

    /// Clone all repositories
    #[command(visible_alias = "c")]
    Clone { msg: Option<String> },

    /// Pull all repositories
    #[command(visible_alias = "p")]
    Pull { msg: Option<String> },

    /// Add all files in repositories
    #[command(visible_alias = "a")]
    Add { msg: Option<String> },

    /// Perform a git commit in all repositories
    #[command(visible_alias = "ct")]
    Commit { msg: Option<String> },

    /// Perform a git commit in all repositories, with predefined message
    #[command(visible_alias = "m")]
    CommitMsg { msg: Option<String> },
}
