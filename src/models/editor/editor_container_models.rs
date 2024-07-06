use std::{cmp::min, usize};

use ratatui::layout::Rect;

use crate::models::app_state::AppStateActions;

use super::editor_model::{EditorCursorDirection, EditorModel, EditorModelActions};

#[derive(Debug)]
pub enum EditorFocus {
    Next,
    Prev,
}

#[derive(Debug)]
pub enum EditorContainerModelActions {
    InitEditor(Rect),
    ToggleResize,
    ResizeEditor(Rect),
    Input(char),
    Undo,
    Redo,
    Backspace,
    Enter,
    MoveCursor(EditorCursorDirection),
    ChangeFocus(EditorFocus),
    AddEditor,
    CloseEditor,
    ToggleLines,
}

#[derive(Debug, Clone)]
pub struct EditorContainerModel {
    initialized: bool,
    resized: bool,
    active_editor_index: usize,
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
            EditorContainerModelActions::InitEditor(rect) => {
                self.initialized = true;
                self.editors
                    .iter_mut()
                    .for_each(|editor| editor.resize(rect));
                None
            }
            EditorContainerModelActions::Undo => {
                self.editors[self.active_editor_index].handle_action(EditorModelActions::Undo);
                None
            }
            EditorContainerModelActions::Redo => {
                self.editors[self.active_editor_index].handle_action(EditorModelActions::Redo);
                None
            }
            EditorContainerModelActions::Input(c) => {
                self.editors[self.active_editor_index].handle_action(EditorModelActions::Input(c));
                // self.editors[self.active_editor_index as usize].handle_input(c);
                None
            }
            EditorContainerModelActions::Enter => {
                // self.editors[self.active_editor_index as usize].add_line();
                self.editors[self.active_editor_index].handle_action(EditorModelActions::AddLine);
                None
            }
            EditorContainerModelActions::MoveCursor(direction) => {
                // self.editors[self.active_editor_index as usize].move_cursor(direction);
                self.editors[self.active_editor_index]
                    .handle_action(EditorModelActions::MoveCursor(direction));
                None
            }
            EditorContainerModelActions::ChangeFocus(direction) => match direction {
                EditorFocus::Next => {
                    self.active_editor_index = min(
                        self.editors.len() - 1,
                        self.active_editor_index.checked_add(1).unwrap_or(0),
                    );
                    None
                }
                EditorFocus::Prev => {
                    self.active_editor_index = self.active_editor_index.checked_sub(1).unwrap_or(0);
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
            EditorContainerModelActions::Backspace => {
                self.editors[self.active_editor_index].handle_action(EditorModelActions::Backspace);
                None
            }
            EditorContainerModelActions::ToggleResize => {
                self.resized = true;
                None
            }
            EditorContainerModelActions::ResizeEditor(rect) => {
                self.resized = false;
                self.editors
                    .iter_mut()
                    .for_each(|editor| editor.resize(rect));
                None
            }
        }
    }

    pub fn get_editors(&self) -> &Vec<EditorModel> {
        &self.editors
    }

    pub fn get_active_editor_index(&self) -> usize {
        self.active_editor_index
    }

    pub fn get_active_cursor_position(&self) -> (u16, u16) {
        self.editors[self.active_editor_index as usize].get_cursor_position()
    }

    pub fn get_is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn get_is_resized_set(&self) -> bool {
        self.resized
    }
}
