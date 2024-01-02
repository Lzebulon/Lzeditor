pub mod color;
pub mod cursor;
pub mod editor;
pub mod keybind;
pub mod keyboard;
//pub mod editor_mode;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub type Position = Vec2;
