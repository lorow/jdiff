use crate::store::dispatcher::Store;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Normal,
    Editing,
    Command,
}

pub enum AppStateActions {
    ChangeMode(AppMode),
    Exit,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub should_quit: bool,
    pub mode: AppMode,
}

impl Store for AppState {
    type Action = AppStateActions;

    fn handle(&mut self, action: &Self::Action) {
        match action {
            AppStateActions::ChangeMode(mode) => self.mode = *mode,
            AppStateActions::Exit => self.should_quit = true,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState::default()
    }

    pub fn exit_model() -> Self {
        AppState {
            should_quit: true,
            ..Default::default()
        }
    }
}
