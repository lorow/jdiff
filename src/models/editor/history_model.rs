use ratatui::layout::Rect;

use super::types::{CursorPosition, EditorLine};

#[derive(Debug)]
pub struct EditorBackupModel {
    pub data: Vec<EditorLine>,
    pub cursor_position: CursorPosition,
    pub current_size: Rect,
    pub visible_lines: (u16, u16), // not sure if I need those
}

impl EditorBackupModel {
    pub fn new(
        data: Vec<EditorLine>,
        cursor_position: CursorPosition,
        current_size: Rect,
        visible_lines: (u16, u16),
    ) -> Self {
        Self {
            data,
            cursor_position,
            current_size,
            visible_lines,
        }
    }
}
