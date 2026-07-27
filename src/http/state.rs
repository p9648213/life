use std::collections::HashMap;

type Id = u32;

pub struct State<T> {
    current_id: Id,
    state_map: HashMap<Id, T>
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> State<T> {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            state_map: HashMap::new()
        }
    }

    pub fn insert(&mut self, state: T) {
        self.current_id += 1;
        self.state_map.insert(self.current_id, state);
    }
}

