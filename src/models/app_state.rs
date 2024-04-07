use super::{
    app_model::{AppModel, AppModelActions},
    command_bar::{CommandBarModel, CommandBarModelActions},
    counter::{CounterModel, CounterModelActions},
    router::{RouterModel, RouterModelActions},
};

pub enum AppStateActions {
    AppModelActions(AppModelActions),
    CounterModelActions(CounterModelActions),
    CommandBarActions(CommandBarModelActions),
    RouterModelActions(RouterModelActions),
}

#[derive(Debug, Default)]
pub struct AppState {
    pub app_state_store: AppModel,
    pub counter_store: CounterModel,
    pub command_bar_store: CommandBarModel,
    pub router_store: RouterModel,
}

impl AppState {
    pub fn update(&mut self, message: Option<AppStateActions>) {
        let mut action_to_resolve = message;

        while let Some(action) = action_to_resolve {
            match action {
                AppStateActions::AppModelActions(model_action) => {
                    action_to_resolve = self.app_state_store.update(model_action)
                }
                AppStateActions::CounterModelActions(model_action) => {
                    action_to_resolve = self.counter_store.update(model_action)
                }
                AppStateActions::CommandBarActions(model_action) => {
                    action_to_resolve = self.command_bar_store.update(model_action)
                }
                AppStateActions::RouterModelActions(model_action) => {
                    action_to_resolve = self.router_store.update(model_action)
                }
            }
        }
    }

    pub fn new() -> Self {
        AppState::default()
    }
}
