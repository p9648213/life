use std::{env, path::PathBuf};

use crate::storage::error::StoreError;

#[derive(Debug)]
enum Command {
    Load,
}

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

    pub fn execute(&self, statement: &str) {
        let split = statement.split_ascii_whitespace();
        let mut command = None;
        let mut arguments = Vec::new();
        for part in split {
            match part {
                "LOAD" => {
                    if command.is_none() {
                        command = Some(Command::Load);
                    }
                }
                arg => {
                    if command.is_some() {
                        arguments.push(arg);
                    }
                }
            }
        }
        if let Some(command) = command {
            match command {
                Command::Load => Self::load(arguments),
            }
        }
    }

    fn load(arguments: Vec<&str>) {
        if arguments.len() == 1 {
            let file_name = arguments[0];
        }
    }
}
