use crossterm::event::KeyEvent;

use super::app_state::AppStateActions;

pub enum EditorCursorDirection {
    Left,
    Right,
    Up,
    Down,
}

pub enum EditorContainerModelActions {
    Input(char),
    ModifierInput(KeyEvent),
    MoveCursor(EditorCursorDirection),
    // todo figure out how to handle ctrl+something
    ChangeFocus(u16),
    CloseEditor(u16),
    ToggleLines,
}

type EditorLine = (String, String);

#[derive(Debug, Default)]
struct EditorModel {
    data: Vec<EditorLine>,
    visible_lines: Vec<EditorLine>,
}

#[derive(Debug, Default)]
pub struct EditorContainerModel {
    active_editor_index: u16,
    editors: Vec<EditorModel>,
}

impl EditorContainerModel {
    pub fn update(&mut self, action: EditorContainerModelActions) -> Option<AppStateActions> {
        match action {
            EditorContainerModelActions::Input(_) => None,
            EditorContainerModelActions::MoveCursor(_) => None,
            EditorContainerModelActions::ChangeFocus(_) => None,
            EditorContainerModelActions::CloseEditor(_) => None,
            EditorContainerModelActions::ToggleLines => None,
            EditorContainerModelActions::ModifierInput(_) => None,
        }
    }
}
