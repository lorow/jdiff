use crate::store::dispatcher::Store;

pub enum CounterModelActions {
    Increment,
    Decrement,
}

#[derive(Debug, Default)]
pub struct CounterModel {
    pub counter: i64,
}

impl Store for CounterModel {
    type Action = CounterModelActions;

    fn handle(&mut self, action: &Self::Action) {
        match action {
            CounterModelActions::Increment => self.counter += 1,
            CounterModelActions::Decrement => self.counter -= 1,
        }
    }
}

impl CounterModel {
    pub fn new() -> Self {
        CounterModel::default()
    }
}
