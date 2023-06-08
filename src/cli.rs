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

    /// Print full-text license information
    #[arg(long)]
    pub license: bool,

    /// Print full-text code-of-conduct (not implemented)
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
