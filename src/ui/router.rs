use std::collections::HashMap;

use crossterm::event::KeyEvent;
use ratatui::Frame;

use super::views::view::View;

pub struct Router {
    registered_views: HashMap<String, Box<dyn View>>,
    current_view: String,
}

impl Router {
    pub fn new(initial_view: Box<dyn View>) -> Self {
        Router {
            registered_views: HashMap::from([("/".into(), initial_view)]),
            current_view: "/".into(),
        }
    }

    pub fn register_view(&mut self, path: String, view: Box<dyn View>) -> Result<(), String> {
        if self.registered_views.contains_key(&path) {
            return Err("Key already exists".into());
        }
        self.registered_views.insert(path, view);
        Ok(())
    }

    pub fn route(&mut self, path: String) -> Result<(), String> {
        if !self.registered_views.contains_key(&path) {
            return Err("Seelected path does not exists".into());
        }
        self.current_view = path;
        Ok(())
    }

    pub fn handle_event(&mut self, event: &KeyEvent) {
        let current_view = self.current_view.clone();
        self.registered_views
            .get(&current_view)
            .unwrap()
            .handle_event(self, event)
        //|path| self.route(path)
    }

    pub fn render_view(&self, frame: &mut Frame) {
        // self.current_view.render(frame)
    }
}
