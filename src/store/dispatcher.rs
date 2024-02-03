use std::{
    any::{Any, TypeId},
    collections::{HashMap, VecDeque},
};

pub trait Store: Any + Send + Sync {
    type Action;

    fn handle(&mut self, action: &Self::Action);
}

pub trait StoreContainer<Action> {
    fn begin_dispatch(&mut self);
    fn handle(&mut self, action: &Action);
    fn as_any(&self) -> &dyn Any;
}

struct ConcreteStoreContainer<S: Store> {
    done: bool,
    store: S,
}

impl<S: Store> StoreContainer<S::Action> for ConcreteStoreContainer<S> {
    fn begin_dispatch(&mut self) {
        self.done = false;
    }

    fn handle(&mut self, action: &S::Action) {
        if !self.done {
            self.store.handle(action);
            self.done = true;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<S: Store> ConcreteStoreContainer<S> {
    pub fn new(store: S) -> Self {
        ConcreteStoreContainer { done: false, store }
    }
}

pub struct Dispatcher<A> {
    registered_stores: HashMap<TypeId, Box<dyn StoreContainer<A>>>,
}

impl<A> Default for Dispatcher<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> Dispatcher<A> {
    pub fn new() -> Dispatcher<A> {
        Dispatcher {
            registered_stores: HashMap::new(),
        }
    }

    pub fn register_store<S: Store<Action = A>>(&mut self, store: S) {
        self.registered_stores.insert(
            TypeId::of::<S>(),
            Box::new(ConcreteStoreContainer::new(store)),
        );
    }

    pub fn get_store<S: Store<Action = A>>(&self) -> Option<&S> {
        self.registered_stores
            .get(&TypeId::of::<S>())
            .and_then(|entry| entry.as_any().downcast_ref::<ConcreteStoreContainer<S>>())
            .map(|container| &container.store)
    }

    pub fn dispatch(&mut self, action: A) {
        let mut deferred_queue = VecDeque::new();
        deferred_queue.push_back(action);

        while let Some(event) = deferred_queue.pop_front() {
            let pending: Vec<_> = self
                .registered_stores
                .iter_mut()
                .map(|(&id, store)| {
                    store.begin_dispatch();
                    id
                })
                .collect();

            for id in pending {
                if let Some(entry) = self.registered_stores.get_mut(&id) {
                    entry.handle(&event);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Dispatcher, Store};

    enum MarbleStoreActions {
        Increment,
        Decrement,
        Add(i64),
    }

    struct MarbleStore {
        pub marbles: i64,
    }

    impl Store for MarbleStore {
        type Action = MarbleStoreActions;

        fn handle(&mut self, action: &Self::Action) {
            match action {
                MarbleStoreActions::Increment => self.marbles += 1,
                MarbleStoreActions::Decrement => self.marbles -= 1,
                MarbleStoreActions::Add(amount) => self.marbles += amount,
            }
        }
    }

    impl MarbleStore {
        pub fn new() -> Self {
            MarbleStore { marbles: 0 }
        }
    }

    #[test]
    fn test_registering_store() {
        let mut marble_dispatcher = Dispatcher::<MarbleStoreActions>::new();
        let marbles_store = MarbleStore::new();

        marble_dispatcher.register_store(marbles_store);
    }

    #[test]
    fn test_action_increment() {
        let mut marble_dispatcher = Dispatcher::<MarbleStoreActions>::new();
        marble_dispatcher.register_store(MarbleStore::new());
        marble_dispatcher.dispatch(MarbleStoreActions::Increment);
        let store = marble_dispatcher.get_store::<MarbleStore>().unwrap();

        assert_eq!(store.marbles, 1)
    }

    #[test]
    fn test_action_decrement() {
        let mut marble_dispatcher = Dispatcher::<MarbleStoreActions>::new();
        marble_dispatcher.register_store(MarbleStore::new());
        marble_dispatcher.dispatch(MarbleStoreActions::Decrement);
        let store = marble_dispatcher.get_store::<MarbleStore>().unwrap();

        assert_eq!(store.marbles, -1);
    }

    #[test]
    fn test_action_add() {
        let mut marble_dispatcher = Dispatcher::<MarbleStoreActions>::new();
        marble_dispatcher.register_store(MarbleStore::new());

        marble_dispatcher.dispatch(MarbleStoreActions::Add(2137));
        let store = marble_dispatcher.get_store::<MarbleStore>().unwrap();

        assert_eq!(store.marbles, 2137);
    }
}
