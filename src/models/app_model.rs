use super::app_state::AppStateActions;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    Normal,
    Editing,
    Command,
}

pub enum AppModelActions {
    ChangeMode(AppMode),
    Exit,
}

#[derive(Debug, Default, Clone)]
pub struct AppModel {
    should_quit: bool,
    mode: AppMode,
}

impl AppModel {
    pub fn update(&mut self, action: AppModelActions) -> Option<AppStateActions> {
        match action {
            AppModelActions::ChangeMode(mode) => {
                self.mode = mode;
                None
            }
            AppModelActions::Exit => {
                self.should_quit = true;
                None
            }
        }
    }

    pub fn get_should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn get_app_mode(&self) -> AppMode {
        self.mode
    }
}
