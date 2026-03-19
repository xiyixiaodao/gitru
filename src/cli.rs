use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Equivalent to init + install
    II {
        /// hook name, currently only supports commit-msg and pre-commit
        hook: String,

        #[arg(short = 'f', long = "force")]
        force: bool,
    },

    /// Initialize configuration file in the repository root
    Init {
        /// hook name, currently only supports commit-msg
        hook: String,

        #[arg(short = 'f', long = "force")]
        force: bool,
    },

    /// Install git hook script into the .git/hooks directory
    Install {
        /// hook name, currently only supports commit-msg
        hook: String,

        #[arg(short = 'f', long = "force")]
        force: bool,
    },

    /// Uninstall git hook script from the .git/hooks directory
    Uninstall {
        /// hook name, currently only supports commit-msg
        hook: String,
    },

    /// Run the specified git hook script
    Run {
        /// hook name, currently only supports commit-msg
        #[command(subcommand)]
        hook: RunCmd,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum RunCmd {
    /// Validate commit message
    // #[command(name = "commit-msg")]
    CommitMsg {
        /// Path to commit message file
        #[arg(long)]
        msg: PathBuf,

        /// Path to rule file
        #[arg(long)]
        rule: PathBuf,
    },

    /// Run pre-commit hook
    // #[command(name = "pre-commit")]
    PreCommit {
        /// Path to rule file
        #[arg(long)]
        rule: PathBuf,
    },
}
