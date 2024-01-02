use std::ffi::c_char;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

extern crate raylib;
use lzeditor::editor::{Editor, EditorState};
use raylib::*;

fn main() {
    let mut args = args();

    let file_name = args.nth(1).unwrap_or("a.txt".to_string());

    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .expect("");

    let reader = BufReader::new(file.try_clone().unwrap());

    let mut text = vec![];

    let lines = reader.lines();

    for l in lines {
        text.push(l.unwrap());
    }

    let font = unsafe { GetFontDefault() };

    let mut editor = Editor::new(file, font);

    editor.text = text;

    let background_color = editor.config.style.background_color;
    let text_color = editor.config.style.text_color;
    let text_font_size = editor.config.style.text_font_size;

    let cursor_color = editor.config.style.cursor_color;

    unsafe {
        raylib::InitWindow(0, 0, raylib_str!("lzeditor"));
        SetExitKey(-1);

        //let font = GetFontDefault();
        let font = LoadFont(raylib_str!("/usr/local/share/fonts/f/FiraMonoNerdFontPropo_Regular.otf"));
        editor.font = font;
        while !WindowShouldClose() {
            if IsKeyPressed(KeyboardKey_KEY_A.try_into().unwrap())
                && editor.state == EditorState::IDLE
            {
                break;
            }

            editor.process_keys();

            draw!({
                ClearBackground(background_color.into());

                editor.cursor.draw_cursor(&editor);

                editor.draw_text();

                editor.draw_text_intern(
                    format!("-- {:?} --", editor.state),
                    10,
                    700,
                    text_font_size.try_into().unwrap(),
                    text_color.into(),
                );

                if editor.state == EditorState::COMMAND {
                    let command = "";
                    editor.draw_text_intern(
                        format!(": {}", command),
                        10,
                        700 + text_font_size as usize,
                        text_font_size.try_into().unwrap(),
                        text_color.into(),
                    );
                }
            });
        }

        editor.save();

        CloseWindow();
    }
}
