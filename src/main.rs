#![allow(dead_code)]

mod vec3;
mod win32;
mod window;

use crate::window::*;

fn main() {
    let mut window = Window::new(640, 480, "Test Window").unwrap();
    window.show();
    while !window.should_close() {
        window.poll_messages();
    }
}
