#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Key {
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_X,
    KEY_Y,
    KEY_Z,
    KEY_UP,
    KEY_DOWN,
    KEY_RIGHT,
    KEY_LEFT,
    ESCAPE,
    UNDEFINED,
}

impl From<raylib::KeyboardKey> for Key {
    fn from(value: raylib::KeyboardKey) -> Self {
        match value {
            raylib::KeyboardKey_KEY_A => Key::KEY_Q,
            raylib::KeyboardKey_KEY_Q => Key::KEY_A,
            raylib::KeyboardKey_KEY_UP => Key::KEY_UP,
            raylib::KeyboardKey_KEY_DOWN => Key::KEY_DOWN,
            raylib::KeyboardKey_KEY_RIGHT => Key::KEY_RIGHT,
            raylib::KeyboardKey_KEY_LEFT => Key::KEY_LEFT,
            _ => Key::UNDEFINED,
        }
    }
}
