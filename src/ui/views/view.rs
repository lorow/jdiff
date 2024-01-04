use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::ui::router::Router;

pub trait ViewEventHandler {
    fn handle_event(&mut self, router: &mut Router, key_event: &KeyEvent);
}

pub trait View: ViewEventHandler {
    fn render(&self, frame: &mut Frame);
    // fn handle_event<F>(&self, route_to: F, key_event: &KeyEvent)
    // where
    //     F: FnOnce(String);
}
