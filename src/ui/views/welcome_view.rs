use super::view::{View, ViewContext};
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
    fn init(
        &mut self,
        rame: &mut ratatui::prelude::Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        None
    }

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
        _context: ViewContext,
        _app_state: &AppState,
    ) -> Option<AppStateActions> {
        match key_event.code {
            Char('n') => Some(AppStateActions::RouterModelActions(
                RouterModelActions::Route("/editor".into()),
            )),
            Char('q') => Some(AppStateActions::AppModelActions(AppModelActions::Exit)),
            _ => None,
        }
    }

    fn get_has_been_initialized(&self, app_state: &AppState) -> bool {
        true
    }

    fn get_has_been_resized(&self, app_state: &AppState) -> bool {
        false
    }

    fn handle_resize(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        None
    }
}
