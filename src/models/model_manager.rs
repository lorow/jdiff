use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use super::app_state::{AppState, AppStateActions};

#[derive(Debug)]
pub struct ModelManager {
    app_state: Arc<Mutex<AppState>>,
    sender: mpsc::Sender<AppStateActions>,
}

impl ModelManager {
    pub fn new(app_state: AppState) -> Self {
        let (sender, receiver) = mpsc::channel();
        let app_state = Arc::new(Mutex::new(app_state));

        let app_state_clone = Arc::clone(&app_state);
        thread::spawn(move || loop {
            if let Ok(command) = receiver.recv() {
                let mut app_state = app_state_clone.lock().unwrap();
                app_state.update(command);
            }
        });

        Self { app_state, sender }
    }

    pub fn update(&self, command: AppStateActions) {
        if let Err(_) = self.sender.send(command) {
            // todo handle this better
            // we're probably exiting, it's fine
        };
    }

    pub fn get_state(&self) -> AppState {
        self.app_state.lock().unwrap().clone()
    }
}
