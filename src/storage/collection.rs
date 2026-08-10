use std::{
    fs,
    io::{self, BufReader, Read, Write},
    marker::PhantomData,
    path::PathBuf,
};

use crate::storage::{
    decode::{Decode, Decoder},
    encode::Encode,
    error::StoreError,
};

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
        let mut file = fs::OpenOptions::new()
            .create(false)
            .append(true)
            .open(&self.path)
            .unwrap();
        let mut bytes = vec![];
        let payload = item.encode()?;
        let payload_size = u32::try_from(payload.len())
            .map_err(|err| StoreError::FieldTooLarge(err.to_string()))?;
        bytes.extend_from_slice(&payload_size.to_be_bytes());
        bytes.extend_from_slice(&payload);
        file.write_all(&bytes).unwrap();
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<T>, StoreError>
    where
        T: Decode,
    {
        let f = fs::File::open(&self.path).unwrap();
        let mut reader = BufReader::new(f);
        let mut items = vec![];
        loop {
            let mut len_buf = [0u8; 4];
            match reader.read_exact(&mut len_buf) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(StoreError::ReadError(err.to_string())),
            };
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut payload = vec![0u8; len];
            reader
                .read_exact(&mut payload)
                .map_err(|err| StoreError::ReadError(err.to_string()))?;
            let mut decoder = Decoder::new(&payload);
            let item = T::decode(&mut decoder)?;
            items.push(item);
        }
        Ok(items)
    }
}
