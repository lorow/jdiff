use crossterm::event::KeyEvent;
use ratatui::{prelude::Rect, Frame};

use crate::models::app_state::{AppState, AppStateActions};

pub enum TabState {
    None,
    Tab,
    BackTab,
}

pub struct ViewContext {
    pub is_ctrl_pressed: bool,
    pub is_shift_pressed: bool,
    pub tab_state: TabState,
}

impl ViewContext {
    pub fn new(is_ctrl_pressed: bool, is_shift_pressed: bool, tab_state: TabState) -> ViewContext {
        ViewContext {
            is_ctrl_pressed,
            is_shift_pressed,
            tab_state,
        }
    }
}

pub trait View {
    fn get_has_been_initialized(&self, app_state: &AppState) -> bool;
    fn get_has_been_resized(&self, app_state: &AppState) -> bool;

    fn init(
        &mut self,
        rame: &mut Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions>;

    fn render(&self, frame: &mut Frame, rect: Rect, app_state: &AppState);

    fn handle_event(
        &mut self,
        key_event: &KeyEvent,
        context: ViewContext,
        app_state: &AppState,
    ) -> Option<AppStateActions>;

    fn handle_resize(
        &mut self,
        frame: &mut Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions>;
}
