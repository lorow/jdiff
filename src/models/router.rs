use crate::models::app_state::AppStateActions;

pub enum RouterModelActions {
    Route(String),
}

#[derive(Debug, Clone)]
pub struct RouterModel {
    routes: Vec<String>,
    current_route: String,
}

impl RouterModel {
    pub fn update(&mut self, action: RouterModelActions) -> Option<AppStateActions> {
        match action {
            RouterModelActions::Route(path) => {
                if self.routes.contains(&path) {
                    self.current_route = path.clone();
                }
            }
        }

        None
    }

    pub fn register_routes(&mut self, routes: Vec<String>) {
        self.routes = routes;
    }

    pub fn get_current_route(&self) -> String {
        self.current_route.clone()
    }
}

impl Default for RouterModel {
    fn default() -> Self {
        Self {
            routes: Default::default(),
            current_route: "/".into(),
        }
    }
}
