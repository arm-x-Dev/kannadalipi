//! System Tray integration for Aksharamale (ಅಕ್ಷರಮಾಲೆ).
//! Provides a background notification area icon with a right-click context menu
//! and balloon/toast notifications on launch and state changes.

use std::sync::atomic::Ordering;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIIF_INFO, NIM_ADD, NIM_DELETE,
    NIM_MODIFY, NOTIFYICONDATAW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    GetCursorPos, LoadIconW, PostQuitMessage, RegisterClassW, SetForegroundWindow, TrackPopupMenu,
    IDI_APPLICATION, MF_CHECKED, MF_SEPARATOR, MF_STRING, MF_UNCHECKED, TPM_BOTTOMALIGN,
    TPM_RIGHTALIGN, WINDOW_EX_STYLE, WM_COMMAND, WM_DESTROY, WM_RBUTTONUP, WM_USER, WNDCLASSW,
    WS_OVERLAPPEDWINDOW,
};

use crate::hook::{toggle_engine, ENGINE_ENABLED};

const WM_TRAYICON: u32 = WM_USER + 1;
const ID_TRAY_TOGGLE: usize = 1001;
const ID_TRAY_ABOUT: usize = 1002;
const ID_TRAY_EXIT: usize = 1003;

static mut TRAY_HWND: HWND = HWND(std::ptr::null_mut());

pub struct TrayIcon;

impl TrayIcon {
    pub fn init() -> Result<Self, windows::core::Error> {
        unsafe {
            let class_name = w!("AksharamaleTrayClass");

            let wc = WNDCLASSW {
                lpfnWndProc: Some(tray_window_proc),
                hInstance: windows::Win32::Foundation::HINSTANCE(std::ptr::null_mut()),
                lpszClassName: class_name,
                ..Default::default()
            };
            RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                class_name,
                w!("AksharamaleTrayWindow"),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                0,
                0,
                None,
                None,
                windows::Win32::Foundation::HINSTANCE(std::ptr::null_mut()),
                None,
            )?;

            TRAY_HWND = hwnd;

            // Create tray icon with startup notification toast
            let mut nid = Self::create_notify_icon_data(hwnd, true);

            nid.uFlags |= NIF_INFO;
            Self::copy_to_u16_slice(&mut nid.szInfoTitle, "Aksharamale is Active (ಅಕ್ಷರಮಾಲೆ)");
            Self::copy_to_u16_slice(
                &mut nid.szInfo,
                "Kannada typing is ON. Press [F12] to switch English/Kannada anywhere!",
            );
            nid.dwInfoFlags = NIIF_INFO;

            let _ = Shell_NotifyIconW(NIM_ADD, &mut nid);

            Ok(Self)
        }
    }

    /// Update tray icon tooltip and state when toggled.
    pub fn update_state(active: bool) {
        unsafe {
            if !TRAY_HWND.0.is_null() {
                let mut nid = Self::create_notify_icon_data(TRAY_HWND, active);
                let _ = Shell_NotifyIconW(NIM_MODIFY, &mut nid);
            }
        }
    }

    fn create_notify_icon_data(hwnd: HWND, active: bool) -> NOTIFYICONDATAW {
        let mut nid = NOTIFYICONDATAW::default();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        nid.uCallbackMessage = WM_TRAYICON;

        unsafe {
            if let Ok(h_module) = windows::Win32::System::LibraryLoader::GetModuleHandleW(None) {
                let custom_icon = windows::Win32::UI::WindowsAndMessaging::LoadImageW(
                    windows::Win32::Foundation::HINSTANCE(h_module.0),
                    PCWSTR(1 as *const u16),
                    windows::Win32::UI::WindowsAndMessaging::IMAGE_ICON,
                    32,
                    32,
                    windows::Win32::UI::WindowsAndMessaging::IMAGE_FLAGS(0),
                );
                if let Ok(h_icon) = custom_icon {
                    nid.hIcon = windows::Win32::UI::WindowsAndMessaging::HICON(h_icon.0);
                } else {
                    nid.hIcon = LoadIconW(None, IDI_APPLICATION).unwrap_or_default();
                }
            } else {
                nid.hIcon = LoadIconW(None, IDI_APPLICATION).unwrap_or_default();
            }
        }

        let tip_text = if active {
            "Aksharamale (ಅಕ್ಷರಮಾಲೆ): Kannada ON (F12 to toggle)"
        } else {
            "Aksharamale: English Passthrough (F12 to toggle)"
        };

        Self::copy_to_u16_slice(&mut nid.szTip, tip_text);

        nid
    }

    fn copy_to_u16_slice(dest: &mut [u16], src: &str) {
        let max_len = dest.len().saturating_sub(1);
        for (i, c) in src.encode_utf16().enumerate().take(max_len) {
            dest[i] = c;
        }
        dest[src.encode_utf16().count().min(max_len)] = 0;
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        unsafe {
            if !TRAY_HWND.0.is_null() {
                let mut nid = NOTIFYICONDATAW::default();
                nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
                nid.hWnd = TRAY_HWND;
                nid.uID = 1;
                let _ = Shell_NotifyIconW(NIM_DELETE, &mut nid);
                let _ = DestroyWindow(TRAY_HWND);
            }
        }
    }
}

unsafe extern "system" fn tray_window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            let event = l_param.0 as u32;
            if event == WM_RBUTTONUP {
                show_context_menu(hwnd);
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let cmd_id = (w_param.0 & 0xFFFF) as usize;
            match cmd_id {
                ID_TRAY_TOGGLE => {
                    let active = toggle_engine();
                    TrayIcon::update_state(active);
                }
                ID_TRAY_ABOUT => {
                    windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                        hwnd,
                        w!("Aksharamale (ಅಕ್ಷರಮಾಲೆ) v0.1.0\nOpen-source Kannada Unicode Typing Engine\nKP Rao (KaGaPa) Phonetic Layout\n\nHotkey: Press F12 or Scroll Lock to toggle anywhere!"),
                        w!("About Aksharamale"),
                        windows::Win32::UI::WindowsAndMessaging::MB_OK | windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION,
                    );
                }
                ID_TRAY_EXIT => {
                    PostQuitMessage(0);
                }
                _ => {}
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

unsafe fn show_context_menu(hwnd: HWND) {
    if let Ok(menu) = CreatePopupMenu() {
        let is_active = ENGINE_ENABLED.load(Ordering::SeqCst);
        let check_flag = if is_active { MF_CHECKED } else { MF_UNCHECKED };

        let _ = AppendMenuW(
            menu,
            check_flag | MF_STRING,
            ID_TRAY_TOGGLE,
            w!("Kannada Typing (F12)"),
        );
        let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
        let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_ABOUT, w!("About Aksharamale..."));
        let _ = AppendMenuW(menu, MF_STRING, ID_TRAY_EXIT, w!("Exit"));

        let mut pt = POINT::default();
        let _ = GetCursorPos(&mut pt);

        let _ = SetForegroundWindow(hwnd);

        let _ = TrackPopupMenu(
            menu,
            TPM_BOTTOMALIGN | TPM_RIGHTALIGN,
            pt.x,
            pt.y,
            0,
            hwnd,
            None,
        );

        let _ = DestroyMenu(menu);
    }
}
