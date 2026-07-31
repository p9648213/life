use std::{env, fs, path::PathBuf};

use crate::storage::error::StoreError;

#[derive(Debug)]
pub struct Store<'store> {
    path: &'store str,
}

impl<'store> Store<'store> {
    pub fn connect(path: &'store str) -> Result<Self, StoreError> {
        let manifest_dir = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR")
                .map_err(|err| StoreError::ConnectionError(err.to_string()))?,
        );
        let storage_dir = manifest_dir.join(path);
        let _entries = fs::read_dir(&storage_dir)
            .map_err(|err| StoreError::ConnectionError(err.to_string()))?;
        Ok(Self { path })
    }
}
