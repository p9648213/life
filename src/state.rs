use std::collections::HashMap;

use crate::storage::store::Store;

#[derive(Debug)]
pub struct State {
    pub store: Store,
    pub session: HashMap<String, u32>,
}
