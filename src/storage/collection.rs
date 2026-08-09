use std::{fs::OpenOptions, io::Write, marker::PhantomData, path::PathBuf};

use crate::storage::{codec::Encode, error::StoreError};

pub struct Colection<T> {
    path: PathBuf,
    _collection_type: PhantomData<T>,
}

impl<T> Colection<T> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            _collection_type: PhantomData,
        }
    }

    pub fn insert_one(&self, item: T) -> Result<(), StoreError>
    where
        T: Encode,
    {
        let mut file = OpenOptions::new().create(true).append(true).open(&self.path).unwrap();
        let encode = item.encode();
        file.write_all(&encode).unwrap();
        Ok(())
    }
}
