use crate::editor;

enum Actions {
    ARROW_UP,
    ARROW_DOWN,
    ARROW_RIGHT,
    ARROW_LEFT,
    EXIT,
    SWITCH_MODE(editor::EditorState),
}
