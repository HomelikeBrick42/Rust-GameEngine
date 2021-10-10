use std::cell::RefCell;
use std::sync::Arc;
use crate::window::Window;

pub trait Renderer {
    fn present(&mut self) -> Result<(), String>;
    fn get_window(&self) -> Arc<RefCell<Window>>;
}
