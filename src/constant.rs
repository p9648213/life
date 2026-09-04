// HTTP
pub const MAX_REQUEST_BYTES: usize = 1024 * 1024;
pub const MAX_BUFFER_SIZE: usize = 512;
pub const CONTENT_LENGTH: &str = "Content-Length";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const LOCATION: &str = "Location";
pub const TRANSFER_ENCODING: &str = "Transfer-Encoding";
pub const CONNECTION: &str = "Connection";
pub const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

// STORAGE COLLECTION
pub const STORAGE_FOLDER: &str = "storage";
pub const STORAGE_MAGIC: &str = "toikhongdien";
pub const STORAGE_MAGIC_END: usize = STORAGE_MAGIC.len();
pub const STORAGE_VERSION: u8 = 1;
pub const STORAGE_VERSION_OFFSET: usize = STORAGE_MAGIC_END;
pub const STORAGE_NEXT_ID: u32 = 1;
pub const STORAGE_NEXT_ID_OFFSET: usize = STORAGE_VERSION_OFFSET + size_of::<u8>();
pub const STORAGE_RECORD_COUNT: u32 = 0;
pub const STORAGE_RECORD_COUNT_OFFSET: usize = STORAGE_NEXT_ID_OFFSET + size_of::<u32>();
pub const STORAGE_DEAD_BYTES: u64 = 0;
pub const STORAGE_DEAD_BYTES_OFFSET: usize = STORAGE_RECORD_COUNT_OFFSET + size_of::<u32>();
pub const STORAGE_HEADER_TOTAL_BYTES: usize = STORAGE_DEAD_BYTES_OFFSET + size_of::<u64>();
pub const STORAGE_PAYLOAD_FRAME_LIVE: &[u8] = &[0b0000_0001];
pub const STORAGE_PAYLOAD_FRAME_OFF: &[u8] = &[0b0000_0000];
pub const STORAGE_PAYLOAD_FLAG_SIZE: usize = size_of::<u8>();
pub const STORAGE_PAYLOAD_LEN_SIZE: usize = size_of::<u32>();
pub const RESOURCE_COLLECTION: &str = "resource";
pub const COLLECTION_EXTENSION: &str = "store";

// STORAGE INDEX
pub const INDEX_MAGIC: &str = "toikhongdienindex";
pub const INDEX_MAGIC_END: usize = INDEX_MAGIC.len();
pub const INDEX_VERSION: u8 = 1;
pub const INDEX_VERSION_OFFSET: usize = INDEX_MAGIC_END;
pub const INDEX_RECORD_COUNT: u32 = 0;
pub const INDEX_RECORD_COUNT_OFFSET: usize = INDEX_VERSION_OFFSET + size_of::<u8>();
pub const INDEX_HEADER_TOTAL_BYTES: usize = INDEX_RECORD_COUNT_OFFSET + size_of::<u32>();
pub const INDEX_EXTENSION: &str = "idx";
pub const INDEX_RECORD_LEN: usize = size_of::<u32>() + size_of::<u64>();
