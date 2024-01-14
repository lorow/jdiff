use std::sync::{Mutex, Arc};

use crate::{
    model::{Model, ModelActions},
    store::dispatcher::Dispatcher,
};

use super::view::View;
use crossterm::event::{KeyEventKind, KeyCode::Char};
use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct CounterView {
    dispatcher: Arc<Mutex<Dispatcher<ModelActions>>>,
}

impl CounterView {
    pub fn new(dispatcher: Arc<Mutex<Dispatcher<ModelActions>>>) -> Self {
        CounterView {
            dispatcher,
        }
    }
}

impl View for CounterView {
    fn render(&self, frame: &mut Frame) {
        let dispatcher = self.dispatcher.lock().unwrap();
        let store = dispatcher.get_store::<Model>().unwrap();

        frame.render_widget(
            Paragraph::new(format!(
                "
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
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
            frame.size(),
        )
    }

    fn handle_event(&mut self, key_event: &crossterm::event::KeyEvent, route_dispatcher: &mut Dispatcher<crate::ui::router::Navigate>) {
        if key_event.kind == KeyEventKind::Press {
            let mut dispatcher = self.dispatcher.lock().unwrap();
            match key_event.code {
                Char('j') => dispatcher.dispatch(ModelActions::Increment),       
                Char('k') => dispatcher.dispatch(ModelActions::Decrement),
                Char('q') => dispatcher.dispatch(ModelActions::Exit),
                _ => {}     
            }
        }
    }

}
