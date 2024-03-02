use super::view::View;
use crate::{
    models::app_state::AppStateActions, store::dispatcher::Dispatcher, ui::router::Navigate,
};
use crossterm::event::KeyCode::Char;
use ratatui::{
    prelude::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct WelcomeVIew {
    app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>,
}

impl WelcomeVIew {
    pub fn new(app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>) -> Self {
        WelcomeVIew {
            app_state_dispatcher,
        }
    }
}

impl View for WelcomeVIew {
    fn render(&self, frame: &mut ratatui::Frame, rect: Rect) {
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
        route_dispatcher: &mut crate::store::dispatcher::Dispatcher<crate::ui::router::Navigate>,
    ) {
        let mut app_state_dispatcher = self.app_state_dispatcher.lock().unwrap();

        match key_event.code {
            Char('n') => route_dispatcher.dispatch(Navigate::Path("/counter".into())),
            Char('q') => app_state_dispatcher.dispatch(AppStateActions::Exit),
            _ => {}
        }
    }
}
