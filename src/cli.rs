use clap::{Parser, Subcommand};

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
        hook: String,

        // commit-msg hook requires a commit message file
        // pre-commit hook does not require a file parameter
        #[arg(required = false)]
        file: Option<String>,
    },
}
