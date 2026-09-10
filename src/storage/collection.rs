use std::{
    fs::{self, File}, io::{self, BufReader, Read, Seek, SeekFrom, Write}, marker::PhantomData, path::PathBuf,
};

use crate::{
    constant::{
        INDEX_HEADER_TOTAL_BYTES, INDEX_MAGIC, INDEX_MAGIC_END, INDEX_RECORD_COUNT_OFFSET,
        INDEX_RECORD_LEN, STORAGE_DEAD_BYTES_OFFSET, STORAGE_HEADER_TOTAL_BYTES, STORAGE_MAGIC,
        STORAGE_MAGIC_END, STORAGE_NEXT_ID, STORAGE_NEXT_ID_OFFSET, STORAGE_PAYLOAD_FLAG_SIZE,
        STORAGE_PAYLOAD_FRAME_LIVE, STORAGE_PAYLOAD_FRAME_OFF, STORAGE_PAYLOAD_LEN_SIZE,
        STORAGE_RECORD_COUNT_OFFSET, STORAGE_VERSION, STORAGE_VERSION_OFFSET,
    },
    storage::{
        decode::{Decode, Decoder},
        encode::Encode,
        error::StoreError,
        util::HasId,
    },
};

pub struct Colection<T> {
    // TODO: IMPROVE FILE OPEN
    store_path: PathBuf,
    index_path: PathBuf,
    _collection_type: PhantomData<T>,
}

impl<T> Colection<T> {
    pub fn new(store_path: PathBuf, index_path: PathBuf) -> Self {
        Self {
            store_path,
            index_path,
            _collection_type: PhantomData,
        }
    }

    fn check_index_magic_bytes(&self, f: &mut File) -> Result<(), StoreError> {
        let mut magic_bytes_buff = [0u8; INDEX_MAGIC_END];
        f.read_exact(&mut magic_bytes_buff)?;
        let magic_bytes = str::from_utf8(&magic_bytes_buff)?;
        if magic_bytes != INDEX_MAGIC {
            return Err(StoreError::InvalidStorageIndexFormat);
        }
        Ok(())
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
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.index_path)?;
        self.check_index_magic_bytes(&mut f)?;
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        if id > record_count || id < STORAGE_NEXT_ID {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        let entry_position = INDEX_HEADER_TOTAL_BYTES as u32 + (id - 1) * INDEX_RECORD_LEN as u32;
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
        if offset == 0 {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        Ok(offset)
    }

    fn update_id_offset(&self, id: u32, update_offset: u64) -> Result<(), StoreError> {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.index_path)?;
        let mut magic_bytes_buff = [0u8; INDEX_MAGIC_END];
        f.read_exact(&mut magic_bytes_buff)?;
        let magic_bytes = str::from_utf8(&magic_bytes_buff)?;
        if magic_bytes != INDEX_MAGIC {
            return Err(StoreError::InvalidStorageIndexFormat);
        }
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        if id > record_count || id < STORAGE_NEXT_ID {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        let entry_position = INDEX_HEADER_TOTAL_BYTES as u32 + (id - 1) * INDEX_RECORD_LEN as u32;
        f.seek(SeekFrom::Start(entry_position as u64 + 4))?;
        f.write_all(&update_offset.to_be_bytes())?;
        Ok(())
    }

    pub fn insert_one(&mut self, item: T) -> Result<(), StoreError>
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
        match self.find_id_offset(id) {
            Ok(_) => return Err(StoreError::InvalidNextId(id)),
            Err(StoreError::StorageIndexIdNotFound) => {}
            Err(error) => return Err(error),
        }
        let next_id = if let Some(value) = id.checked_add(1) {
            value
        } else {
            return Err(StoreError::OverflowId);
        };
        let record_count = u32::from_be_bytes(record_count_buf);
        let mut bytes = vec![];
        let payload = item.encode(id)?;
        let payload_size = u32::try_from(payload.len())?;
        f.seek(SeekFrom::Start(STORAGE_RECORD_COUNT_OFFSET as u64))?;
        f.write_all(&(record_count + 1).to_be_bytes())?;
        f.seek(SeekFrom::Start(STORAGE_NEXT_ID_OFFSET as u64))?;
        f.write_all(&(next_id).to_be_bytes())?;
        bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_LIVE);
        bytes.extend_from_slice(&payload_size.to_be_bytes());
        bytes.extend_from_slice(&payload);
        let frame_offset = f.seek(SeekFrom::End(0))?;
        self.insert_index(id, frame_offset)?;
        f.write_all(&bytes)?;
        Ok(())
    }

    pub fn delete_one(&mut self, id: u32) -> Result<(), StoreError>
    where
        T: Decode + HasId,
    {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.store_path)?;
        let file_len = f.metadata()?.len();
        let off_set = self.find_id_offset(id)?;
        let record_count_position = f.seek(SeekFrom::Start(STORAGE_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let mut record_count = u32::from_be_bytes(record_count_buf);
        record_count = record_count
            .checked_sub(1)
            .ok_or(StoreError::RecordCountMismatch)?;
        f.seek(SeekFrom::Start(off_set + 1))?;
        let mut payload_len_buf = [0u8; 4];
        f.read_exact(&mut payload_len_buf)?;
        let payload_start = f.stream_position()?;
        let remaining = file_len
            .checked_sub(payload_start)
            .ok_or(StoreError::TruncatedFrame)?;
        let payload_len = u32::from_be_bytes(payload_len_buf);
        if payload_len as u64 > remaining {
            return Err(StoreError::TruncatedFrame);
        }
        let mut payload = vec![0u8; payload_len as usize];
        f.read_exact(&mut payload)?;
        let mut decoder = Decoder::new(&payload);
        let item = T::decode(&mut decoder)?;
        if !decoder.bytes.is_empty() {
            return Err(StoreError::TrailingBytesInPayload);
        }
        if item.id() != id {
            return Err(StoreError::IndexRecordIdMismatch {
                expected_id: id,
                actual_id: item.id(),
            });
        }
        self.update_id_offset(id, 0)?;
        f.seek(SeekFrom::Start(off_set))?;
        f.write_all(STORAGE_PAYLOAD_FRAME_OFF)?;
        f.seek(SeekFrom::Start(record_count_position))?;
        f.write_all(&record_count.to_be_bytes())?;
        let dead_bytes =
            STORAGE_PAYLOAD_FLAG_SIZE + STORAGE_PAYLOAD_LEN_SIZE + payload_len as usize;
        let mut dead_bytes_buf = [0u8; 8];
        f.read_exact(&mut dead_bytes_buf)?;
        let total_dead_bytes = u64::from_be_bytes(dead_bytes_buf);
        f.seek(SeekFrom::Current(-8))?;
        f.write_all(&(total_dead_bytes + dead_bytes as u64).to_be_bytes())?;
        Ok(())
    }

    pub fn update_one(&mut self, id: u32, item: T) -> Result<(), StoreError>
    where
        T: Encode + Decode + HasId,
    {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.store_path)?;
        let file_len = f.metadata()?.len();
        let mut bytes = vec![];
        let payload = item.encode(id)?;
        let payload_size = u32::try_from(payload.len())?;
        bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_LIVE);
        bytes.extend_from_slice(&payload_size.to_be_bytes());
        bytes.extend_from_slice(&payload);
        let old_offset = self.find_id_offset(id)?;
        f.seek(SeekFrom::Start(old_offset + 1))?;
        let mut old_payload_len_buf = [0u8; 4];
        f.read_exact(&mut old_payload_len_buf)?;
        let old_payload_start = f.stream_position()?;
        let remaining = file_len
            .checked_sub(old_payload_start)
            .ok_or(StoreError::TruncatedFrame)?;
        let old_payload_len = u32::from_be_bytes(old_payload_len_buf);
        if old_payload_len as u64 > remaining {
            return Err(StoreError::TruncatedFrame);
        }
        let mut old_payload = vec![0u8; old_payload_len as usize];
        f.read_exact(&mut old_payload)?;
        let mut decoder = Decoder::new(&old_payload);
        let old_item = T::decode(&mut decoder)?;
        if !decoder.bytes.is_empty() {
            return Err(StoreError::TrailingBytesInPayload);
        }
        if old_item.id() != id {
            return Err(StoreError::IndexRecordIdMismatch {
                expected_id: id,
                actual_id: old_item.id(),
            });
        }
        f.seek(SeekFrom::Start(old_offset))?;
        f.write_all(STORAGE_PAYLOAD_FRAME_OFF)?;
        let mut payload_size_buf = [0u8; 4];
        f.read_exact(&mut payload_size_buf)?;
        let payload_size = u32::from_be_bytes(payload_size_buf);
        let total_record_bytes = 5 + payload_size;
        f.seek(SeekFrom::Start(STORAGE_DEAD_BYTES_OFFSET as u64))?;
        let mut dead_bytes_buf = [0u8; 8];
        f.read_exact(&mut dead_bytes_buf)?;
        let dead_bytes = u64::from_be_bytes(dead_bytes_buf);
        f.seek(SeekFrom::Current(-8))?;
        f.write_all(&(total_record_bytes as u64 + dead_bytes).to_be_bytes())?;
        let frame_offset = f.seek(SeekFrom::End(0))?;
        f.write_all(&bytes)?;
        self.update_id_offset(id, frame_offset)?;
        Ok(())
    }

    pub fn list(&mut self) -> Result<Vec<T>, StoreError>
    where
        T: Decode + HasId,
    {
        let f = fs::OpenOptions::new().read(true).open(&self.store_path)?;
        let mut reader = BufReader::new(f);
        let file_len = reader.get_ref().metadata()?.len();
        let mut header_buf = [0u8; STORAGE_HEADER_TOTAL_BYTES];
        reader.read_exact(&mut header_buf)?;
        let magic_bytes = str::from_utf8(&header_buf[..STORAGE_MAGIC_END])?;
        let version = header_buf[STORAGE_VERSION_OFFSET];
        let next_id = u32::from_be_bytes(
            header_buf[STORAGE_NEXT_ID_OFFSET..STORAGE_RECORD_COUNT_OFFSET].try_into()?,
        );
        let record_count = u32::from_be_bytes(
            header_buf[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET].try_into()?,
        );
        let dead_bytes = u64::from_be_bytes(
            header_buf[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES].try_into()?,
        );
        let mut total_record_dead_bytes = 0;
        if magic_bytes != STORAGE_MAGIC {
            return Err(StoreError::InvalidStorageFormat);
        }
        if version != STORAGE_VERSION {
            return Err(StoreError::UnsupportVersion);
        }
        let mut items = vec![];
        loop {
            let mut flag_buf = [0u8; 1];
            match reader.read_exact(&mut flag_buf) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(StoreError::IoError(err)),
            };
            let flag = u8::from_be_bytes(flag_buf);
            let mut len_buf = [0u8; 4];
            match reader.read_exact(&mut len_buf) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                    return Err(StoreError::TruncatedFrame);
                }
                Err(err) => return Err(StoreError::IoError(err)),
            };
            let len = u32::from_be_bytes(len_buf) as usize;
            let payload_start = reader.stream_position()?;
            let remaining = file_len
                .checked_sub(payload_start)
                .ok_or(StoreError::TruncatedFrame)?;
            if len > remaining as usize {
                return Err(StoreError::TruncatedFrame);
            }
            if flag == 0 {
                total_record_dead_bytes = total_record_dead_bytes + 5 + len;
                reader.seek(SeekFrom::Current(len as i64))?;
            } else if flag == 1 {
                let mut payload = vec![0u8; len];
                reader.read_exact(&mut payload)?;
                let mut decoder = Decoder::new(&payload);
                let item = T::decode(&mut decoder)?;
                if !decoder.bytes.is_empty() {
                    return Err(StoreError::TrailingBytesInPayload);
                }
                let id = item.id();
                let _ = self.find_id_offset(id)?;
                if id >= next_id {
                    return Err(StoreError::InvalidNextId(next_id));
                }
                items.push(item);
                if items.len() as u32 > record_count {
                    return Err(StoreError::RecordCountMismatch);
                }
            } else {
                return Err(StoreError::InvalidFrameFlag(flag));
            }
        }
        if record_count != items.len() as u32 {
            return Err(StoreError::RecordCountMismatch);
        }
        if total_record_dead_bytes as u64 != dead_bytes {
            return Err(StoreError::DeadBytesMismatch);
        }
        Ok(items)
    }

    pub fn record_count(&self) -> Result<u32, StoreError> {
        let mut f = fs::OpenOptions::new().read(true).open(&self.store_path)?;
        let mut header_buf = [0u8; STORAGE_HEADER_TOTAL_BYTES];
        f.read_exact(&mut header_buf)?;
        let record_count = u32::from_be_bytes(
            header_buf[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET].try_into()?,
        );
        Ok(record_count)
    }
}
