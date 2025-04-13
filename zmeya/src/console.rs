use windows::{
    core::*,
    Win32::System::Console::*,
};
use windows::Win32::Foundation::HANDLE;
use std::thread::sleep;

#[cfg(windows)]
pub fn clear_console_windows() {

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();

        // Get console info to find the size of the screen buffer
        let mut console_info = CONSOLE_SCREEN_BUFFER_INFO::default();
        GetConsoleScreenBufferInfo(handle, &mut console_info).unwrap();

        let console_size = console_info.dwSize.X as u32 * console_info.dwSize.Y as u32;
        let coord = COORD { X: 0, Y: 0 };

        // Fill the console with spaces
        let mut chars_written = 0;
        FillConsoleOutputCharacterA(handle, b' ' as i8, console_size, coord, &mut chars_written).unwrap();

        // Reset the attributes
        FillConsoleOutputAttribute(
            handle,
            console_info.wAttributes.0,
            console_size,
            coord,
            &mut chars_written
        ).unwrap();

        // Move cursor to top-left corner
        SetConsoleCursorPosition(handle, coord).unwrap();
    }
}


pub fn create_new_console() -> Result<()> {
    unsafe {
        // free current console. One process can't use 2 console, as it says win api documentation
        FreeConsole()?;
        // Allocate a new console
        AllocConsole()?;

        // let mut input_buffer = [0u8; 2];
        // let mut chars_read = 0;
        // ReadConsoleA(
        //     GetStdHandle(STD_INPUT_HANDLE)?,
        //     input_buffer.as_mut_ptr() as *mut _,
        //     1,
        //     &mut chars_read,
        //     None,
        // )?;

        Ok(())
    }
}