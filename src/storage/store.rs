use std::{
    env,
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
};

use crate::storage::error::StoreError;

#[derive(Debug)]
pub struct Store {
    path: PathBuf,
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
        Ok(Self { path: storage_dir })
    }

    pub fn list<T>(&self, collection_name: &str) -> Result<Vec<T>, StoreError> {
        let collection_path = self.path.join(collection_name);
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
        println!("Resouces: {raw}");
        Ok(Vec::new())
    }

    pub fn insert_one<T>(&self, collection_name: &str, item: T) -> Result<bool, StoreError> {
        Ok(true)
    }
}
