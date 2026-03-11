use crate::util::colored_print::{print_error, print_success};

pub mod commit_msg;

pub fn init(hook: &str, force: bool) -> Result<(), String> {
    // write files to the current project root based on the hook type
    match hook {
        "commit-msg" => {
            commit_msg::init(force)?;
        }
        _ => {
            return Err(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "unknown hook").to_string(),
            );
        }
    }

    Ok(())
}

pub fn install(hook: &str, force: bool) -> Result<(), String> {
    match hook {
        "commit-msg" => {
            commit_msg::install(force)?;
        }
        _ => {
            return Err(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "unknown hook").to_string(),
            );
        }
    }

    Ok(())
}

pub fn uninstall(hook: &str) -> Result<(), String> {
    match hook {
        "commit-msg" => {
            commit_msg::uninstall()?;
        }
        _ => {
            return Err(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "unknown hook").to_string(),
            );
        }
    }

    Ok(())
}

pub fn run_hook(hook: &str, file: Option<&str>) -> Result<(), String> {
    match hook {
        "commit-msg" => {
            let path = file.ok_or("commit-msg requires a file")?;

            if let Err(e) = commit_msg::run(path) {
                print_error(&e);
                // status code 1 means fail
                std::process::exit(1);
            }

            // status code 0 means success
            print_success("commit-msg validation passed");
            std::process::exit(0);
        }
        _ => Err(format!("unsupported hook: {}", hook)),
    }
}
