use std::sync::{Arc, Mutex};

use crossterm::event::KeyCode::Char;

use crate::{
    models::app_state::{AppMode, AppState, AppStateActions},
    store::dispatcher::Dispatcher,
};

use ratatui::{
    prelude::{Frame, Rect},
    text::{Line, Text},
    widgets::Paragraph,
};

pub struct CommandBar {
    input: String,
    cursor_position: usize,
    app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>,
}

impl CommandBar {
    pub fn new(app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>) -> Self {
        CommandBar {
            cursor_position: 0,
            input: "".into(),
            app_state_dispatcher,
        }
    }

    pub fn render(&self, frame: &mut Frame, rect: Rect) {
        let app_state_store_mode = {
            let app_state_dispatcher = self.app_state_dispatcher.lock().unwrap();
            app_state_dispatcher.get_store::<AppState>().unwrap().mode
        };

        if app_state_store_mode == AppMode::Command {
            let text = Text::from(Line::from(self.input.clone()));
            frame.render_widget(Paragraph::new(text), rect);

            frame.set_cursor(rect.x + self.cursor_position as u16, rect.y + 1)
        } else {
        }
    }

    pub fn handle_event(&mut self, key_event: &crossterm::event::KeyEvent) {
        match key_event.code {
            Char(data) => {
                self.input.insert(self.cursor_position, data);
                self.cursor_position += 1;
            }
            crossterm::event::KeyCode::Backspace => {
                let is_not_leftmost = self.cursor_position != 0;

                if is_not_leftmost {
                    let current_index = self.cursor_position;
                    let from_left_to_current_index = current_index - 1;

                    let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
                    let after_chars_to_delete = self.input.chars().skip(current_index);

                    self.input = before_char_to_delete.chain(after_chars_to_delete).collect();
                    self.cursor_position -= 1;
                }
            }
            crossterm::event::KeyCode::Enter => todo!(),
            crossterm::event::KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            crossterm::event::KeyCode::Right => {
                if self.cursor_position < self.input.len() {
                    self.cursor_position += 1;
                }
            }
            crossterm::event::KeyCode::Esc => {
                self.app_state_dispatcher
                    .lock()
                    .unwrap()
                    .dispatch(AppStateActions::ChangeMode(AppMode::Normal));

                self.handle_esc();
            }
            _ => {}
        }
    }

    pub fn handle_esc(&mut self) {
        self.input = "".into();
        self.cursor_position = 0;
    }
}
