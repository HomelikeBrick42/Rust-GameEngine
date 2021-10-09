#![allow(dead_code)]

use core::ptr::{null, null_mut};

use crate::win32::*;
use std::process::exit;
use std::mem::size_of_val;

mod vec3;
mod win32;

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
unsafe extern "system" fn window_message_callback(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    if uMsg == WM_NCCREATE {
        let create_struct = lParam as LPVOID as *mut CREATESTRUCTA;
        SetWindowLongPtrA(hWnd, GWLP_USERDATA, (*create_struct).lpCreateParams as LONG_PTR);
        DefWindowProcA(hWnd, uMsg, wParam, lParam)
    } else {
        match uMsg {
            WM_CLOSE | WM_DESTROY => {
                let running = GetWindowLongPtrA(hWnd, GWLP_USERDATA) as LPVOID as *mut bool;
                *running = false;
                0
            }
            _ => DefWindowProcA(hWnd, uMsg, wParam, lParam),
        }
    }
}

fn main() {
    const WINDOW_CLASS_NAME: &str = "WindowClassName\0";
    const WINDOW_TITLE: &str = "Test Window\0";
    const WINDOW_STYLE: DWORD = WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX | WS_CAPTION;
    const WINDOW_STYLE_EX: DWORD = 0;
    const WINDOW_WIDTH: u32 = 640;
    const WINDOW_HEIGHT: u32 = 480;

    let instance = unsafe { GetModuleHandleA(null()) };

    unsafe {
        let mut window_class = WNDCLASSEXA::default();

        window_class.cbSize = size_of_val(&window_class) as UINT;
        window_class.hInstance = instance;
        window_class.lpfnWndProc = Some(window_message_callback);
        window_class.lpszClassName = WINDOW_CLASS_NAME.as_ptr();
        window_class.hCursor = LoadCursorA(null_mut(), IDC_ARROW);

        let result = RegisterClassExA(&window_class);
        if result == 0 {
            eprintln!("Failed to register window class with code: {}", GetLastError());
            exit(1);
        }
    }

    let mut running = true;

    let window = unsafe {
        let mut rect = RECT::default();
        rect.left = 100;
        rect.right = rect.left + WINDOW_WIDTH as i32;
        rect.top = 100;
        rect.bottom = rect.top + WINDOW_HEIGHT as i32;

        if AdjustWindowRectEx(&mut rect, WINDOW_STYLE, 0, WINDOW_STYLE_EX) == 0 {
            eprintln!("Failed to calculate window width and height with code: {}", GetLastError());
            exit(1);
        }

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        CreateWindowExA(
            WINDOW_STYLE_EX,
            WINDOW_CLASS_NAME.as_ptr(),
            WINDOW_TITLE.as_ptr(),
            WINDOW_STYLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            null_mut(),
            null_mut(),
            instance,
            &mut running as *mut bool as LPVOID,
        )
    };
    if window.is_null() {
        eprintln!("Failed to create window with code: {}", unsafe { GetLastError() });
        exit(1);
    }

    unsafe { ShowWindow(window, SW_SHOW) };

    while running {
        unsafe {
            let mut message = MSG::default();
            while PeekMessageA(&mut message, window, 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&message);
                DispatchMessageA(&message);
            }
        }
    }

    unsafe {
        DestroyWindow(window);
        UnregisterClassA(WINDOW_CLASS_NAME.as_ptr(), instance);
    }
}
