use crate::{
    event::EventHandler,
    model::{Model, ModelActions},
    store::dispatcher::Dispatcher,
};

use super::view::{View, ViewEventHandler};
use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct CounterView<'a> {
    dispatcher: &'a Dispatcher<ModelActions>,
}

impl<'a> CounterView<'a> {
    pub fn new(dispatcher: &'a Dispatcher<ModelActions>) -> Self {
        CounterView {
            dispatcher: dispatcher,
        }
    }
}

impl<'a> View for CounterView<'a> {
    fn render(&self, frame: &mut Frame) {
        let store = self.dispatcher.get_store::<Model>().unwrap();

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
}

impl<'a> ViewEventHandler for CounterView<'a> {
    fn handle_event(
        &mut self,
        router: &mut crate::ui::router::Router,
        key_event: &crossterm::event::KeyEvent,
    ) {
        todo!()
    }
}
