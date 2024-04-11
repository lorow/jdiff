use super::view::View;
use crate::models::{
    app_model::AppModelActions,
    app_state::{AppState, AppStateActions},
    router::RouterModelActions,
};
use crossterm::event::KeyCode::Char;
use ratatui::{
    prelude::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};

#[derive(Default)]
pub struct WelcomeVIew {}

impl WelcomeVIew {
    pub fn new() -> Self {
        WelcomeVIew {}
    }
}

impl View for WelcomeVIew {
    fn render(&self, frame: &mut ratatui::Frame, rect: Rect, app_state: &AppState) {
        frame.render_widget(
            Paragraph::new(
                "
                Welcome! \n\
                N) - Create new workspace \n\
                O) - Open saved worksapces
                Q) - Exit
                "
                .to_string(),
            )
            .block(
                Block::new()
                    .title("Welcome to JDiff")
                    .title_alignment(ratatui::prelude::Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center),
            rect,
        )
    }

    fn handle_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
        is_ctrl_pressed: bool,
        is_shift_pressed: bool,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        match key_event.code {
            Char('n') => {
                return Some(AppStateActions::RouterModelActions(
                    RouterModelActions::Route("/counter".into()),
                ));
            }
            Char('q') => Some(AppStateActions::AppModelActions(AppModelActions::Exit)),
            _ => None,
        }
    }
}
