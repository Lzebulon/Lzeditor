use raylib::KeyboardKey;

use crate::editor::Editor;


pub trait EditorMode {
    fn process_keys(editor : &mut Editor, key : KeyboardKey);
}

pub struct DefaultMode();

impl EditorMode for DefaultMode {

}
