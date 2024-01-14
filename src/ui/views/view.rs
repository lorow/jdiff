use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::{store::dispatcher::Dispatcher, ui::router::Navigate};

pub trait View {
    fn render(&self, frame: &mut Frame);
    fn handle_event(&mut self, key_event: &KeyEvent, route_dispatcher: &mut Dispatcher<Navigate>);
}
