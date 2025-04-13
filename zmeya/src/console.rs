use windows::core::Result;
use windows::Win32::System::Console::{AllocConsole, FreeConsole};


pub fn create_new_console(width: i16, height: i16) -> Result<()> {
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