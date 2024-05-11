use std::{
    cmp::{max, min}, ops::Sub, usize
};

use crossterm::event::KeyEvent;
use ratatui::{layout::{Direction, Rect}, Frame};

use super::app_state::AppStateActions;

pub enum EditorCursorDirection {
    Left,
    Right,
    Up,
    Down,
}

pub enum EditorFocus {
    Next,
    Prev,
}

pub enum EditorContainerModelActions {
    InitEditor(Rect),

    ToggleResize,
    ResizeEditor(Rect),

    Input(char),
    Backspace,
    Enter,
    MoveCursor(EditorCursorDirection),
    // todo figure out how to handle ctrl+something
    ChangeFocus(EditorFocus),
    AddEditor,
    CloseEditor,
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
            self.visible_lines.0 = max(self.visible_lines.0, 0);
            self.visible_lines.1 = min(self.visible_lines.1, self.data.len() as u16);
        }
    }

    fn move_cursor(&mut self, direction: EditorCursorDirection) {
        match direction {
            EditorCursorDirection::Left => {
                self.cursor_position.0 = self.cursor_position.0.checked_sub(1).unwrap_or(0);
            },
            EditorCursorDirection::Right => {
                self.cursor_position.0 = min(self.cursor_position.0 + 1, self.data[self.cursor_position.1 as usize].1.len() as u16 );
            },
            EditorCursorDirection::Up => {
                self.cursor_position.1 =  self.cursor_position.1.checked_sub(1).unwrap_or(0);
                self.cursor_position.0 = min(self.cursor_position.0, self.data[self.cursor_position.1 as usize].1.len() as u16 );
                self.update_visible_lines(1);
            },
            EditorCursorDirection::Down => {
                self.cursor_position.1 = min(self.cursor_position.1 + 1, self.data.len() as u16 - 1);
                self.cursor_position.0 = min(self.cursor_position.0, self.data[self.cursor_position.1 as usize].1.len() as u16 );
                self.update_visible_lines(-1);
            },
        }
    }
}

#[derive(Debug)]
pub struct EditorContainerModel {
    initialized: bool,
    resized: bool,
    active_editor_index: u16,
    editors: Vec<EditorModel>,
}

impl Default for EditorContainerModel {
    fn default() -> Self {
        EditorContainerModel {
            initialized: false,
            resized: false,
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
            EditorContainerModelActions::MoveCursor(direction) => {
                self.editors[self.active_editor_index as usize].move_cursor(direction);
                return None; 
            },
            EditorContainerModelActions::ChangeFocus(direction) => match direction {
                EditorFocus::Next => {
                    self.active_editor_index = min(
                        self.active_editor_index + 1,
                        (self.editors.len() - 1) as u16,
                    );
                    None
                }
                EditorFocus::Prev => {
                    self.active_editor_index = max(0, self.active_editor_index - 1);
                    None
                }
            },
            EditorContainerModelActions::ToggleLines => None,
            EditorContainerModelActions::CloseEditor => None,
            EditorContainerModelActions::AddEditor => {
                // for now, we don't allow more than two editors, maybe in the future
                // for now I need to actually implement this shit lmao
                if self.editors.len() >= 2 {
                    return None;
                }
                self.editors.push(EditorModel::default());

                None
            }
            EditorContainerModelActions::Backspace => todo!(),
            EditorContainerModelActions::InitEditor(rect) => {
                self.initialized = true;
                self.editors.iter_mut().for_each(|editor| {
                    editor.visible_lines = (0, rect.height);
                    editor.update_visible_lines(-1)
                });

                None
            }
            EditorContainerModelActions::ToggleResize => {
                self.resized = true;
                None
            }
            EditorContainerModelActions::ResizeEditor(rect) => {
                self.resized = false;
                self.editors.iter_mut().for_each(|editor| {
                    editor.visible_lines = (0, rect.height);
                    editor.update_visible_lines(-1)
                });
                None
            }
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

    pub fn get_is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn get_is_resized_set(&self) -> bool {
        self.resized
    }
}
