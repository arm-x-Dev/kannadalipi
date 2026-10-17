#![windows_subsystem = "windows"]

mod types;
mod keymap;
mod engine;
mod injector;
mod hook;
mod tray;

use windows::core::w;
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS};
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, MessageBoxW, TranslateMessage, MB_ICONINFORMATION, MB_OK, MSG,
};

fn main() {
    // Single-Instance Guard: Prevent duplicate instances from running concurrently
    let mutex_handle = unsafe {
        let handle = CreateMutexW(None, true, w!("Global\\AksharamaleSingleInstanceMutex"));
        if GetLastError() == ERROR_ALREADY_EXISTS {
            let _ = MessageBoxW(
                None,
                w!("Aksharamale is already running in your System Tray.\nPress [F12] to toggle Kannada typing."),
                w!("Aksharamale"),
                MB_OK | MB_ICONINFORMATION,
            );
            return;
        }
        handle
    };

    // Install the low-level keyboard hook
    if let Err(_) = hook::install_hook() {
        unsafe {
            let _ = MessageBoxW(
                None,
                w!("Failed to install keyboard hook. Please check system permissions."),
                w!("Aksharamale Error"),
                windows::Win32::UI::WindowsAndMessaging::MB_OK | windows::Win32::UI::WindowsAndMessaging::MB_ICONERROR,
            );
        }
        return;
    }

    // Initialize System Tray Icon
    let _tray = match tray::TrayIcon::init() {
        Ok(t) => t,
        Err(_) => {
            hook::uninstall_hook();
            return;
        }
    };

    // Standard Win32 message pump (blocks efficiently without CPU consumption)
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    // Clean up hook and single-instance mutex
    hook::uninstall_hook();
    if let Ok(h) = mutex_handle {
        unsafe {
            let _ = CloseHandle(h);
        }
    }
}
