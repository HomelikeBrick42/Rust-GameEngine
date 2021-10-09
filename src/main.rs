#![allow(dead_code)]

use std::cell::RefCell;
use std::sync::Arc;

use crate::opengl_renderer::OpenGLRenderer;
use crate::window::*;

mod vec3;
mod win32;
mod window;
mod renderer;
mod opengl_renderer;

fn main() {
    let window = Arc::new(RefCell::new(Window::new(640, 480, "Test Window").unwrap()));
    let renderer = OpenGLRenderer::new(window.clone()).unwrap();

    window.borrow_mut().show();
    while !window.borrow_mut().should_close() {
        window.borrow_mut().poll_messages();
        renderer.borrow_mut().present().unwrap();
    }
    window.borrow_mut().hide();
}
