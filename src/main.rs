use crate::install::init_commit_msg_rule;
use clap::Parser;
use cli::Hooks;
use install::install_commit_msg_hook;
use util::init_tracing_once;

mod cli;
mod config;
mod install;
mod util;
mod validate;
fn main() {
    init_tracing_once();

    let args = Hooks::parse();
    match args {
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
        },
    }
}
