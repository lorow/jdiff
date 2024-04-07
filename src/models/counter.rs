use super::app_state::AppStateActions;

pub enum CounterModelActions {
    Increment,
    Decrement,
}

#[derive(Debug, Default)]
pub struct CounterModel {
    counter: i64,
}

impl CounterModel {
    pub fn update(&mut self, action: CounterModelActions) -> Option<AppStateActions> {
        match action {
            CounterModelActions::Increment => {
                self.counter += 1;
                None
            }
            CounterModelActions::Decrement => {
                self.counter -= 1;
                None
            }
        }
    }

    pub fn get_counter(&self) -> i64 {
        self.counter
    }
    pub fn new() -> Self {
        CounterModel::default()
    }
}
