use crate::storage::store::Store;

#[derive(Debug)]
pub struct State<'state> {
    pub store: Store<'state>,
}
