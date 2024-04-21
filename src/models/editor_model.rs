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
    Backspace,
    ModifierInput(KeyEvent),
    MoveCursor(EditorCursorDirection),
    // todo figure out how to handle ctrl+something
    ChangeFocus(u16),
    AddEditor,
    CloseEditor(u16),
    ToggleLines,
}

// todo, this is not the best way to do this
type EditorLine = (usize, String);

#[derive(Debug)]
pub struct EditorModel {
    data: Vec<EditorLine>,
    visible_lines: Vec<EditorLine>,
    cursor_position: (u16, u16),
}

impl Default for EditorModel {
    fn default() -> Self {
        EditorModel {
            data: Vec::from([
                (1, String::from("awdawd"))
            ]),
            visible_lines: Vec::from([
                (1, String::from("awdawd")),
            ]),
            cursor_position: (0, 0),
        }
    }
}

impl EditorModel {
    pub fn get_visible_lines(&self) -> Vec<EditorLine>{
        self.visible_lines.clone()
    }    
}

#[derive(Debug)]
pub struct EditorContainerModel {
    active_editor_index: u16,
    editors: Vec<EditorModel>,
}

impl Default for EditorContainerModel{
    fn default() -> Self {
        EditorContainerModel {
            active_editor_index: 0,
            editors: Vec::from([
                EditorModel::default(),
            ]),
        }
    }
}

impl EditorContainerModel {
    pub fn update(&mut self, action: EditorContainerModelActions) -> Option<AppStateActions> {
        match action {
            EditorContainerModelActions::Input(_) => None,
            EditorContainerModelActions::MoveCursor(_) => None,
            EditorContainerModelActions::ChangeFocus(_) => None,
            EditorContainerModelActions::ToggleLines => None,
            EditorContainerModelActions::CloseEditor(_) => None,
            EditorContainerModelActions::AddEditor => {
                // for now, we don't allow more than two editors, maybe in the future
                // for now I need to actually implement this shit lmao
                if self.editors.len() >= 2 {
                    return None;
                }
                
                None
            },
            EditorContainerModelActions::ModifierInput(_) => None,
            EditorContainerModelActions::Backspace => todo!(),
            
        }
    }

    pub fn get_editors(&self) -> &Vec<EditorModel> {
        &self.editors
    }
}
