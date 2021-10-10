use std::cell::RefCell;
use std::mem::size_of;
use std::sync::Arc;

use crate::renderer::Renderer;
use crate::win32::*;
use crate::window::Window;

type HGLRC = HANDLE;

#[link(name = "OpenGL32")]
extern "system" {
    pub fn wglCreateContext(hdc: HDC) -> HGLRC;
    pub fn wglDeleteContext(hglrc: HGLRC) -> BOOL;
    pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> BOOL;
}

pub struct OpenGLRenderer {
    window: Arc<RefCell<Window>>,
    context: HGLRC,
}

impl OpenGLRenderer {
    pub fn new(window: Arc<RefCell<Window>>) -> Result<Arc<RefCell<dyn Renderer>>, String> {
        let device_context = unsafe { window.borrow().get_device_context() };

        let pixel_format_descriptor = PIXELFORMATDESCRIPTOR {
            nSize: size_of::<PIXELFORMATDESCRIPTOR>() as WORD,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_DOUBLEBUFFER | PFD_SUPPORT_OPENGL,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cRedBits: 0,
            cRedShift: 0,
            cGreenBits: 0,
            cGreenShift: 0,
            cBlueBits: 0,
            cBlueShift: 0,
            cAlphaBits: 0,
            cAlphaShift: 0,
            cAccumBits: 0,
            cAccumRedBits: 0,
            cAccumGreenBits: 0,
            cAccumBlueBits: 0,
            cAccumAlphaBits: 0,
            cDepthBits: 24,
            cStencilBits: 8,
            cAuxBuffers: 0,
            iLayerType: PFD_MAIN_PLANE,
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: 0,
            dwDamageMask: 0,
        };

        let pixel_format = unsafe { ChoosePixelFormat(device_context, &pixel_format_descriptor) };
        if pixel_format == 0 {
            return Err(format!("Failed to choose pixel format with code: {}", unsafe { GetLastError() }).to_string());
        }

        if unsafe { SetPixelFormat(device_context, pixel_format, &pixel_format_descriptor) } == 0 {
            return Err(format!("Failed to set pixel format with code: {}", unsafe { GetLastError() }).to_string());
        }

        let context = unsafe { wglCreateContext(device_context) };
        if context.is_null() {
            return Err(format!("Failed to create opengl context with code: {}", unsafe { GetLastError() }).to_string());
        }

        if unsafe { wglMakeCurrent(device_context, context) } == 0 {
            return Err(format!("Failed to make opengl context current with code: {}", unsafe { GetLastError() }).to_string());
        }

        Ok(Arc::new(RefCell::new(OpenGLRenderer {
            window,
            context,
        })))
    }
}

impl Drop for OpenGLRenderer {
    fn drop(&mut self) {
        unsafe { wglDeleteContext(self.context) };
    }
}

impl Renderer for OpenGLRenderer {
    fn present(&mut self) -> Result<(), String> {
        unsafe {
            let success = SwapBuffers(self.window.borrow().get_device_context());
            if success == 0 {
                Err(format!("Failed to swap buffers with code: {}", GetLastError()).to_string())
            } else {
                Ok(())
            }
        }
    }

    fn get_window(&self) -> Arc<RefCell<Window>> {
        self.window.clone()
    }
}
