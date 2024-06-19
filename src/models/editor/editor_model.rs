use ratatui::layout::Rect;
use std::{
    cmp::{max, min},
    usize,
};

type LineNumber = usize;
type EditorLine = (LineNumber, String);

#[derive(Debug)]
pub enum EditorCursorDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub enum ReversibleEditorModelActions {
    Input(char),
    AddLine,
}

pub enum EditorModelActions {
    Input(char),
    MoveCursor(EditorCursorDirection),
    AddLine,
}

#[derive(Debug)]
pub struct EditorModel {
    data: Vec<EditorLine>,
    actions_stack: Vec<ReversibleEditorModelActions>,
    current_action: u64,
    current_size: Rect,
    visible_lines: (u16, u16),
    cursor_position: (u16, u16),
}

impl Default for EditorModel {
    fn default() -> Self {
        EditorModel {
            data: Vec::from([(1, String::from("awdawd"))]),
            actions_stack: Vec::new(),
            current_action: 0,
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

    pub fn handle_action(&mut self, action: EditorModelActions) {}
}
