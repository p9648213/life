use std::{env, path::PathBuf};

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
        Ok(Self { path: storage_dir })
    }

    pub fn list(&self) {}
}
