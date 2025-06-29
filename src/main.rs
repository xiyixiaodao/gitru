use crate::install::{init_commit_msg_rule, remove_commit_msg_hook};
use clap::Parser;
use cli::{Cli, Hooks};
use install::install_commit_msg_hook;
use util::init_tracing_once;

mod cli;
mod commit_msg;
mod config;
mod install;
mod util;
mod validate;

fn main() {
    // Enable ANSI color support for legacy Windows10 console
    #[cfg(windows)]
    util::enable_old_windows_color_support();

    // Initialize global logging system
    init_tracing_once();

    let cli = Cli::parse();
    let hook = cli.command;

    match hook {
        Hooks::CommitMsg { action } => match action {
            cli::CommitMsgAction::Init => {
                init_commit_msg_rule();
            }
            cli::CommitMsgAction::Install => {
                install_commit_msg_hook();
            }
            cli::CommitMsgAction::II => {
                install_commit_msg_hook();
                init_commit_msg_rule();
            }
            cli::CommitMsgAction::Validate {
                msg_path,
                rule_path,
            } => {
                validate::validate_msg(msg_path.as_str(), rule_path.as_str());
            }
            cli::CommitMsgAction::Uninstall => remove_commit_msg_hook(),
        },
    }
}
