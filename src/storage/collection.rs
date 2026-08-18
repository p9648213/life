use std::{
    fs,
    io::{self, BufReader, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    path::PathBuf,
};

use crate::{
    constant::{
        INDEX_HEADER_TOTAL_BYTES, INDEX_RECORD_COUNT_OFFSET, INDEX_RECORD_LEN,
        STORAGE_HEADER_TOTAL_BYTES, STORAGE_MAGIC, STORAGE_MAGIC_END, STORAGE_NEXT_ID_OFFSET,
        STORAGE_PAYLOAD_LEN_SIZE, STORAGE_RECORD_COUNT_OFFSET, STORAGE_VERSION,
        STORAGE_VERSION_OFFSET,
    },
    storage::{
        decode::{Decode, Decoder},
        encode::Encode,
        error::StoreError,
    },
};

pub struct Colection<T> {
    store_path: PathBuf,
    index_path: PathBuf,
    record_count: Option<u32>,
    _collection_type: PhantomData<T>,
}

impl<T> Colection<T> {
    pub fn new(store_path: PathBuf, index_path: PathBuf) -> Self {
        Self {
            store_path,
            index_path,
            record_count: None,
            _collection_type: PhantomData,
        }
    }

    fn insert_index(&self, id: u32, frame_offset: u64) -> Result<(), StoreError> {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.index_path)?;
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        f.write_all(&(record_count + 1).to_be_bytes())?;
        f.seek(SeekFrom::End(0))?;
        f.write_all(&id.to_be_bytes())?;
        f.write_all(&frame_offset.to_be_bytes())?;
        Ok(())
    }

    fn find_id_offset(&self, id: u32) -> Result<u64, StoreError> {
        let mut f = fs::OpenOptions::new().read(true).open(&self.index_path)?;
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        if id >= record_count {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        let entry_position = INDEX_HEADER_TOTAL_BYTES as u32 + id * INDEX_RECORD_LEN as u32;
        f.seek(SeekFrom::Start(entry_position as u64))?;
        let mut id_buf = [0u8; 4];
        f.read_exact(&mut id_buf)?;
        let index_id = u32::from_be_bytes(id_buf);
        if index_id != id {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        let mut offset_buf = [0u8; 8];
        f.read_exact(&mut offset_buf)?;
        let offset = u64::from_be_bytes(offset_buf);
        Ok(offset)
    }

    pub fn insert_one(&self, item: T) -> Result<(), StoreError>
    where
        T: Encode,
    {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.store_path)?;
        f.seek(SeekFrom::Start(STORAGE_NEXT_ID_OFFSET as u64))?;
        let mut next_id_buf = [0u8; 4];
        f.read_exact(&mut next_id_buf)?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let id = u32::from_be_bytes(next_id_buf);
        let next_id = id + 1;
        let record_count = u32::from_be_bytes(record_count_buf);
        let mut bytes = vec![];
        let payload = item.encode(id)?;
        let payload_size = u32::try_from(payload.len())?;
        f.seek(SeekFrom::Start(STORAGE_RECORD_COUNT_OFFSET as u64))?;
        f.write_all(&(record_count + 1).to_be_bytes())?;
        f.seek(SeekFrom::Start(STORAGE_NEXT_ID_OFFSET as u64))?;
        f.write_all(&(next_id).to_be_bytes())?;
        bytes.extend_from_slice(&payload_size.to_be_bytes());
        bytes.extend_from_slice(&payload);
        let frame_offset = f.seek(SeekFrom::End(0))?;
        f.write_all(&bytes)?;
        self.insert_index(id, frame_offset)?;
        Ok(())
    }

    pub fn delete_one(&mut self, id: u32) -> Result<(), StoreError> {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.store_path)?;
        let off_set = self.find_id_offset(id)?;
        f.seek(SeekFrom::Start(off_set + STORAGE_PAYLOAD_LEN_SIZE as u64))?;
        let mut id_buf = [0u8; 4];
        f.read_exact(&mut id_buf)?;
        let id = u32::from_be_bytes(id_buf);
        println!("{}", id);
        Ok(())
    }

    pub fn list(&mut self) -> Result<Vec<T>, StoreError>
    where
        T: Decode,
    {
        let f = fs::OpenOptions::new().read(true).open(&self.store_path)?;
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
            let mut f = fs::OpenOptions::new().read(true).open(&self.store_path)?;
            let mut header_buf = [0u8; STORAGE_HEADER_TOTAL_BYTES];
            f.read_exact(&mut header_buf)?;
            let record_count = u32::from_be_bytes(
                header_buf[STORAGE_RECORD_COUNT_OFFSET..STORAGE_HEADER_TOTAL_BYTES].try_into()?,
            );
            Ok(record_count)
        }
    }
}
