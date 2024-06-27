use ratatui::layout::Rect;
use std::{
    cmp::{max, min},
    usize,
};

use super::{
    editor_traits::History,
    history_model::EditorBackupModel,
    types::{CursorPosition, EditorLine},
};

#[derive(Debug)]
pub enum EditorCursorDirection {
    Left,
    Right,
    Up,
    Down,
}

// todo rethink how editors should work once we have multiselect
pub enum EditorModelActions {
    Input(char),
    MoveCursor(EditorCursorDirection),
    Backspace,
    DeleteLine,
    AddLine,
    Undo,
    Redo,
}

#[derive(Debug)]
pub struct EditorModel {
    data: Vec<EditorLine>,
    history: Vec<EditorBackupModel>,
    current_history_index: usize,
    current_size: Rect,
    visible_lines: (u16, u16),
    cursor_position: CursorPosition,
}

impl Default for EditorModel {
    fn default() -> Self {
        EditorModel {
            data: Vec::from([(1, String::from("awdawd"))]),
            history: Vec::<EditorBackupModel>::new(),
            current_history_index: 0,
            current_size: Rect::default(),
            visible_lines: (0, 1),
            cursor_position: (0, 0),
        }
    }
}

impl EditorModel {
    pub fn resize(&mut self, rect: Rect) {
        self.visible_lines = (0, rect.height);
        self.update_visible_lines(-1);
    }

    pub fn get_visible_lines(&self) -> Vec<EditorLine> {
        self.data[self.visible_lines.0 as usize..self.visible_lines.1 as usize].to_vec()
    }

    pub fn get_cursor_position(&self) -> (u16, u16) {
        self.cursor_position
    }

    fn add_line(&mut self) {
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

    fn delete_line(&mut self) {
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

    fn handle_input(&mut self, c: char) {
        let position = self.cursor_position.1 as usize;
        self.data[position]
            .1
            .insert(self.cursor_position.0 as usize, c);
        self.cursor_position.0 += 1;
    }

    fn move_cursor(&mut self, direction: EditorCursorDirection) {
        match direction {
            EditorCursorDirection::Left => {
                self.cursor_position.0 = self.cursor_position.0.checked_sub(1).unwrap_or(0);
            }
            EditorCursorDirection::Right => {
                self.cursor_position.0 = min(
                    self.cursor_position.0 + 1,
                    self.data[self.cursor_position.1 as usize].1.len() as u16,
                );
            }
            EditorCursorDirection::Up => {
                self.cursor_position.1 = self.cursor_position.1.checked_sub(1).unwrap_or(0);
                self.cursor_position.0 = min(
                    self.cursor_position.0,
                    self.data[self.cursor_position.1 as usize].1.len() as u16,
                );
                self.update_visible_lines(1);
            }
            EditorCursorDirection::Down => {
                self.cursor_position.1 =
                    min(self.cursor_position.1 + 1, self.data.len() as u16 - 1);

                self.cursor_position.0 = min(
                    self.cursor_position.0,
                    self.data[self.cursor_position.1 as usize].1.len() as u16,
                );
                self.update_visible_lines(-1);
            }
        }
    }

    pub fn handle_action(&mut self, action: EditorModelActions) {
        match action {
            EditorModelActions::Input(char) => {
                self.handle_input(char);
            }
            EditorModelActions::MoveCursor(direction) => self.move_cursor(direction),
            EditorModelActions::AddLine => {
                self.add_line();
            }
            EditorModelActions::Backspace => todo!(),
            EditorModelActions::DeleteLine => {
                self.delete_line();
            }
            EditorModelActions::Undo => self.restore(),
            EditorModelActions::Redo => self.undo_restore(),
        }
    }
}

impl History for EditorModel {
    fn restore(&mut self) {
        if let Some(last_entry) = self.history.get(self.current_history_index) {
            self.data = last_entry.data.clone();
            self.cursor_position = last_entry.cursor_position;
            self.current_size = last_entry.current_size;
            self.current_history_index = min(0, self.current_history_index - 1);
        }
    }

    fn undo_restore(&mut self) {
        if let Some(last_entry) = self.history.get(self.current_history_index) {
            self.data = last_entry.data.clone();
            self.cursor_position = last_entry.cursor_position;
            self.current_size = last_entry.current_size;
            self.current_history_index =
                min(self.current_history_index, self.current_history_index + 1);
        }
    }

    fn backup(&mut self) {
        self.history.truncate(self.current_history_index);
        self.history.push(EditorBackupModel::new(
            self.data.clone(),
            self.get_cursor_position(),
            self.current_size.clone(),
        ))
    }

    fn save(self) -> String {
        "".to_string()
    }
}
