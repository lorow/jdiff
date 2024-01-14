use crate::store::dispatcher::Store;

pub enum ModelActions {
    Increment,
    Decrement,
    Exit,
}

#[derive(Debug, Default)]
pub struct Model {
    pub counter: i64,
    pub should_quit: bool,
}

impl Store for Model {
    type Action = ModelActions;

    fn handle(&mut self, action: &Self::Action) {
        match action {
            ModelActions::Increment => self.counter += 1,
            ModelActions::Decrement => self.counter -= 1,
            ModelActions::Exit => self.should_quit = true,
        }
    }
}

impl Model {
    pub fn new() -> Self {
        Model::default()
    }

    pub fn exit_model() -> Self {
        Model {
            should_quit: true,
            ..Default::default()
        }
    }
}
