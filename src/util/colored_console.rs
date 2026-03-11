// Non-Windows platforms: do nothing
#[cfg(not(windows))]
pub fn init_console() {
    // ANSI is supported by default, no extra handling required
}

// Windows platforms: enable virtual terminal mode
#[cfg(windows)]
pub fn init_console() {
    use winapi::um::{
        consoleapi::{GetConsoleMode, SetConsoleMode},
        processenv::GetStdHandle,
        winbase::STD_OUTPUT_HANDLE,
        wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}
