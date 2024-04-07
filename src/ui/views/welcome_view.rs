use super::view::View;
use crate::{
    models::{
        app_model::AppModelActions,
        app_state::{AppState, AppStateActions},
    },
    store::dispatcher::Dispatcher,
    ui::router::Navigate,
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
        route_dispatcher: &mut Dispatcher<Navigate>,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        match key_event.code {
            Char('n') => {
                route_dispatcher.dispatch(Navigate::Path("/counter".into()));
                None
            }
            Char('q') => {
                Some(AppStateActions::AppModelActions(AppModelActions::Exit))
            }
            _ => None,
        }
    }
}
