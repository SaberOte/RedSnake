use windows::core::Result;
use windows::Win32::System::Console;
use std::io;
use std::io::{Write};


pub fn create_new_console(width: i16, height: i16) -> Result<()> {
    unsafe {
        // free current console. One process can't use 2 console, as it says win api documentation
        Console::FreeConsole()?;
        // Allocate a new console
        Console::AllocConsole()?;

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

pub fn set_cursor(x: i16, y: i16) -> Result<()> {
    unsafe{
        let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE)?;
        Console::SetConsoleCursorPosition(handle, Console::COORD{
            X: x,
            Y: y
        })?;
    }
    
    Ok(())
}

pub fn update_screen(x: i16, y: i16, c: char) -> Result<()> {
    set_cursor(1 + x * 2,
               1 + y)?;
    print!("{}{}", c, c);
    io::stdout().flush()?;
    Ok(())
}