use std::{
    cmp::{max, min},
    usize,
};

use crossterm::cursor::position;
use ratatui::layout::Rect;

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
    AddLine,
    Undo,
    Redo,
}

#[derive(Debug, Clone)]
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
        let mut editor = EditorModel {
            data: Vec::from([(1, String::from("awdawd"))]),
            history: Vec::<EditorBackupModel>::new(),
            current_history_index: 0,
            current_size: Rect::default(),
            visible_lines: (0, 1),
            cursor_position: (0, 0),
        };

        editor.backup();
        editor
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
        let position = self.cursor_position.1 as usize;
        let last_line_number = self.data[position].0;
        self.data
            .insert(position + 1, (last_line_number + 1, String::from("")));

        self.data
            .iter_mut()
            .skip(position + 2)
            .for_each(|line| line.0 += 1);
        self.cursor_position.1 += 1;
        self.cursor_position.0 = 0;

        self.update_visible_lines(1);
    }

    fn delete_line(&mut self) {
        let position = self.cursor_position.1 as usize;

        // we're asked to remove the last line, no point
        if position == 0 {
            return;
        }

        if !self.data[position].1.is_empty() {
            let previous_position = position.saturating_sub(1);
            self.data[previous_position].1 = {
                let mut previous_line = self.data[previous_position].1.clone();
                previous_line += self.data[position].1.as_str();
                previous_line
            }
        }

        self.data.remove(position);
        self.cursor_position.1 -= 1;
        self.data
            .iter_mut()
            .skip(self.cursor_position.1 as usize + 1)
            .for_each(|line| line.0 -= 1);
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
        self.move_cursor(EditorCursorDirection::Right);
    }

    fn handle_backspace(&mut self) {
        let (row, col) = self.cursor_position;
        if row == 0 {
            return self.delete_line();
        } else {
            let before_char_to_delete = self.data[col as usize].1.chars().take(row as usize - 1);
            let after_chars_to_delete = self.data[col as usize].1.chars().skip(row as usize);
            self.data[col as usize] = (
                self.data[col as usize].0,
                before_char_to_delete.chain(after_chars_to_delete).collect(),
            );
            self.move_cursor(EditorCursorDirection::Left)
        }
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
                self.backup();
            }
            EditorModelActions::MoveCursor(direction) => self.move_cursor(direction),
            EditorModelActions::AddLine => {
                self.add_line();
                self.backup();
            }
            EditorModelActions::Backspace => {
                self.handle_backspace();
                self.backup();
            }
            EditorModelActions::Undo => {
                self.restore();
            }
            EditorModelActions::Redo => {
                self.undo_restore();
                self.update_visible_lines(0);
            }
        }
    }
}

impl History for EditorModel {
    fn restore(&mut self) {
        // we want to restore the state before the current change, hence -2
        if let Some(last_entry) = self
            .history
            .get(self.current_history_index.saturating_sub(2))
        {
            self.data = last_entry.data.clone();
            self.cursor_position = last_entry.cursor_position;
            self.current_size = last_entry.current_size;
            self.visible_lines = last_entry.visible_lines.clone();
            self.current_history_index = self.current_history_index.saturating_sub(1);
        }
    }

    fn undo_restore(&mut self) {
        if let Some(last_entry) = self.history.get(self.current_history_index) {
            self.data = last_entry.data.clone();
            self.cursor_position = last_entry.cursor_position;
            self.current_size = last_entry.current_size;
            self.current_history_index = min(self.history.len(), self.current_history_index + 1);
        }
    }

    fn backup(&mut self) {
        // we want to keep the default state of 1 empty line.
        // allowing the index to go to 0 and then truncating the history
        // would wipe everything. So then the new state with changes
        // would become the default state instead

        if self.current_history_index >= 1 {
            self.history.truncate(self.current_history_index);
        }

        self.history.push(EditorBackupModel::new(
            self.data.clone(),
            self.get_cursor_position(),
            self.current_size.clone(),
            self.visible_lines,
        ));
        self.current_history_index += 1;
    }

    fn save(self) -> String {
        "".to_string()
    }
}
