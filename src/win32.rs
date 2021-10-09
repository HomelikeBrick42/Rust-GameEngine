#![allow(non_snake_case, non_camel_case_types)]

use std::os::raw::c_int;

pub type VOID = std::ffi::c_void;
pub type LPVOID = *mut VOID;
pub type UINT = u32;
pub type HANDLE = *mut VOID;
pub type UINT_PTR = *mut UINT;
pub type LONG_PTR = isize;
pub type ULONG_PTR = usize;
pub type LPCSTR = *const u8;
pub type WORD = u16;
pub type BOOL = i32;
pub type DWORD = u32;
pub type LONG = i32;

pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;
pub type HMODULE = HANDLE;
pub type HMENU = HANDLE;
pub type HWND = HANDLE;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type ATOM = WORD;

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
    | WS_CAPTION
    | WS_SYSMENU
    | WS_THICKFRAME
    | WS_MINIMIZEBOX
    | WS_MAXIMIZEBOX;
pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

pub const SW_SHOW: c_int = 5;
pub const SW_HIDE: c_int = 0;

pub const PM_REMOVE: UINT = 0x0001;

pub const WM_CLOSE: UINT = 0x0010;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;

pub const GWLP_USERDATA: c_int = -21;

pub const fn MAKEINTRESOURCEW(i: WORD) -> LPCSTR {
    i as ULONG_PTR as LPCSTR
}

pub const IDC_ARROW: LPCSTR = MAKEINTRESOURCEW(32512);

pub type WNDPROC = Option<
    unsafe extern "system" fn(
        hWnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT,
>;

macro_rules! unsafe_impl_default_zeroed {
        ($t:ty) => {
            impl Default for $t {
                #[inline]
                #[must_use]
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
        };
    }

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub right: LONG,
    pub top: LONG,
    pub bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);

pub type LPRECT = *mut RECT;

#[repr(C)]
pub struct WNDCLASSEXA {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR,
    pub hIconSm: HICON,
}
unsafe_impl_default_zeroed!(WNDCLASSEXA);

#[repr(C)]
pub struct POINT {
    x: LONG,
    y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

pub type LPMSG = *mut MSG;

pub struct CREATESTRUCTA {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCSTR,
    pub lpszClass: LPCSTR,
    pub dwExStyle: DWORD,
}
unsafe_impl_default_zeroed!(CREATESTRUCTA);

#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn GetLastError() -> DWORD;
}

#[link(name = "User32")]
extern "system" {
    pub fn RegisterClassExA(lpWndClass: *const WNDCLASSEXA) -> ATOM;
    pub fn UnregisterClassA(lpClassName: LPCSTR, hInstance: HINSTANCE) -> BOOL;
    pub fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn AdjustWindowRectEx(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD) -> BOOL;
    pub fn PeekMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageA(lpMsg: *const MSG) -> LRESULT;
    pub fn LoadCursorA(hInstance: HINSTANCE, lpCursorName: LPCSTR) -> HCURSOR;
    pub fn PostQuitMessage(nExitCode: c_int);
    pub fn GetWindowLongPtrA(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
    pub fn SetWindowLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
}
