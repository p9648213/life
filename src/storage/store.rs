use std::{
    env,
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
};

use crate::storage::error::StoreError;

#[derive(Debug)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn connect(path: &str) -> Result<Self, StoreError> {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR")
                .map_err(|err| StoreError::ConnectionError(err.to_string()))?,
        );
        let storage_dir = manifest_dir.join(path);
        if !storage_dir.exists() || !storage_dir.is_dir() {
            return Err(StoreError::ConnectionError(format!(
                "Storage path does not exist: {}",
                storage_dir.display()
            )));
        }
        Ok(Self { root: storage_dir })
    }

    pub fn open_collection(&self, collection_name: &str) -> Result<String, StoreError> {
         let collection_path = self.root.join(collection_name);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(collection_path)
            .map_err(|err| StoreError::ConnectionError(err.to_string()))?;
        let mut raw = String::new();
        file.read_to_string(&mut raw)
            .map_err(|err| StoreError::ReadError(err.to_string()))?;
        Ok(raw)
    }

    pub fn list<T>(&self, collection_name: &str) -> Result<Vec<T>, StoreError> {
        let raw = self.open_collection(collection_name)?;
        println!("Resouces: {raw}");
        Ok(Vec::new())
    }

    pub fn insert_one<T>(&self, collection_name: &str, item: T) -> Result<bool, StoreError> {
        let raw = self.open_collection(collection_name)?;
        Ok(true)
    }
}
