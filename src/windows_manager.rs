use std::fmt::Display;

use windows::{
    Win32::Foundation::*,
    Win32::{
        System::{
            ProcessStatus::GetModuleBaseNameW,
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
        UI::WindowsAndMessaging::*,
    },
};

use crate::window::Window;

#[derive(Debug, Clone)]
pub struct WinError;

impl Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to process WinApi calls")
    }
}

//thread_local!(static LAST_RESULT: RefCell<Vec<Window>> = RefCell::new(Vec::new()));
//LAST_RESULT.with(|list| list.borrow_mut().clear());

pub struct WinManager {
    result: Vec<Window>,
}

impl WinManager {
    fn new() -> Self {
        WinManager { result: Vec::new() }
    }

    pub fn get_process_name_by_pid(pid: u32) -> String {
        let default_process_name = String::from(""); // TODO: do smth with this

        let handle: HANDLE = unsafe {
            {
                let this = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, BOOL(0), pid);
                match this {
                    Ok(handle) => handle,
                    Err(_) => return default_process_name,
                }
            }
        };
        if handle.is_invalid() {
            return default_process_name;
        }

        let mut process_name: [u16; 512] = [0; 512];
        let len = unsafe { GetModuleBaseNameW(handle, HMODULE(0), &mut process_name) };
        unsafe { CloseHandle(handle).unwrap() };
        if len == 0 {
            return default_process_name;
        }
        let process_name = String::from_utf16_lossy(&process_name[..len as usize]);

        process_name
    }

    pub fn set_window_position(handle: isize, window: RECT) {
        unsafe {
            SetWindowPos(
                HWND(handle),
                HWND(0),
                window.left,
                window.top,
                window.right - window.left,
                window.bottom - window.top,
                SWP_NOACTIVATE | SWP_NOOWNERZORDER | SWP_NOZORDER,
            )
            .unwrap();
        };
    }

    pub fn get_windows_data() -> Result<Vec<Window>, WinError> {
        let mut winmgr = WinManager::new();
        winmgr.request_windows_data();

        Ok(winmgr.result)
    }

    fn request_windows_data(&mut self) {
        unsafe {
            EnumWindows(
                Some(Self::enum_window_callback),
                LPARAM(&mut self.result as *mut Vec<Window> as isize),
            )
            .unwrap();
        }
    }

    extern "system" fn enum_window_callback(window_handle: HWND, result: LPARAM) -> BOOL {
        unsafe {
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(window_handle, Some(&mut pid));

            let mut title: [u16; 512] = [0; 512];
            let len = GetWindowTextW(window_handle, &mut title);
            let title = String::from_utf16_lossy(&title[..len as usize]);

            let mut classname: [u16; 512] = [0; 512];
            let len = GetClassNameW(window_handle, &mut classname);
            let classname = String::from_utf16_lossy(&classname[..len as usize]);

            let mut info = WINDOWINFO {
                cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
                ..Default::default()
            };
            GetWindowInfo(window_handle, &mut info).unwrap();

            let process = WinManager::get_process_name_by_pid(pid);

            if !title.is_empty() && info.dwStyle.contains(WS_VISIBLE) {
                let window = Window {
                    process_name: process,
                    pid,
                    handle: window_handle.0,
                    title,
                    class: classname,
                    window: info.rcWindow,
                };

                let result = &mut *(result.0 as *const Vec<Window> as *mut Vec<Window>);
                result.push(window);
            }

            true.into()
        }
    }
}
