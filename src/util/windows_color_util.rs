/// Enables ANSI escape sequence support for legacy Windows10 consoles,
/// Invoke this function at the program entry point (main()) for backward compatibility.
#[cfg(windows)]
pub fn enable_old_windows_color_support() {
    use winapi::um::{
        consoleapi::SetConsoleMode, processenv::GetStdHandle,
        wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    };

    unsafe {
        let handle = GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
        let mut mode = 0;
        if winapi::um::consoleapi::GetConsoleMode(handle, &mut mode) != 0 {
            SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}
