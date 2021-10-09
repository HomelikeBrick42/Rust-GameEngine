#![allow(dead_code)]

use crate::win32::*;

mod vec3;
mod win32;

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
unsafe extern "system" fn window_message_callback(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unimplemented!()
}

fn main() {
    const WINDOW_CLASS_NAME: LPCSTR = "WindowClassName\0".as_ptr();
    let instance = unsafe { GetModuleHandleA(std::ptr::null()) };

    unsafe {
        let mut window_class = WNDCLASSEXA::default();

        window_class.hInstance = instance;
        window_class.lpfnWndProc = Some(window_message_callback);
        window_class.lpszClassName = WINDOW_CLASS_NAME;

        let result = RegisterClassExA(&window_class);
        if result != 0 {
            eprintln!("Failed to register window class with code: {}", result);
        }
    }

    unsafe {
        UnregisterClassA(WINDOW_CLASS_NAME, instance);
    }
}
