use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    path::PathBuf,
};

use crate::{
    constant::{
        INDEX_HEADER_TOTAL_BYTES, INDEX_MAGIC, INDEX_MAGIC_END, INDEX_RECORD_COUNT_OFFSET,
        INDEX_RECORD_LEN, INDEX_VERSION, STORAGE_DEAD_BYTES_OFFSET, STORAGE_HEADER_TOTAL_BYTES,
        STORAGE_MAGIC, STORAGE_MAGIC_END, STORAGE_NEXT_ID, STORAGE_NEXT_ID_OFFSET,
        STORAGE_PAYLOAD_FLAG_SIZE, STORAGE_PAYLOAD_FRAME_LIVE, STORAGE_PAYLOAD_FRAME_OFF,
        STORAGE_PAYLOAD_LEN_SIZE, STORAGE_RECORD_COUNT_OFFSET, STORAGE_VERSION,
    },
    storage::{
        decode::{Decode, Decoder},
        encode::Encode,
        error::StoreError,
        util::HasId,
    },
};

pub struct Colection<T> {
    store_file: File,
    index_file: File,
    _collection_type: PhantomData<T>,
}

impl<T> Colection<T> {
    pub fn new(store_path: PathBuf, index_path: PathBuf) -> Result<Self, StoreError> {
        Ok(Self {
            store_file: fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&store_path)?,
            index_file: fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&index_path)?,
            _collection_type: PhantomData,
        })
    }

    fn check_collection_header(&self) -> Result<(), StoreError> {
        let mut f = &self.store_file;
        f.seek(SeekFrom::Start(0))?;
        let mut magic_bytes_buf = [0u8; STORAGE_MAGIC_END];
        let mut version_buf = [0u8; 1];
        let mut next_id_buf = [0u8; 4];
        f.read_exact(&mut magic_bytes_buf)?;
        f.read_exact(&mut version_buf)?;
        f.read_exact(&mut next_id_buf)?;
        let magic_bytes = str::from_utf8(&magic_bytes_buf)?;
        let version = u8::from_be_bytes(version_buf);
        let next_id = u32::from_be_bytes(next_id_buf);
        if magic_bytes != STORAGE_MAGIC {
            return Err(StoreError::InvalidStorageFormat);
        }
        if version != STORAGE_VERSION {
            return Err(StoreError::UnsupportStorageVersion);
        }
        let mut f = &self.index_file;
        let file_len = f.metadata()?.len();
        f.seek(SeekFrom::Start(0))?;
        let mut magic_bytes_buf = [0u8; INDEX_MAGIC_END];
        let mut version_buf = [0u8; 1];
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut magic_bytes_buf)?;
        f.read_exact(&mut version_buf)?;
        f.read_exact(&mut record_count_buf)?;
        let magic_bytes = str::from_utf8(&magic_bytes_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        let version = u8::from_be_bytes(version_buf);
        if magic_bytes != INDEX_MAGIC {
            return Err(StoreError::InvalidStorageIndexFormat);
        }
        if version != INDEX_VERSION {
            return Err(StoreError::UnsupportIndexVersion);
        }
        let expect_len = u64::from(record_count)
            .checked_mul(INDEX_RECORD_LEN as u64)
            .and_then(|bytes| bytes.checked_add(INDEX_HEADER_TOTAL_BYTES as u64))
            .ok_or(StoreError::InvalidStorageIndexFormat)?;
        if expect_len != file_len {
            return Err(StoreError::InvalidStorageIndexFormat);
        }
        if next_id
            != record_count
                .checked_add(1)
                .ok_or(StoreError::OverflowIndexRecordCount)?
        {
            return Err(StoreError::InvalidStorageIndexFormat);
        }
        Ok(())
    }

    fn insert_index(&self, id: u32, frame_offset: u64) -> Result<(), StoreError> {
        let mut f = &self.index_file;
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        let check_record_count = record_count
            .checked_add(1)
            .ok_or(StoreError::OverflowIndexRecordCount)?;
        if check_record_count != id {
            return Err(StoreError::InvalidNextId(id));
        }
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        f.write_all(&check_record_count.to_be_bytes())?;
        f.seek(SeekFrom::End(0))?;
        f.write_all(&id.to_be_bytes())?;
        f.write_all(&frame_offset.to_be_bytes())?;
        Ok(())
    }

    fn find_id_offset(&self, id: u32) -> Result<u64, StoreError> {
        let mut f = &self.index_file;
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        if id > record_count || id < STORAGE_NEXT_ID {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        let entry_position =
            INDEX_HEADER_TOTAL_BYTES as u64 + (u64::from(id) - 1) * INDEX_RECORD_LEN as u64;
        f.seek(SeekFrom::Start(entry_position))?;
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

    fn update_id_offset(&self, id: u32, update_offset: u64) -> Result<(), StoreError> {
        let mut f = &self.index_file;
        f.seek(SeekFrom::Start(INDEX_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let record_count = u32::from_be_bytes(record_count_buf);
        if id > record_count || id < STORAGE_NEXT_ID {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        let entry_position =
            INDEX_HEADER_TOTAL_BYTES as u64 + (u64::from(id) - 1) * INDEX_RECORD_LEN as u64;
        f.seek(SeekFrom::Start(entry_position + 4))?;
        f.write_all(&update_offset.to_be_bytes())?;
        Ok(())
    }

    pub fn insert_one(&mut self, item: T) -> Result<(), StoreError>
    where
        T: Encode,
    {
        self.check_collection_header()?;
        let mut f = &self.store_file;
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
        let frame_offset = f.seek(SeekFrom::End(0))?;
        f.seek(SeekFrom::Start(STORAGE_RECORD_COUNT_OFFSET as u64))?;
        let check_record_count = record_count
            .checked_add(1)
            .ok_or(StoreError::OverflowStoreRecordCount)?;
        f.write_all(&check_record_count.to_be_bytes())?;
        f.seek(SeekFrom::Start(STORAGE_NEXT_ID_OFFSET as u64))?;
        f.write_all(&(next_id).to_be_bytes())?;
        bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_LIVE);
        bytes.extend_from_slice(&payload_size.to_be_bytes());
        bytes.extend_from_slice(&payload);
        f.seek(SeekFrom::End(0))?;
        f.write_all(&bytes)?;
        self.insert_index(id, frame_offset)?;
        Ok(())
    }

    pub fn delete_one(&mut self, id: u32) -> Result<(), StoreError>
    where
        T: Decode + HasId,
    {
        self.check_collection_header()?;
        let mut f = &self.store_file;
        let file_len = f.metadata()?.len();
        let offset = self.find_id_offset(id)?;
        if offset == 0 {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        let record_count_position = f.seek(SeekFrom::Start(STORAGE_RECORD_COUNT_OFFSET as u64))?;
        let mut record_count_buf = [0u8; 4];
        f.read_exact(&mut record_count_buf)?;
        let mut record_count = u32::from_be_bytes(record_count_buf);
        record_count = record_count
            .checked_sub(1)
            .ok_or(StoreError::RecordCountMismatch)?;
        f.seek(SeekFrom::Start(offset))?;
        let mut flag_buf = [0u8; 1];
        f.read_exact(&mut flag_buf)?;
        let flag = u8::from_be_bytes(flag_buf);
        if flag == 0 {
            return Err(StoreError::StorageIndexDeleted);
        }
        if !(0..=1).contains(&flag) {
            return Err(StoreError::InvalidFrameFlag(flag));
        }
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
        let dead_bytes = (payload_len as usize)
            .checked_add(STORAGE_PAYLOAD_FLAG_SIZE)
            .ok_or(StoreError::OverflowDeadbytesSize)?
            .checked_add(STORAGE_PAYLOAD_LEN_SIZE)
            .ok_or(StoreError::OverflowDeadbytesSize)?;
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
        f.seek(SeekFrom::Start(STORAGE_DEAD_BYTES_OFFSET as u64))?;
        let mut dead_bytes_buf = [0u8; 8];
        f.read_exact(&mut dead_bytes_buf)?;
        let total_dead_bytes = u64::from_be_bytes(dead_bytes_buf);
        let update_dead_bytes = total_dead_bytes
            .checked_add(dead_bytes as u64)
            .ok_or(StoreError::OverflowDeadbytesSize)?;
        f.seek(SeekFrom::Current(-8))?;
        f.write_all(&update_dead_bytes.to_be_bytes())?;
        f.seek(SeekFrom::Start(offset))?;
        f.write_all(STORAGE_PAYLOAD_FRAME_OFF)?;
        f.seek(SeekFrom::Start(record_count_position))?;
        f.write_all(&record_count.to_be_bytes())?;
        self.update_id_offset(id, 0)?;
        Ok(())
    }

    pub fn update_one(&mut self, id: u32, item: T) -> Result<(), StoreError>
    where
        T: Encode + Decode + HasId,
    {
        self.check_collection_header()?;
        let mut f = &self.store_file;
        let file_len = f.metadata()?.len();
        let mut bytes = vec![];
        let payload = item.encode(id)?;
        let payload_size = u32::try_from(payload.len())?;
        bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_LIVE);
        bytes.extend_from_slice(&payload_size.to_be_bytes());
        bytes.extend_from_slice(&payload);
        let old_offset = self.find_id_offset(id)?;
        if old_offset == 0 {
            return Err(StoreError::StorageIndexIdNotFound);
        }
        f.seek(SeekFrom::Start(
            old_offset
                .checked_add(1)
                .ok_or(StoreError::InvalidStorageIndexFormat)?,
        ))?;
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
        let mut flag_buf = [0u8; 1];
        let mut payload_size_buf = [0u8; 4];
        f.read_exact(&mut flag_buf)?;
        f.read_exact(&mut payload_size_buf)?;
        let flag = u8::from_be_bytes(flag_buf);
        if flag == 0 {
            return Err(StoreError::StorageIndexDeleted);
        }
        if !(0..=1).contains(&flag) {
            return Err(StoreError::InvalidFrameFlag(flag));
        }
        let payload_size = u32::from_be_bytes(payload_size_buf);
        let total_record_bytes = payload_size
            .checked_add(5)
            .ok_or(StoreError::OverflowDeadbytesSize)?;
        f.seek(SeekFrom::Start(STORAGE_DEAD_BYTES_OFFSET as u64))?;
        let mut dead_bytes_buf = [0u8; 8];
        f.read_exact(&mut dead_bytes_buf)?;
        let dead_bytes = u64::from_be_bytes(dead_bytes_buf);
        f.seek(SeekFrom::Current(-8))?;
        let update_dead_bytes = (total_record_bytes as u64)
            .checked_add(dead_bytes)
            .ok_or(StoreError::OverflowDeadbytesSize)?;
        f.write_all(&update_dead_bytes.to_be_bytes())?;
        f.seek(SeekFrom::Start(old_offset))?;
        f.write_all(STORAGE_PAYLOAD_FRAME_OFF)?;
        let frame_offset = f.seek(SeekFrom::End(0))?;
        f.write_all(&bytes)?;
        self.update_id_offset(id, frame_offset)?;
        Ok(())
    }

    pub fn list(&mut self) -> Result<Vec<T>, StoreError>
    where
        T: Decode + HasId,
    {
        self.check_collection_header()?;
        let mut f = &self.store_file;
        f.seek(SeekFrom::Start(0))?;
        let mut reader = BufReader::new(f);
        let file_len = reader.get_ref().metadata()?.len();
        let mut header_buf = [0u8; STORAGE_HEADER_TOTAL_BYTES];
        reader.read_exact(&mut header_buf)?;
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
        let mut items = vec![];
        let mut live_entry = HashMap::new();
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
                let record_offset = payload_start - 5;
                let mut payload = vec![0u8; len];
                reader.read_exact(&mut payload)?;
                let mut decoder = Decoder::new(&payload);
                let item = T::decode(&mut decoder)?;
                if !decoder.bytes.is_empty() {
                    return Err(StoreError::TrailingBytesInPayload);
                }
                let id = item.id();
                let offset = self.find_id_offset(id)?;
                if record_offset != offset {
                    return Err(StoreError::IndexRecordOffsetMismatch {
                        expected_offset: record_offset,
                        actual_offset: offset,
                    });
                }
                if id >= next_id {
                    return Err(StoreError::InvalidNextId(next_id));
                }
                items.push(item);
                live_entry.insert(id, offset);
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
        let f = &self.index_file;
        let mut reader = BufReader::new(f);
        reader.seek(SeekFrom::Start(INDEX_HEADER_TOTAL_BYTES as u64))?;
        let mut expected_id = 1;
        loop {
            let mut id_buf = [0u8; 4];
            let mut offset_buf = [0u8; 8];
            match reader.read_exact(&mut id_buf) {
                Ok(()) => {}
                Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(StoreError::IoError(err)),
            };
            reader.read_exact(&mut offset_buf)?;
            let id = u32::from_be_bytes(id_buf);
            let offset = u64::from_be_bytes(offset_buf);
            if id != expected_id {
                return Err(StoreError::InvalidStorageIndexFormat);
            }
            expected_id += 1;
            let expected_offset = live_entry.get(&id).copied().unwrap_or(0);
            if offset != expected_offset {
                return Err(StoreError::IndexRecordOffsetMismatch {
                    expected_offset,
                    actual_offset: offset,
                });
            }
        }
        Ok(items)
    }

    pub fn record_count(&self) -> Result<u32, StoreError> {
        self.check_collection_header()?;
        let mut f = &self.store_file;
        f.seek(SeekFrom::Start(0))?;
        let mut header_buf = [0u8; STORAGE_HEADER_TOTAL_BYTES];
        f.read_exact(&mut header_buf)?;
        let record_count = u32::from_be_bytes(
            header_buf[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET].try_into()?,
        );
        Ok(record_count)
    }
}
