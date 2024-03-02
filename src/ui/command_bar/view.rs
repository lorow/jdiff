use std::sync::{Arc, Mutex};

use crossterm::event::KeyCode::Char;

use crate::{
    models::app_state::{AppMode, AppState, AppStateActions},
    store::dispatcher::Dispatcher,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Frame, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::Paragraph,
};

// this should also be a proper model tho, with actions adding and handling input and or cursor position
// and it's state
pub struct CommandBar {
    input: String,
    cursor_position: usize,
    app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>,
}

impl CommandBar {
    pub fn new(app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>) -> Self {
        CommandBar {
            cursor_position: 1,
            input: ":".into(),
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
            let status_bar_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Percentage(90),
                    Constraint::Min(15),
                ])
                .split(rect);

            let (mode_text, mode_color) = match app_state_store_mode {
                AppMode::Editing => ("Editing", Style::default().bg(Color::Green)),
                AppMode::Normal => ("Normal", Style::default().bg(Color::Blue)),
                _ => ("", Style::default()),
            };

            let mode_widget = Text::from(Line::from(mode_text));
            let mode_paragraph = Paragraph::new(mode_widget)
                .alignment(Alignment::Center)
                .style(mode_color);
            frame.render_widget(mode_paragraph, status_bar_layout[0]);

            let middle_paragraph = Paragraph::default().style(Style::default().bg(Color::Black));
            frame.render_widget(middle_paragraph, status_bar_layout[1]);

            let lines_widget = Text::from(Line::from("Lines 10:80"));
            let lines_widget = Paragraph::new(lines_widget)
                .alignment(Alignment::Center)
                .style(Style::default().bg(Color::Blue));

            frame.render_widget(lines_widget, status_bar_layout[2]);
        }
    }

    pub fn handle_event(&mut self, key_event: &crossterm::event::KeyEvent) {
        match key_event.code {
            Char(data) => {
                self.input.insert(self.cursor_position, data);
                self.cursor_position += 1;
            }
            crossterm::event::KeyCode::Backspace => {
                let is_not_leftmost = self.cursor_position != 1;

                if is_not_leftmost {
                    let current_index = self.cursor_position;
                    let from_left_to_current_index = current_index - 1;

                    let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
                    let after_chars_to_delete = self.input.chars().skip(current_index);

                    self.input = before_char_to_delete.chain(after_chars_to_delete).collect();
                    self.cursor_position -= 1;
                }
            }
            crossterm::event::KeyCode::Enter => self.handle_input(),
            crossterm::event::KeyCode::Left => {
                if self.cursor_position > 1 {
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

                self.reset_input();
            }
            _ => {}
        }
    }

    pub fn reset_input(&mut self) {
        self.input = ":".into();
        self.cursor_position = 1;
    }

    pub fn handle_input(&mut self) {
        let command = self.input.split_off(1);
        self.reset_input();

        match command.as_str() {
            "q" | "exit" | "quit" => self
                .app_state_dispatcher
                .lock()
                .unwrap()
                .dispatch(AppStateActions::Exit),
            "save" => {}
            _ => {}
        }
    }
}
