use std::ffi::c_int;

use raylib::{DrawRectangle, Font, GetGlyphAtlasRec, GetGlyphInfo};

use crate::{editor::Editor, Position};

#[derive(Debug)]
pub struct Cursor {
    pub position: Position,
}

impl Cursor {
    pub const fn new() -> Cursor {
        Cursor {
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn set_column(&mut self, column: usize, text: &Vec<String>) {
        self.position.x = text[self.position.y].len().min(column);
    }

    pub fn set_row(&mut self, row: usize) {
        self.position.y = row;
    }

    pub fn forward(&mut self, pos: isize, text: &Vec<String>) {
        if text.len() > self.position.y {
            self.position.x = text[self.position.y]
                .len()
                .min(self.position.x.saturating_add_signed(pos));
        } else {
            self.position.x = 0;
        }
    }

    pub fn down(&mut self, pos: isize) {
        self.position.y = self.position.y.saturating_add_signed(pos);
    }

    pub fn draw_cursor(&self, editor: &Editor) {
        let mut posx = 0;

        let font_size = editor.config.style.text_font_size;

        let scale_factor = font_size as f32 / editor.font.baseSize as f32;

        if editor.text.len() > self.position.y {
            let mut chars = editor.text[self.position.y].chars();

            for i in 0..self.position.x {
                match chars.next() {
                    Some(c) => {
                        let info = unsafe { GetGlyphInfo(editor.font, c as c_int) };

                        let rec = unsafe { GetGlyphAtlasRec(editor.font, c as c_int) };
                        if info.advanceX != 0 {
                            posx += info.advanceX;
                        } else {
                            posx += rec.width as c_int + info.offsetX;
                        }
                    }
                    None => {},
                }
            }
            posx = (posx as f32 * scale_factor) as c_int;
        }



        unsafe {
            DrawRectangle(
                posx as c_int,
                (self.position.y * font_size as usize) as c_int,
                editor.config.style.text_font_size / 2,
                editor.config.style.text_font_size,
                editor.config.style.cursor_color.into(),
            )
        }
    }
}
