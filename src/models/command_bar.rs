use crate::models::{app_model::AppModelActions, app_state::AppStateActions};

use super::app_model::AppMode;

#[derive(Debug)]
pub struct CommandBarModel {
    pub input: String,
    pub cursor_position: usize,
}

pub enum CursorDirection {
    Left,
    Right,
}

pub enum CommandBarModelActions {
    Input(char),
    MoveCursor(CursorDirection),
    Backspace,
    Enter,
    Reset,
}

impl CommandBarModel {
    pub fn update(&mut self, action: CommandBarModelActions) -> Option<AppStateActions> {
        match action {
            CommandBarModelActions::Input(data) => {
                self.input.insert(self.cursor_position, data);
                self.cursor_position += 1;
                None
            }
            CommandBarModelActions::MoveCursor(direction) => match direction {
                CursorDirection::Left => {
                    if self.cursor_position > 1 {
                        self.cursor_position -= 1;
                    }
                    None
                }
                CursorDirection::Right => {
                    if self.cursor_position < self.input.len() {
                        self.cursor_position += 1;
                    }
                    None
                }
            },
            CommandBarModelActions::Backspace => {
                let is_not_leftmost = self.cursor_position != 1;

                if is_not_leftmost {
                    let current_index = self.cursor_position;
                    let from_left_to_current_index = current_index - 1;

                    let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
                    let after_chars_to_delete = self.input.chars().skip(current_index);

                    self.input = before_char_to_delete.chain(after_chars_to_delete).collect();
                    self.cursor_position -= 1;
                }
                None
            }
            CommandBarModelActions::Enter => {
                let input = self.get_input();
                let command = input.to_string().split_off(1);

                self.handle_input(&command)
            }
            CommandBarModelActions::Reset => {
                self.input = ":".into();
                self.cursor_position = 1;
                Some(AppStateActions::AppModelActions(AppModelActions::ChangeMode(AppMode::Normal)))
            }
        }
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn handle_input(&self, command: &str) -> Option<AppStateActions> {
        print!("{}", command);
        match command {
            "q" | "exit" | "quit" => Some(AppStateActions::AppModelActions(AppModelActions::Exit)),
            "save" => None,
            _ => None,
        }
    }

    pub fn new() -> Self {
        CommandBarModel::default()
    }
}

impl Default for CommandBarModel {
    fn default() -> Self {
        CommandBarModel {
            input: ":".to_string(),
            cursor_position: 1,
        }
    }
}
