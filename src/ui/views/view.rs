use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, Frame};

use crate::models::app_state::{AppState, AppStateActions};

pub trait View {
    fn render(&self, frame: &mut Frame, rect: Rect, app_state: &AppState);
    fn handle_event(
        &mut self,
        key_event: &KeyEvent,
        is_ctrl_pressed: bool,
        is_shift_pressed: bool,
        app_state: &AppState,
    ) -> Option<AppStateActions>;
}
