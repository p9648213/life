use std::{marker::PhantomData, path::PathBuf};

pub struct Collection<T> {
    path: PathBuf,
    _collection_type: PhantomData<T>
}
