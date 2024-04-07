use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, Frame};

use crate::{
    models::app_state::{AppState, AppStateActions},
    store::dispatcher::Dispatcher,
    ui::router::Navigate,
};

pub trait View {
    fn render(&self, frame: &mut Frame, rect: Rect, app_state: &AppState);
    fn handle_event(
        &mut self,
        key_event: &KeyEvent,
        route_dispatcher: &mut Dispatcher<Navigate>,
        app_state: &AppState,
    ) -> Option<AppStateActions>;
}
