use std::{
    fs,
    io::{self, BufReader, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    path::PathBuf,
};

use crate::{
    constant::{
        STORAGE_HEADER_TOTAL_BYTES, STORAGE_MAGIC, STORAGE_MAGIC_END, STORAGE_NEXT_ID_OFFSET,
        STORAGE_RECORD_COUNT_OFFSET, STORAGE_VERSION, STORAGE_VERSION_OFFSET,
    },
    storage::{
        decode::{Decode, Decoder},
        encode::Encode,
        error::StoreError,
    },
};

pub struct Colection<T> {
    path: PathBuf,
    record_count: Option<u32>,
    _collection_type: PhantomData<T>,
}

impl<T> Colection<T> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            record_count: None,
            _collection_type: PhantomData,
        }
    }

    pub fn insert_one(&self, item: T) -> Result<(), StoreError>
    where
        T: Encode,
    {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)?;
        f.seek(SeekFrom::Start(STORAGE_NEXT_ID_OFFSET as u64))?;
        let mut next_id_buf = [0u8; 4];
        f.read_exact(&mut next_id_buf)?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let next_id = u32::from_be_bytes(next_id_buf);
        let record_count = u32::from_be_bytes(record_count_buf);
        let mut bytes = vec![];
        let payload = item.encode(next_id)?;
        let payload_size = u32::try_from(payload.len())?;
        f.seek(SeekFrom::Start(STORAGE_RECORD_COUNT_OFFSET as u64))?;
        f.write_all(&(record_count + 1).to_be_bytes())?;
        f.seek(SeekFrom::Start(STORAGE_NEXT_ID_OFFSET as u64))?;
        f.write_all(&(next_id + 1).to_be_bytes())?;
        bytes.extend_from_slice(&payload_size.to_be_bytes());
        bytes.extend_from_slice(&payload);
        f.seek(SeekFrom::End(0))?;
        f.write_all(&bytes)?;
        Ok(())
    }

    pub fn list(&mut self) -> Result<Vec<T>, StoreError>
    where
        T: Decode,
    {
        let f = fs::OpenOptions::new().read(true).open(&self.path)?;
        let mut reader = BufReader::new(f);
        let mut header_buf = [0u8; STORAGE_HEADER_TOTAL_BYTES];
        reader.read_exact(&mut header_buf)?;
        let magic_bytes = str::from_utf8(&header_buf[..STORAGE_MAGIC_END])?;
        let version = header_buf[STORAGE_VERSION_OFFSET];
        let record_count = u32::from_be_bytes(
            header_buf[STORAGE_RECORD_COUNT_OFFSET..STORAGE_HEADER_TOTAL_BYTES].try_into()?,
        );
        self.record_count = Some(record_count);
        if magic_bytes != STORAGE_MAGIC {
            return Err(StoreError::InvalidStorageFormat);
        }
        if version != STORAGE_VERSION {
            return Err(StoreError::UnsupportVersion);
        }
        let mut items = vec![];
        loop {
            let mut len_buf = [0u8; 4];
            match reader.read_exact(&mut len_buf) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(StoreError::IoError(err)),
            };
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut payload = vec![0u8; len];
            reader.read_exact(&mut payload)?;
            let mut decoder = Decoder::new(&payload);
            let item = T::decode(&mut decoder)?;
            items.push(item);
        }
        Ok(items)
    }

    pub fn record_count(&self) -> Result<u32, StoreError> {
        if let Some(record_count) = self.record_count {
            Ok(record_count)
        } else {
            let f = fs::OpenOptions::new().read(true).open(&self.path)?;
            let mut reader = BufReader::new(f);
            let mut header_buf = [0u8; STORAGE_HEADER_TOTAL_BYTES];
            reader.read_exact(&mut header_buf)?;
            let record_count = u32::from_be_bytes(
                header_buf[STORAGE_RECORD_COUNT_OFFSET..STORAGE_HEADER_TOTAL_BYTES].try_into()?,
            );
            Ok(record_count)
        }
    }
}
