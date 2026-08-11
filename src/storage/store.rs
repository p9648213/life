use std::{
    env,
    fs::{self},
    io::Write,
    path::PathBuf,
};

use crate::{
    constant::{STORAGE_MAGIC, STORAGE_NEXT_ID, STORAGE_RECORD_COUNT, STORAGE_VERSION},
    storage::{collection::Colection, error::StoreError},
};

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
            fs::create_dir(&storage_dir)
                .map_err(|err| StoreError::ConnectionError(err.to_string()))?;
        }
        Ok(Self { path: storage_dir })
    }

    pub fn collection<T>(&self, name: &str) -> Colection<T> {
        Colection::new(self.path.join(name))
    }

    pub fn create_collection(&self, name: &str) -> std::io::Result<()> {
        let collection_path = self.path.join(name);
        if !collection_path.is_file() {
            let mut file = fs::File::create(collection_path)?;
            file.write_all(STORAGE_MAGIC.as_bytes())?;
            file.write_all(&[STORAGE_VERSION])?;
            file.write_all(&STORAGE_NEXT_ID.to_be_bytes())?;
            file.write_all(&STORAGE_RECORD_COUNT.to_be_bytes())?;
        }
        Ok(())
    }
}
