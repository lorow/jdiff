use ratatui::{widgets::{Block, BorderType, Borders, Paragraph}, prelude::{Rect, Alignment}};
use crossterm::event::KeyCode::Char;
use crate::ui::router::Navigate;

use super::view::View;

#[derive(Default)]
pub struct WelcomeVIew {}

impl WelcomeVIew {
    pub fn new() -> Self {
        WelcomeVIew {}
    }
}

impl View for WelcomeVIew {
    fn render(&self, frame: &mut ratatui::Frame, rect: Rect) {
        frame.render_widget(
            Paragraph::new(format!(
                "
          Welcome! \n\
          N) - Create new workspace \n\
          O) - Open saved worksapces 
          Q) - Exit
        "
            ))
            .block(
                Block::new()
                    .title("Welcome to JDiff")
                    .title_alignment(ratatui::prelude::Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ).alignment(Alignment::Center),
            frame.size(),
        )
    }

    fn handle_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
        route_dispatcher: &mut crate::store::dispatcher::Dispatcher<crate::ui::router::Navigate>,
    ) {
        match key_event.code {
            Char('n') => route_dispatcher.dispatch(Navigate::Path("/counter".into())),
            _ => {},
        }
    }
}
