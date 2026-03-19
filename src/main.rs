use clap::Parser;
use gitru::cli::{Cli, Commands};
use gitru::hook::{self, run_hook};
use gitru::util::colored_console::init_console;
use gitru::util::colored_print::print_error;

fn main() {
    init_console();

    let cli = Cli::parse();

    match cli.command {
        Commands::II { hook, force } => {
            if let Err(e) = hook::init(&hook, force) {
                print_error(&e);
                std::process::exit(1);
            }

            if let Err(e) = hook::install(&hook, force) {
                print_error(&e);
                std::process::exit(1);
            }
        }

        Commands::Init { hook, force } => {
            if let Err(e) = hook::init(&hook, force) {
                print_error(&e);
                std::process::exit(1);
            }
        }

        Commands::Install { hook, force } => {
            if let Err(e) = hook::install(&hook, force) {
                print_error(&e);
                std::process::exit(1);
            }
        }

        Commands::Uninstall { hook } => {
            if let Err(e) = hook::uninstall(&hook) {
                print_error(&e);
                std::process::exit(1);
            }
        }

        Commands::Run { hook } => {
            if let Err(err) = run_hook(&hook) {
                print_error(&err);
                std::process::exit(1);
            }
        }
    }
}
