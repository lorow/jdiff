use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{models::model::ModelActions, store::dispatcher::Dispatcher};

pub fn update(dispatcher: &mut Dispatcher<ModelActions>, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => dispatcher.dispatch(ModelActions::Exit),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {}
        }
        KeyCode::Right | KeyCode::Char('j') => dispatcher.dispatch(ModelActions::Increment),
        KeyCode::Left | KeyCode::Char('k') => dispatcher.dispatch(ModelActions::Decrement),
        _ => {}
    };
}
