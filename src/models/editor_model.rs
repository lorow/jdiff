use std::cmp::min;

use crossterm::event::KeyEvent;
use ratatui::layout::Rect;

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
    Enter,
    ModifierInput(KeyEvent),
    MoveCursor(EditorCursorDirection),
    // todo figure out how to handle ctrl+something
    ChangeFocus(u16),
    AddEditor,
    CloseEditor(u16),
    ToggleLines,
}

type LineNumber = usize;
// todo, this is not the best way to do this
type EditorLine = (LineNumber, String);

#[derive(Debug)]
pub struct EditorModel {
    data: Vec<EditorLine>,
    current_size: Rect,
    visible_lines: (u16, u16),
    cursor_position: (u16, u16),
}

impl Default for EditorModel {
    fn default() -> Self {
        EditorModel {
            data: Vec::from([(1, String::from("awdawd"))]),
            // TODO I need to somehow pass the size here
            // TODO how though?
            current_size: Rect::default(),
            visible_lines: (0, 1),
            cursor_position: (0, 0),
        }
    }
}

impl EditorModel {
    pub fn get_visible_lines(&self) -> Vec<EditorLine> {
        self.data[self.visible_lines.0 as usize..self.visible_lines.1 as usize].to_vec()
    }

    pub fn add_line(&mut self) {
        let position = self.cursor_position.0 as usize;
        let last_line_number = self.data[position].0;
        self.data
            .insert(position + 1, (last_line_number + 1, String::from("")));

        self.data
            .iter_mut()
            .skip(position + 2)
            .for_each(|line| line.0 += 1);

        self.cursor_position.1 += 1;
        self.update_visible_lines(1);
    }

    pub fn delete_line(&mut self) {
        let position = self.cursor_position.0 as usize;
        self.data.remove(position);
        self.cursor_position.1 -= 1;
        self.update_visible_lines(-1);
    }

    fn update_visible_lines(&mut self, direction: i16) {
        if self.data.len() >= self.current_size.height.into() {
            self.visible_lines = (0, self.data.len() as u16);
        } else {
            match direction {
                1 => {
                    self.visible_lines.0 += 1;
                    self.visible_lines.1 += 1;
                }
                -1 => {
                    self.visible_lines.0 -= 1;
                    self.visible_lines.1 -= 1;
                }
                _ => {}
            }
            self.visible_lines.1 = min(self.visible_lines.1, self.data.len() as u16);
        }
        print!("{:?}", self.visible_lines);
    }
}

#[derive(Debug)]
pub struct EditorContainerModel {
    active_editor_index: u16,
    editors: Vec<EditorModel>,
}

impl Default for EditorContainerModel {
    fn default() -> Self {
        EditorContainerModel {
            active_editor_index: 0,
            editors: Vec::from([EditorModel::default()]),
        }
    }
}

impl EditorContainerModel {
    pub fn update(&mut self, action: EditorContainerModelActions) -> Option<AppStateActions> {
        match action {
            EditorContainerModelActions::Input(c) => None,
            EditorContainerModelActions::Enter => {
                self.editors[self.active_editor_index as usize].add_line();
                None
            }
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
            }
            EditorContainerModelActions::ModifierInput(_) => None,
            EditorContainerModelActions::Backspace => todo!(),
        }
    }

    pub fn get_editors(&self) -> &Vec<EditorModel> {
        &self.editors
    }

    pub fn get_active_editor_index(&self) -> u16 {
        self.active_editor_index
    }

    pub fn get_active_cursor_position(&self) -> (u16, u16) {
        self.editors[self.active_editor_index as usize].cursor_position
    }
}
