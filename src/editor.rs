use std::{
    ffi::{c_char, c_float, c_int, c_uint},
    fs::{self, File},
    io::{BufWriter, Seek, Write},
    vec,
};

use raylib::{raylib_str, DrawTextEx, Font, Vector2};

use crate::{color::Color, cursor::Cursor};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum EditorState {
    IDLE,
    INSERT,
    VISUAL,
    COMMAND,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EditorStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub text_font_size: c_int,
    pub cursor_color: Color,
}

impl EditorStyle {
    const fn new() -> Self {
        EditorStyle {
            background_color: Color {
                r: 10,
                g: 10,
                b: 10,
                a: 255,
            },
            text_color: Color {
                r: 200,
                g: 200,
                b: 200,
                a: 255,
            },
            text_font_size: 20,
            cursor_color: Color {
                r: 200,
                g: 100,
                b: 100,
                a: 255,
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct EditorConfig {
    pub style: EditorStyle,
}

impl EditorConfig {
    const fn new() -> Self {
        Self {
            style: EditorStyle::new(),
        }
    }
}

#[derive(Debug)]
pub struct Editor {
    pub state: EditorState,
    pub cursor: Cursor,
    pub text: Vec<String>,
    pub font: Font,
    pub file: File,
    pub config: EditorConfig,
    pub command_line: String,
}

fn check_is_pressed_continue(key: c_uint) -> bool {
    use raylib::*;
    let key = key.try_into().unwrap();
    unsafe { IsKeyPressed(key) || IsKeyPressedRepeat(key) }
}

impl Editor {
    pub fn new(file: File, font: Font) -> Self {
        Self {
            state: EditorState::IDLE,
            cursor: Cursor::new(),
            text: vec![],
            font,
            file,
            config: EditorConfig::new(),
            command_line: String::new(),
        }
    }

    pub fn process_keys(&mut self) {
        use raylib::*;
        unsafe {
            if IsKeyPressed(KeyboardKey_KEY_ESCAPE.try_into().unwrap()) {
                self.state = EditorState::IDLE;
                for i in CharsQueue() {
                    println!("{i}");
                }
            }

            if check_is_pressed_continue(KeyboardKey_KEY_LEFT) && self.cursor.position.x > 0 {
                self.cursor.forward(-1, &self.text);
            }

            if check_is_pressed_continue(KeyboardKey_KEY_RIGHT) {
                self.cursor.forward(1, &self.text);
            }

            if check_is_pressed_continue(KeyboardKey_KEY_UP) && self.cursor.position.y > 0 {
                self.cursor.down(-1);
            }

            if check_is_pressed_continue(KeyboardKey_KEY_DOWN) {
                self.cursor.down(1);
            }

            if IsKeyPressed(KeyboardKey_KEY_I.try_into().unwrap())
                && self.state == EditorState::IDLE
            {
                self.state = EditorState::INSERT;
                return;
            }

            if check_is_pressed_continue(KeyboardKey_KEY_BACKSPACE)
                && self.state == EditorState::INSERT
            {
                let _ = self.text[self.cursor.position.y].remove(self.cursor.position.x);
                self.cursor.forward(-1, &self.text);
            }

            if IsKeyPressed(KeyboardKey_KEY_V.try_into().unwrap())
                && self.state == EditorState::IDLE
            {
                self.state = EditorState::VISUAL;
            }

            if IsKeyPressed(KeyboardKey_KEY_SEMICOLON.try_into().unwrap())
                && self.state == EditorState::IDLE
            {
                self.state = EditorState::COMMAND;
            }

            match self.state {
                EditorState::IDLE => {}
                EditorState::INSERT => loop {
                    let c = raylib::GetCharPressed();
                    if c == 0 {
                        break;
                    }

                    let c: u32 = c.try_into().expect("ah convertion failed");
                    let c = char::from_u32(c).expect("error convertion");
                    if self.cursor.position.x >= self.text[self.cursor.position.y].len() {
                        self.text[self.cursor.position.y].push(c);
                    } else {
                        self.text[self.cursor.position.y].insert(self.cursor.position.x, c);
                        self.cursor.position.x += 1;
                    }
                },
                EditorState::COMMAND => loop {
                    let c = raylib::GetCharPressed();
                    if c == 0 {
                        break;
                    }

                    let c: u32 = c.try_into().expect("ah convertion failed");
                    let c = char::from_u32(c).expect("error convertion");

                    self.command_line.insert(self.cursor.position.x, c);
                    self.cursor.position.x += 1;
                },
                EditorState::VISUAL => todo!(),
            }
        }
    }

    pub fn draw_text(&self) {
        for (line, l) in self.text.clone().into_iter().enumerate() {
            self.draw_text_intern(
                l,
                0,
                line * self.config.style.text_font_size as usize,
                self.config.style.text_font_size as usize,
                self.config.style.text_color.into(),
            );
        }
    }

    pub fn save(&self) {
        let mut text = String::new();

        for t in self.text.clone() {
            text.push_str(t.as_str());
            text.push('\n');
        }

        let mut f = self.file.try_clone().expect("cannot copy file");

        f.seek(std::io::SeekFrom::Start(0)).expect("file not empty");

        f.write(text.as_bytes()).expect("cannot write");
        f.flush().expect("cannot flush");
    }

    pub fn draw_text_intern(
        &self,
        text: String,
        x: usize,
        y: usize,
        font_size: usize,
        color: Color,
    ) {
        unsafe {
            DrawTextEx(
                self.font,
                raylib_str!(text),
                Vector2 {
                    x: x as c_float,
                    y: y as c_float,
                },
                font_size as c_float,
                0.0,
                color.into(),
            )
        }
    }
}
