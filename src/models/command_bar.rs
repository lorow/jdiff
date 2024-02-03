use crate::store::dispatcher::Store;

pub struct CommandBarModel {
    input: String,
}

pub enum CommandBarModelActions {
    Input(String),
    Commit,
}

impl Store for CommandBarModel {
    type Action = CommandBarModelActions;
    fn handle(&mut self, action: &Self::Action) {
        // handle provided action as someone types
        match action {
            CommandBarModelActions::Input(data) => self.input += data,
            CommandBarModelActions::Commit => {
                todo!("Implement action handler")
            }
        }
    }
}
