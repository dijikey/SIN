#[allow(dead_code)]
mod color;
mod sprite;

#[derive(Debug, Copy, Clone)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone)]
pub struct Sprite{
    pub width: u32,
    pub height: u32,
    pub(crate) rgba: Vec<Color>,
}