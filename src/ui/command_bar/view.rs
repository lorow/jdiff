use crossterm::event::KeyCode::Char;

use crate::{
    models::{
        app_model::AppMode,
        app_state::{AppState, AppStateActions},
        command_bar::{CommandBarModelActions, CursorDirection},
    },
    ui::views::view::View,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Frame, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::Paragraph,
};

#[derive(Default)]
pub struct CommandBar {}

impl CommandBar {
    pub fn new() -> Self {
        Default::default()
    }
}

impl View for CommandBar {
    fn init(
        &mut self,
        rame: &mut Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        None
    }

    fn render(&self, frame: &mut Frame, rect: Rect, app_state: &AppState) {
        let app_state_mode = app_state.app_state_store.get_app_mode();
        if app_state_mode == AppMode::Command {
            let (input, cursor_position) = {
                (
                    app_state.command_bar_store.get_input(),
                    app_state.command_bar_store.get_cursor_position(),
                )
            };

            let text = Text::from(Line::from(input));
            frame.render_widget(Paragraph::new(text), rect);
            frame.set_cursor(rect.x + cursor_position as u16, rect.y + 1)
        } else {
            let status_bar_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Max(10),
                    Constraint::Percentage(90),
                    Constraint::Max(15),
                ])
                .split(rect);

            let (mode_text, mode_color) = match app_state_mode {
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

            let lines_widget = Text::from(Line::from(" 10:80 "));
            let lines_widget = Paragraph::new(lines_widget)
                .alignment(Alignment::Center)
                .style(Style::default().bg(Color::Blue));
            frame.render_widget(lines_widget, status_bar_layout[2]);
        }
    }

    fn handle_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
        is_ctrl_pressed: bool,
        is_shift_pressed: bool,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        match key_event.code {
            Char(data) => Some(AppStateActions::CommandBarActions(
                CommandBarModelActions::Input(data),
            )),
            crossterm::event::KeyCode::Backspace => Some(AppStateActions::CommandBarActions(
                CommandBarModelActions::Backspace,
            )),
            crossterm::event::KeyCode::Enter => Some(AppStateActions::CommandBarActions(
                CommandBarModelActions::Enter,
            )),
            crossterm::event::KeyCode::Left => Some(AppStateActions::CommandBarActions(
                CommandBarModelActions::MoveCursor(CursorDirection::Left),
            )),
            crossterm::event::KeyCode::Right => Some(AppStateActions::CommandBarActions(
                CommandBarModelActions::MoveCursor(CursorDirection::Right),
            )),
            crossterm::event::KeyCode::Esc => Some(AppStateActions::CommandBarActions(
                CommandBarModelActions::Reset,
            )),
            _ => None,
        }
    }

    fn handle_resize(
        &mut self,
        frame: &mut Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        None
    }

    fn get_has_been_initialized(&self, app_state: &AppState) -> bool {
        true
    }

    fn get_has_been_resized(&self, app_state: &AppState) -> bool {
        false
    }
}
