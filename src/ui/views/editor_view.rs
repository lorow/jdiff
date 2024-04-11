// so how I want this view to work is simple. This view will hold a list of editors, and a
// reference to a list of stores for each editor. I also want to be able to add new editors
// and delete existing editors. I also want to be able to switch between editors. I also want
// to be able to save the current editor to a file as a workspace.
//
// now, when someone pastes and there's more than one editor, I want to show a popup and let them
// select to which editor to write to
//
// now, when someone pastes and there's only one editor, I want to just write to that editor
//
// by default, there's only one editor.

use crate::models::app_state::{AppState, AppStateActions};

use super::view::View;

pub struct EditorView {}

impl View for EditorView {
    fn render(
        &self,
        frame: &mut ratatui::Frame,
        layouti: ratatui::prelude::Rect,
        app_state: &AppState,
    ) {
        todo!()
    }

    fn handle_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
        is_ctrl_pressed: bool,
        is_shift_pressed: bool,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        None
    }
}
