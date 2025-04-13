use windows::core::Result;
use windows::Win32::System::Console;
use std::io::{stdout, Write};


pub fn get_cursor() -> Result<(i16, i16)> {
    let mut info: Console::CONSOLE_SCREEN_BUFFER_INFO = Console::CONSOLE_SCREEN_BUFFER_INFO::default();
    unsafe{
        let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE)?;
        Console::GetConsoleScreenBufferInfo (handle, &mut info)?;
    }

    Ok((info.dwCursorPosition.X, info.dwCursorPosition.Y))
}

pub fn set_cursor(x: i16, y: i16) {
    unsafe{
        let handle = Console::GetStdHandle(Console::STD_OUTPUT_HANDLE).expect("Failed to get handle");
        Console::SetConsoleCursorPosition(handle, Console::COORD{
            X: x,
            Y: y,
        }).expect("Failed to set cursor position");
    }
}

pub fn update_screen(x: i16, y: i16, c: char) {
    set_cursor(x, y);
    let mut lock = stdout().lock();
    write!(lock, "{}{}", c, c).unwrap();
    stdout().flush().expect("Failed to flush stdout");
}