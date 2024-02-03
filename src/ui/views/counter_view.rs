use std::sync::{Arc, Mutex};

use crate::{
    models::{
        app_state::{AppMode, AppState, AppStateActions},
        counter::{CounterModel, CounterModelActions},
    },
    store::dispatcher::Dispatcher,
    ui::router::Navigate,
};

use super::view::View;
use crossterm::event::{KeyCode::Char, KeyEventKind};
use ratatui::{
    prelude::{Alignment, Frame, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct CounterView {
    app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>,
    dispatcher: Arc<Mutex<Dispatcher<CounterModelActions>>>,
}

impl CounterView {
    pub fn new(
        app_state_dispatcher: Arc<Mutex<Dispatcher<AppStateActions>>>,
        dispatcher: Arc<Mutex<Dispatcher<CounterModelActions>>>,
    ) -> Self {
        CounterView {
            app_state_dispatcher,
            dispatcher,
        }
    }
}

impl View for CounterView {
    fn render(&self, frame: &mut Frame, rect: Rect) {
        let dispatcher = self.dispatcher.lock().unwrap();
        let store = dispatcher.get_store::<CounterModel>().unwrap();

        frame.render_widget(
            Paragraph::new(format!(
                "
            Press `;`, to go back to the dashboard.\n\
            Press `j` and `k` to increment and decrement the counter respectively.\n\
            Counter: {}
          ",
                store.counter
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
        route_dispatcher: &mut Dispatcher<crate::ui::router::Navigate>,
    ) {
        if key_event.kind == KeyEventKind::Press {
            let app_state_dispatcher = self.app_state_dispatcher.lock().unwrap();
            let app_state = app_state_dispatcher.get_store::<AppState>().unwrap();

            let mut dispatcher = self.dispatcher.lock().unwrap();

            match key_event.code {
                Char('j') => dispatcher.dispatch(CounterModelActions::Increment),
                Char('k') => dispatcher.dispatch(CounterModelActions::Decrement),
                Char(';') => {
                    if app_state.mode == AppMode::Normal {
                        route_dispatcher.dispatch(Navigate::Path("/".into()))
                    }
                }
                _ => {}
            }
        }
    }
}
