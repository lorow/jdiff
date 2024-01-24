use crossterm::event::KeyEvent;
use ratatui::{Frame, prelude::{Layout, Rect}};

use crate::{store::dispatcher::Dispatcher, ui::router::Navigate};

pub trait View {
    fn render(&self, frame: &mut Frame, layouti: Rect);
    fn handle_event(&mut self, key_event: &KeyEvent, route_dispatcher: &mut Dispatcher<Navigate>);
}
