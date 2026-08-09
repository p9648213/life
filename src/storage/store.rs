use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use crate::storage::{collection::Colection, error::StoreError};

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

    pub fn collection<T>(&self, name: &str) -> Colection<T> {
        Colection::new(self.path.join(name))
    }

    pub fn create_collection(&self, name: &str) -> std::io::Result<()> {
        let collection_path = self.path.join(name);
        if !collection_path.is_file() {
            fs::File::create(collection_path)?;
        }
        Ok(())
    }
}
