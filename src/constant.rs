use std::env;
use std::sync::LazyLock;

pub const COMMIT_MSG_RULE_TEMPLATE: &str =
    include_str!("../template/commit-msg-rule-template.toml");
const COMMIT_MSG_HOOK_TEMPLATE_RAW: &str = include_str!("../template/commit-msg-hook-template.sh");

pub const COMMIT_MSG_RULE_FILE_NAME: &str = ".commit-msg-rule.toml";

fn get_program_path() -> Option<String> {
    let exe_path = env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?.to_path_buf();
    let exe_name = exe_path.file_name()?.to_string_lossy().into_owned();

    let path_env = env::var("PATH").ok()?;

    let in_path = env::split_paths(&path_env).any(|p| p == exe_dir);

    if in_path {
        Some(exe_name)
    } else {
        Some(exe_path.to_string_lossy().into_owned())
    }
}

pub static COMMIT_MSG_HOOK_CONTENT: LazyLock<String> = LazyLock::new(|| match get_program_path() {
    Some(program_path) => COMMIT_MSG_HOOK_TEMPLATE_RAW
        .replace("{{program_exec}}", &program_path)
        .replace("\r\n", "\n")
        .replace('\r', ""),

    None => COMMIT_MSG_HOOK_TEMPLATE_RAW
        // Keep a placeholder for detection if replacement fails
        .replace("{{program_exec}}", "{{program_placeholder}}")
        .replace("\r\n", "\n")
        .replace('\r', ""),
});

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_program_name() {
        println!("{:?}", get_program_path());
    }

    #[test]
    fn test_get_commit_msg_hook_content() {
        println!("{}", COMMIT_MSG_HOOK_CONTENT.as_str());
    }
}
