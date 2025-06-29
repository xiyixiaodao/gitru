mod commit_msg_init;
mod commit_msg_install;
mod commit_msg_uninstall;

pub use commit_msg_init::init_commit_msg_rule;
pub use commit_msg_install::install_commit_msg_hook;
pub use commit_msg_uninstall::remove_commit_msg_hook;
