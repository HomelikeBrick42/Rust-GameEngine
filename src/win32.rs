#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
mod internal {
    use std::os::raw::{c_int, c_uint};

    pub type VOID = std::ffi::c_void;
    pub type UINT = c_uint;
    pub type HANDLE = *mut VOID;
    pub type UINT_PTR = *mut UINT;
    pub type LONG_PTR = isize;
    pub type LPCSTR = *const u8;
    pub type WORD = u16;
    pub type BOOL = c_int;

    pub type HICON = HANDLE;
    pub type HINSTANCE = HANDLE;
    pub type HCURSOR = HICON;
    pub type HBRUSH = HANDLE;
    pub type HMODULE = HANDLE;
    pub type HWND = HANDLE;
    pub type WPARAM = UINT_PTR;
    pub type LPARAM = LONG_PTR;
    pub type LRESULT = LONG_PTR;
    pub type ATOM = WORD;

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

    #[link(name = "Kernel32")]
    extern "system" {
        pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    }

    #[link(name = "User32")]
    extern "system" {
        pub fn RegisterClassExA(lpWndClass: *const WNDCLASSEXA) -> ATOM;
        pub fn UnregisterClassA(lpClassName: LPCSTR, hInstance: HINSTANCE) -> BOOL;
    }
}

pub use crate::win32::internal::*;
