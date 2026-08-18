use std::{
    env,
    fs::{self},
    io::Write,
    path::PathBuf,
};

use crate::{
    constant::{
        COLLECTION_EXTENSION, INDEX_EXTENSION, INDEX_MAGIC, INDEX_RECORD_COUNT, INDEX_VERSION,
        STORAGE_MAGIC, STORAGE_NEXT_ID, STORAGE_RECORD_COUNT, STORAGE_VERSION,
    },
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
        Colection::new(
            self.path.join(format!("{}.{}", name, COLLECTION_EXTENSION)),
            self.path.join(format!("{}.{}", name, INDEX_EXTENSION)),
        )
    }

    pub fn create_collection(&self, name: &str) -> std::io::Result<()> {
        let collection_path = self.path.join(format!("{}.{}", name, COLLECTION_EXTENSION));
        if !collection_path.is_file() {
            let mut file = fs::File::create(collection_path)?;
            file.write_all(STORAGE_MAGIC.as_bytes())?;
            file.write_all(&[STORAGE_VERSION])?;
            file.write_all(&STORAGE_NEXT_ID.to_be_bytes())?;
            file.write_all(&STORAGE_RECORD_COUNT.to_be_bytes())?;
        }
        let index_path = self.path.join(format!("{}.{}", name, INDEX_EXTENSION));
        if !index_path.is_file() {
            let mut file = fs::File::create(index_path)?;
            file.write_all(INDEX_MAGIC.as_bytes())?;
            file.write_all(&[INDEX_VERSION])?;
            file.write_all(&INDEX_RECORD_COUNT.to_be_bytes())?;
        }
        Ok(())
    }
}
