use windows::Win32::System::Console::{FreeConsole, AllocConsole, SetConsoleWindowInfo, GetStdHandle, STD_OUTPUT_HANDLE, CONSOLE_WINDOWED_MODE, COORD, SetConsoleScreenBufferInfoEx, CONSOLE_SCREEN_BUFFER_INFOEX, SMALL_RECT};
use windows::Win32::System::Console;
use windows::Win32::Foundation::COLORREF;
use windows::core::Result;



pub fn create_new_console(width: i16, height: i16) -> Result<()> {
    unsafe {
        // free current console. One process can't use 2 console, as it says win api documentation
        FreeConsole()?;
        // Allocate a new console
        AllocConsole()?;

        // Set sizes
        let handle = GetStdHandle(STD_OUTPUT_HANDLE)?;
        let mut size = COORD{X: width, Y: height };
        // SetConsoleDisplayMode(handle, 1, Some(&mut size))?;
        let console_screen_buffer_infoex = CONSOLE_SCREEN_BUFFER_INFOEX{
            cbSize: size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32,
            dwSize: COORD{X: width, Y: height },
            dwCursorPosition: COORD{X: 0, Y: 0 },
            wAttributes: Console::FOREGROUND_RED,
            srWindow: SMALL_RECT{Left: 0, Top: 0, Right: width, Bottom: height },
            dwMaximumWindowSize: COORD{X: width, Y: height },
            wPopupAttributes: 0,
            bFullscreenSupported: Default::default(),
            ColorTable: [
                COLORREF(0x000000),  // Black (0)
                COLORREF(0x000080),  // Dark Blue (1)
                COLORREF(0x008000),  // Dark Green (2)
                COLORREF(0x008080),  // Dark Cyan (3)
                COLORREF(0x800000),  // Dark Red (4)
                COLORREF(0x800080),  // Dark Magenta (5)
                COLORREF(0x808000),  // Dark Yellow (6)
                COLORREF(0xC0C0C0),  // Light Gray (7)
                COLORREF(0x808080),  // Dark Gray (8)
                COLORREF(0x0000FF),  // Blue (9)
                COLORREF(0x00FF00),  // Green (10)
                COLORREF(0x00FFFF),  // Cyan (11)
                COLORREF(0xFF0000),  // Red (12)
                COLORREF(0xFF00FF),  // Magenta (13)
                COLORREF(0xFFFF00),  // Yellow (14)
                COLORREF(0xFFFFFF)   // White (15)
            ]
        };
        SetConsoleScreenBufferInfoEx(handle, &console_screen_buffer_infoex)?;
        SetConsoleWindowInfo(
            handle,
            true,
            &SMALL_RECT{Left: 0, Top: 0, Right: width, Bottom: height }
        )?;
        

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