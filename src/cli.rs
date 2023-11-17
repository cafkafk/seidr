// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

//! Handles command line input

use crate::utils::dir::home_dir;
use crate::utils::strings::INTERACTIVE_NOTICE;

use clap::{ArgAction, CommandFactory, Parser, Subcommand};

const CONFIG_FILE: &str = "/.config/seidr/config.yaml";

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

";

//#[clap(author, version, about, long_about = None)]
#[derive(Parser, Debug)]
#[clap(
    name="seidr - declarative linkfarm",
    author,
    version,
    long_version=env!("CARGO_PKG_VERSION"),
    about="GitOps for the masses",
    long_about="A Rust GitOps and linkfarm orchestrator inspired by GNU Stow",
    subcommand_required=false,
    arg_required_else_help=true,
    help_template=HELP_TEMPLATE.to_owned()+INTERACTIVE_NOTICE,
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

    /// Print code-of-conduct information
    #[arg(long)]
    pub code_of_conduct: bool,

    /// Try to be as quiet as possible (unix philosophy) (not imlemented)
    #[arg(short, long)]
    pub quiet: bool,

    /// No emoji (not imlemented)
    #[arg(short, long)]
    pub no_emoji: bool,

    /// (not imlemented)
    #[arg(short, long)]
    pub unlink: bool,

    /// (not imlemented)
    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long)]
    pub message: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Link all... links
    #[command(visible_alias = "l")]
    Link {},

    /// Do quick pull-commit-push with msg for commit
    #[command(visible_alias = "q")]
    Quick {
        category: Option<String>,
        repo: Option<String>,
    },

    /// Do fast pull-commit-push with msg for commit, skipping repo on failure
    #[command(visible_alias = "f")]
    Fast {},

    /// Clone all repositories
    #[command(visible_alias = "c")]
    Clone {},

    /// Pull all repositories
    #[command(visible_alias = "p")]
    Pull {},

    /// Add all files in repositories
    #[command(visible_alias = "a")]
    Add {},

    /// Perform a git commit in all repositories
    #[command(visible_alias = "ct")]
    Commit {},

    /// Perform a git commit in all repositories, with predefined message
    #[command(visible_alias = "m")]
    CommitMsg {},

    /// Jump to a given object
    #[command(subcommand, visible_alias = "j")]
    Jump(JumpCommands),
}

#[derive(Subcommand, Debug)]
pub enum JumpCommands {
    /// Jump to repo
    #[command(visible_alias = "r")]
    Repo { category: String, name: String },

    /// Jump to link
    #[command(visible_alias = "l")]
    Link { category: String, name: String },
}
