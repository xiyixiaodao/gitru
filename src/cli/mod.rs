mod commit_msg_action;
pub use commit_msg_action::CommitMsgAction;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "gitru", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Hooks,
}

#[derive(Parser, Debug)]
pub enum Hooks {
    /// Operations related to commit-msg hook
    #[command(name = "commit-msg")]
    CommitMsg {
        #[command(subcommand)]
        action: CommitMsgAction,
    },
}
