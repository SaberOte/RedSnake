use windows::core::Result;
use windows::Win32::System::Console;
use std::io::{stdout, Read, Write};

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

pub fn get_user_input() -> String{
    let mut read_buffer: [Console::INPUT_RECORD; 32] = [Console::INPUT_RECORD::default(); 32];
    let mut write_buffer: [Console::INPUT_RECORD; 1] = [Console::INPUT_RECORD::default(); 1];
    let mut read_count: u32 = 0;
    let mut write_count: u32 = 0;
    unsafe {
        let handle = Console::GetStdHandle(Console::STD_INPUT_HANDLE).expect("Failed to get handle");
        // put something in stdin (in order to not pause)
        Console::WriteConsoleInputA(handle, &mut write_buffer, &mut write_count).expect("Failed to write input");

        // receive input from stdin
        Console::ReadConsoleInputW(handle, &mut read_buffer, &mut read_count).expect("Failed to read input");
    }
    
    let mut result = String::new();
    for record in &read_buffer[..read_count as usize] {
        if record.EventType == Console::KEY_EVENT as u16 {
            let key_event = unsafe { record.Event.KeyEvent };
            if key_event.bKeyDown.as_bool() {
                unsafe {
                    if let Some(ch) = char::from_u32(key_event.uChar.UnicodeChar as u32) {
                        result.push(ch);
                    }
                }
            }
        }
    }
    result
}