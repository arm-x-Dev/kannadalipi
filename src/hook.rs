//! Windows Low-Level Keyboard Hook (WH_KEYBOARD_LL) for Aksharamale.
//! Captures keystrokes globally, tracks modifier state, and suppresses Latin keys.

use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::Mutex;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyboardState, ToUnicode, VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN,
    VK_ESCAPE, VK_F12, VK_LEFT, VK_LWIN, VK_MENU, VK_RETURN, VK_RIGHT, VK_RWIN, VK_SCROLL,
    VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT,
    WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};

use crate::engine::TransliterationEngine;
use crate::injector::{inject_backspaces, inject_unicode_text, AKSHARA_EXTRA_INFO};
use crate::types::KeyAction;

/// Global flag controlling whether Kannada transliteration is currently active.
pub static ENGINE_ENABLED: AtomicBool = AtomicBool::new(true);

/// Atomic storage for global HHOOK pointer to ensure thread-safety without static mut warnings.
static HOOK_HANDLE_PTR: AtomicIsize = AtomicIsize::new(0);

/// Global thread-safe engine instance.
static ENGINE: Mutex<Option<TransliterationEngine>> = Mutex::new(None);

/// Install the low-level keyboard hook.
pub fn install_hook() -> Result<(), windows::core::Error> {
    unsafe {
        {
            let mut lock = ENGINE.lock().unwrap();
            *lock = Some(TransliterationEngine::new());
        }

        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            HINSTANCE(null_mut()),
            0,
        )?;

        HOOK_HANDLE_PTR.store(hook.0 as isize, Ordering::SeqCst);
        Ok(())
    }
}

/// Uninstall the low-level keyboard hook.
pub fn uninstall_hook() {
    let ptr = HOOK_HANDLE_PTR.swap(0, Ordering::SeqCst);
    if ptr != 0 {
        unsafe {
            let hook = HHOOK(ptr as *mut core::ffi::c_void);
            let _ = UnhookWindowsHookEx(hook);
        }
    }
}

/// Toggle the engine enabled state. Returns the new state (true = enabled).
pub fn toggle_engine() -> bool {
    let current = ENGINE_ENABLED.load(Ordering::SeqCst);
    let new_state = !current;
    ENGINE_ENABLED.store(new_state, Ordering::SeqCst);

    if !new_state {
        if let Ok(mut lock) = ENGINE.lock() {
            if let Some(engine) = lock.as_mut() {
                engine.reset();
            }
        }
    }

    new_state
}

/// Low-level keyboard callback procedure.
unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code < 0 {
        return CallNextHookEx(None, n_code, w_param, l_param);
    }

    let msg = w_param.0 as u32;

    if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
        let kbd = *(l_param.0 as *const KBDLLHOOKSTRUCT);

        // Strict Injection Guard: Ignore events with LLKHF_INJECTED flag OR our AKSHARA signature
        if (kbd.flags.0 & 0x10) != 0 || kbd.dwExtraInfo == AKSHARA_EXTRA_INFO {
            return CallNextHookEx(None, n_code, w_param, l_param);
        }

        let vk = kbd.vkCode as i32;

        // Toggle Key: F12 or ScrollLock (Classic Nudi / Baraha toggle)
        if vk == VK_SCROLL.0 as i32 || vk == VK_F12.0 as i32 {
            let active = toggle_engine();
            crate::tray::TrayIcon::update_state(active);
            return LRESULT(1);
        }

        // Navigation or breaking keys: reset engine state and pass through
        if vk == VK_SPACE.0 as i32
            || vk == VK_RETURN.0 as i32
            || vk == VK_BACK.0 as i32
            || vk == VK_TAB.0 as i32
            || vk == VK_ESCAPE.0 as i32
            || vk == VK_LEFT.0 as i32
            || vk == VK_RIGHT.0 as i32
            || vk == VK_UP.0 as i32
            || vk == VK_DOWN.0 as i32
            || vk == VK_DELETE.0 as i32
        {
            if let Ok(mut lock) = ENGINE.lock() {
                if let Some(engine) = lock.as_mut() {
                    engine.reset();
                }
            }
            return CallNextHookEx(None, n_code, w_param, l_param);
        }

        // System Modifier Keys: Ctrl, Alt, or Windows Key (Pass through shortcuts like Ctrl+C, Alt+Tab, Win+D)
        let ctrl_pressed = (GetAsyncKeyState(VK_CONTROL.0 as i32) as u16 & 0x8000) != 0;
        let alt_pressed = (GetAsyncKeyState(VK_MENU.0 as i32) as u16 & 0x8000) != 0;
        let win_pressed = (GetAsyncKeyState(VK_LWIN.0 as i32) as u16 & 0x8000) != 0
            || (GetAsyncKeyState(VK_RWIN.0 as i32) as u16 & 0x8000) != 0;

        if ctrl_pressed || alt_pressed || win_pressed {
            return CallNextHookEx(None, n_code, w_param, l_param);
        }

        // If engine is toggled OFF, pass through all keys
        if !ENGINE_ENABLED.load(Ordering::SeqCst) {
            return CallNextHookEx(None, n_code, w_param, l_param);
        }

        // Resolve virtual key to Unicode character considering Shift and CapsLock
        if let Some(ch) = vk_to_char(kbd.vkCode, kbd.scanCode) {
            if let Ok(mut lock) = ENGINE.lock() {
                if let Some(engine) = lock.as_mut() {
                    let action = engine.process_char(ch);
                    match action {
                        KeyAction::PassThrough => {
                            return CallNextHookEx(None, n_code, w_param, l_param);
                        }
                        KeyAction::Suppress => {
                            return LRESULT(1);
                        }
                        KeyAction::Emit(text) => {
                            inject_unicode_text(&text);
                            return LRESULT(1);
                        }
                        KeyAction::Replace { backspaces, text } => {
                            inject_backspaces(backspaces);
                            inject_unicode_text(&text);
                            return LRESULT(1);
                        }
                    }
                }
            }
        }
    }

    CallNextHookEx(None, n_code, w_param, l_param)
}

/// Helper: Translate virtual key code to character based on current keyboard state.
unsafe fn vk_to_char(vk_code: u32, scan_code: u32) -> Option<char> {
    let mut keyboard_state = [0u8; 256];
    if GetKeyboardState(&mut keyboard_state).is_err() {
        return None;
    }

    let shift_down = (GetAsyncKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0;
    if shift_down {
        keyboard_state[VK_SHIFT.0 as usize] = 0x80;
    } else {
        keyboard_state[VK_SHIFT.0 as usize] = 0x00;
    }

    let mut buffer = [0u16; 8];
    let result = ToUnicode(
        vk_code,
        scan_code,
        Some(&keyboard_state),
        &mut buffer,
        0,
    );

    if result > 0 {
        char::decode_utf16(buffer[..result as usize].iter().copied())
            .next()
            .and_then(|r| r.ok())
    } else {
        None
    }
}
