use super::{
    app_model::{AppMode, AppModel, AppModelActions},
    command_bar::{CommandBarModel, CommandBarModelActions},
    editor::editor_container_models::{EditorContainerModel, EditorContainerModelActions},
    router::{RouterModel, RouterModelActions},
};

pub enum BaseActions {
    Resized,
}

pub enum AppStateActions {
    BaseAppActions(BaseActions),
    AppModelActions(AppModelActions),
    CommandBarActions(CommandBarModelActions),
    RouterModelActions(RouterModelActions),
    EditorActions(EditorContainerModelActions),
}

#[derive(Debug, Default)]
pub struct AppState {
    pub app_state_store: AppModel,
    pub command_bar_store: CommandBarModel,
    pub router_store: RouterModel,
    pub editor_store: EditorContainerModel,
}

impl AppState {
    pub fn update(&mut self, message: Option<AppStateActions>) {
        let mut action_to_resolve = message;

        while let Some(action) = action_to_resolve {
            match action {
                AppStateActions::BaseAppActions(base_action) => match base_action {
                    BaseActions::Resized => {
                        action_to_resolve = None;
                        self.editor_store
                            .update(EditorContainerModelActions::ToggleResize);
                    }
                },
                AppStateActions::AppModelActions(model_action) => {
                    action_to_resolve = self.app_state_store.update(model_action)
                }
                AppStateActions::CommandBarActions(model_action) => {
                    let should_reset_mode = match model_action {
                        CommandBarModelActions::Enter => true,
                        _ => false,
                    };

                    action_to_resolve = self.command_bar_store.update(model_action);

                    if should_reset_mode {
                        self.app_state_store
                            .update(AppModelActions::ChangeMode(AppMode::Normal));
                    }
                }
                AppStateActions::RouterModelActions(model_action) => {
                    action_to_resolve = self.router_store.update(model_action)
                }
                AppStateActions::EditorActions(model_action) => {
                    action_to_resolve = self.editor_store.update(model_action)
                }
            }
        }
    }

    pub fn new() -> Self {
        AppState::default()
    }
}
