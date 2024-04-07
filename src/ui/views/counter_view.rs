use crossterm::event::{KeyCode::Char, KeyEventKind};
use ratatui::{
    prelude::{Alignment, Frame, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::models::{
    app_model::AppMode,
    app_state::{AppState, AppStateActions},
    counter::CounterModelActions,
    router::RouterModelActions,
};

use super::view::View;

pub struct CounterView {}

impl CounterView {
    pub fn new() -> Self {
        CounterView {}
    }
}

impl View for CounterView {
    fn render(&self, frame: &mut Frame, rect: Rect, app_state: &AppState) {
        frame.render_widget(
            Paragraph::new(format!(
                "
            Press `;`, to go back to the dashboard.\n\
            Press `j` and `k` to increment and decrement the counter respectively.\n\
            Counter: {}
          ",
                app_state.counter_store.get_counter()
            ))
            .block(
                Block::default()
                    .title("Counter App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
            rect,
        )
    }

    fn handle_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        if key_event.kind == KeyEventKind::Press {
            let app_state = app_state.app_state_store.get_app_mode();

            let event: Option<AppStateActions> = match key_event.code {
                Char('j') => Some(AppStateActions::CounterModelActions(
                    CounterModelActions::Increment,
                )),
                Char('k') => Some(AppStateActions::CounterModelActions(
                    CounterModelActions::Decrement,
                )),
                Char(';') => {
                    if app_state == AppMode::Normal {
                        return Some(AppStateActions::RouterModelActions(
                            RouterModelActions::Route("/".into()),
                        ));
                    }
                    return None;
                }
                _ => None,
            };

            return event;
        }
        None
    }
}
