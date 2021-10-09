pub trait Renderer {
    fn present(&mut self) -> Result<(), String>;
}
