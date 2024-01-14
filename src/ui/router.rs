use crate::store::dispatcher::Store;


pub enum Navigate { 
    Path(String)
}

pub struct Router {
    routes: Vec<String>,
    pub current_route: String,
}

impl Default for Router{
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Router{
            routes: Vec::new(),
            current_route: "/".into(),
        }
    }

    pub fn register_routes(&mut self, routes: Vec<String>){
        self.routes = routes;
    } 
}

impl Store for Router {
    type Action = Navigate;

    fn handle(&mut self, action: &Self::Action) {
      match action {
        Navigate::Path(path) => {
                if self.routes.contains(path){
                    self.current_route = path.clone();
                }
            },
        }
    }
}

