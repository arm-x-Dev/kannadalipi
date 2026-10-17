//! Windows SendInput text injector for Aksharamale.
//! Injects pure Unicode characters directly into whatever window currently has keyboard focus.

use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
    VK_BACK, VIRTUAL_KEY,
};

/// Custom signature embedded in dwExtraInfo to mathematically identify our synthetic keystrokes.
pub const AKSHARA_EXTRA_INFO: usize = 0x414B5348; // 'AKSH'

/// Injects a UTF-8 string as a series of Unicode keypress events via SendInput.
pub fn inject_unicode_text(text: &str) {
    if text.is_empty() {
        return;
    }

    let mut inputs: Vec<INPUT> = Vec::new();

    for utf16_char in text.encode_utf16() {
        // Key DOWN
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: utf16_char,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: AKSHARA_EXTRA_INFO,
                },
            },
        });

        // Key UP
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: utf16_char,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: AKSHARA_EXTRA_INFO,
                },
            },
        });
    }

    unsafe {
        let _ = SendInput(&inputs, size_of::<INPUT>() as i32);
    }
}

/// Injects a specified number of virtual backspace keystrokes to erase previous characters.
pub fn inject_backspaces(count: usize) {
    if count == 0 {
        return;
    }

    let mut inputs: Vec<INPUT> = Vec::with_capacity(count * 2);

    for _ in 0..count {
        // Backspace DOWN
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_BACK,
                    wScan: 0,
                    dwFlags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: AKSHARA_EXTRA_INFO,
                },
            },
        });

        // Backspace UP
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_BACK,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: AKSHARA_EXTRA_INFO,
                },
            },
        });
    }

    unsafe {
        let _ = SendInput(&inputs, size_of::<INPUT>() as i32);
    }
}
