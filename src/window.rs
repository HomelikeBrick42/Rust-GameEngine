use crate::win32::*;
use std::mem::size_of_val;
use std::ptr::{null_mut};

#[allow(non_snake_case)]
unsafe extern "system" fn window_message_callback(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    if uMsg == WM_NCCREATE {
        let create_struct = lParam as LPVOID as *mut CREATESTRUCTA;
        SetWindowLongPtrA(hWnd, GWLP_USERDATA, (*create_struct).lpCreateParams as LONG_PTR);
        DefWindowProcA(hWnd, uMsg, wParam, lParam)
    } else {
        let data = &mut *(GetWindowLongPtrA(hWnd, GWLP_USERDATA) as LPVOID as *mut WindowData);
        match uMsg {
            WM_CLOSE | WM_DESTROY => {
                data.should_close = true;
                0
            },
            _ => DefWindowProcA(hWnd, uMsg, wParam, lParam),
        }
    }
}

const WINDOW_STYLE: DWORD = WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX | WS_CAPTION;
const WINDOW_STYLE_EX: DWORD = 0;

const WINDOW_CLASS_NAME: &str = "WindowClassName\0";

// TODO: Make this thread safe
static mut WINDOW_COUNT: usize = 0;

#[derive(Default)]
struct WindowData {
    should_close: bool,
}

pub struct Window {
    instance: HINSTANCE,
    window_handle: HWND,
    data: Box<WindowData>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Window, String> {
        unsafe {
            let title: Vec<u8> = title.bytes().chain('\0' as u8..='\0' as u8).collect();

            let mut window = Window {
                instance: GetModuleHandleA(null_mut()),
                window_handle: null_mut(),
                data: Box::new(WindowData::default()),
            };

            if WINDOW_COUNT == 0 {
                let mut window_class = WNDCLASSEXA::default();

                window_class.cbSize = size_of_val(&window_class) as UINT;
                window_class.hInstance = window.instance;
                window_class.lpfnWndProc = Some(window_message_callback);
                window_class.lpszClassName = WINDOW_CLASS_NAME.as_ptr();
                window_class.hCursor = LoadCursorA(null_mut(), IDC_ARROW);

                let result = RegisterClassExA(&window_class);
                if result == 0 {
                    return Err(format!("Failed to register window class with code: {}", GetLastError()).to_string());
                }
            }

            let mut rect = RECT::default();
            rect.left = 100;
            rect.right = rect.left + width as i32;
            rect.top = 100;
            rect.bottom = rect.top + height as i32;

            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            if AdjustWindowRectEx(&mut rect, WINDOW_STYLE, 0, WINDOW_STYLE_EX) == 0 {
                return Err(format!("Failed to calculate window width and height with code: {}", GetLastError()).to_string());
            }

            window.window_handle = CreateWindowExA(
                WINDOW_STYLE_EX,
                WINDOW_CLASS_NAME.as_ptr(),
                title.as_ptr(),
                WINDOW_STYLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width,
                height,
                null_mut(),
                null_mut(),
                window.instance,
                window.data.as_mut() as *mut WindowData as LPVOID,
            );
            if window.window_handle.is_null() {
                return Err(format!("Failed to create window with code: {}", GetLastError()).to_string());
            }

            WINDOW_COUNT += 1;
            Ok(window)
        }
    }

    pub fn show(&mut self) {
        unsafe { ShowWindow(self.window_handle, SW_SHOW) };
    }

    pub fn hide(&mut self) {
        unsafe { ShowWindow(self.window_handle, SW_HIDE) };
    }

    pub fn poll_messages(&mut self) {
        unsafe {
            let mut message = MSG::default();
            while PeekMessageA(&mut message, self.window_handle, 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&message);
                DispatchMessageA(&message);
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.data.should_close
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.window_handle);
            WINDOW_COUNT -= 1;
            if WINDOW_COUNT == 0 {
                UnregisterClassA(WINDOW_CLASS_NAME.as_ptr(), self.instance);
            }
        }
    }
}
