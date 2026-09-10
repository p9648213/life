use std::{
    array::TryFromSliceError,
    fmt::{self},
    num::TryFromIntError,
    str::Utf8Error,
};

#[derive(Debug)]
pub enum StoreError {
    ConnectionError(String),
    IoError(std::io::Error),
    FieldTooLarge(String),
    InvalidUtf8(Utf8Error),
    TryFromSliceError(TryFromSliceError),
    TryFromIntError(TryFromIntError),
    InvalidStorageFormat,
    UnsupportStorageVersion,
    UnsupportIndexVersion,
    StorageIndexIdNotFound,
    StorageIndexDeleted,
    IdNotMatch,
    OverflowPayloadSize,
    OverflowDeadbytesSize,
    InvalidStorageIndexFormat,
    OverflowId,
    UnexpectedEndOfPayload,
    RecordCountMismatch,
    DeadBytesMismatch,
    TrailingBytesInPayload,
    InvalidFrameFlag(u8),
    TruncatedFrame,
    InvalidCollectionName,
    IndexRecordIdMismatch {
        expected_id: u32,
        actual_id: u32,
    },
    IndexRecordOffsetMismatch {
        expected_offset: u64,
        actual_offset: u64,
    },
    InvalidNextId(u32),
    OverflowIndexRecordCount,
    OverflowStoreRecordCount,
}

impl From<std::io::Error> for StoreError {
    fn from(err: std::io::Error) -> Self {
        StoreError::IoError(err)
    }
}

impl From<Utf8Error> for StoreError {
    fn from(err: Utf8Error) -> Self {
        StoreError::InvalidUtf8(err)
    }
}

impl From<TryFromSliceError> for StoreError {
    fn from(err: TryFromSliceError) -> Self {
        StoreError::TryFromSliceError(err)
    }
}

impl From<TryFromIntError> for StoreError {
    fn from(err: TryFromIntError) -> Self {
        StoreError::TryFromIntError(err)
    }
}

impl std::error::Error for StoreError {}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::ConnectionError(err) => {
                write!(f, "Error connecting storage: {err}")
            }
            StoreError::IoError(err) => {
                write!(f, "IO error: {err}")
            }
            StoreError::FieldTooLarge(err) => {
                write!(f, "Field Too Large: {err}")
            }
            StoreError::InvalidUtf8(err) => {
                write!(f, "Invalid UTF-8 In Payload: {err}")
            }
            StoreError::TryFromSliceError(err) => {
                write!(f, "try_from slice error: {err}")
            }
            StoreError::TryFromIntError(err) => {
                write!(f, "try_from int error: {err}")
            }
            StoreError::InvalidStorageFormat => {
                write!(f, "Invalid storage format")
            }
            StoreError::UnsupportStorageVersion => {
                write!(f, "Unsupport storage version")
            }
            StoreError::UnsupportIndexVersion => {
                write!(f, "Unsupport index version")
            }
            StoreError::StorageIndexIdNotFound => {
                write!(f, "Storage index id not found")
            }
            StoreError::IdNotMatch => {
                write!(f, "Id not match")
            }
            StoreError::OverflowPayloadSize => {
                write!(f, "Overflow Payload Size")
            }
            StoreError::OverflowDeadbytesSize => {
                write!(f, "Overflow Deadbytes Size")
            }
            StoreError::StorageIndexDeleted => {
                write!(f, "Storage index deleted")
            }
            StoreError::InvalidStorageIndexFormat => {
                write!(f, "Invalid storage index format")
            }
            StoreError::OverflowId => {
                write!(f, "Overflow id")
            }
            StoreError::UnexpectedEndOfPayload => {
                write!(f, "Unexpected end of payload")
            }
            StoreError::RecordCountMismatch => {
                write!(f, "Record count mismatch")
            }
            StoreError::DeadBytesMismatch => {
                write!(
                    f,
                    "Storage dead_bytes does not match the total size of deleted frames"
                )
            }
            StoreError::IndexRecordIdMismatch {
                expected_id,
                actual_id,
            } => {
                write!(
                    f,
                    "index points to record ID {actual_id}, expected record ID {expected_id}"
                )
            }
            StoreError::IndexRecordOffsetMismatch {
                expected_offset,
                actual_offset,
            } => {
                write!(
                    f,
                    "index points to offset{actual_offset}, expected offset {expected_offset}"
                )
            }
            StoreError::TrailingBytesInPayload => {
                write!(f, "Trailing bytes in payload")
            }
            StoreError::InvalidFrameFlag(flag) => {
                write!(f, "Invalid Frame Flag: {flag}")
            }
            StoreError::TruncatedFrame => {
                write!(f, "Storage file ends before the frame is complete")
            }
            StoreError::InvalidCollectionName => {
                write!(f, "Invalid collection name")
            }
            StoreError::InvalidNextId(id) => {
                write!(f, "Invalid next_id {id}: ID has already been issued")
            }
            StoreError::OverflowIndexRecordCount => {
                write!(f, "Overflow index record count")
            }
            StoreError::OverflowStoreRecordCount => {
                write!(f, "Overflow store record count")
            }
        }
    }
}
