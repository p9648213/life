// HTTP
pub const MAX_REQUEST_BYTES: usize = 1024 * 1024;
pub const MAX_BUFFER_SIZE: usize = 512;
pub const CONTENT_LENGTH: &str = "Content-Length";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const LOCATION: &str = "Location";
pub const TRANSFER_ENCODING: &str = "Transfer-Encoding";
pub const CONNECTION: &str = "Connection";
pub const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

// STORAGE
pub const STORAGE_URL: &str = "storage";
pub const STORAGE_MAGIC: &str = "toikhongdien";
pub const STORAGE_MAGIC_END: usize = STORAGE_MAGIC.len();
pub const STORAGE_VERSION: u8 = 1;
pub const STORAGE_VERSION_OFFSET: usize = STORAGE_MAGIC_END;
pub const STORAGE_NEXT_ID: u32 = 0;
pub const STORAGE_NEXT_ID_OFFSET: usize = STORAGE_VERSION_OFFSET + size_of::<u8>();
pub const STORAGE_RECORD_COUNT: u32 = 0;
pub const STORAGE_RECORD_COUNT_OFFSET: usize = STORAGE_NEXT_ID_OFFSET + size_of::<u32>();
pub const STORAGE_HEADER_TOTAL_BYTES: usize = STORAGE_RECORD_COUNT_OFFSET + size_of::<u32>();
pub const RESOURCE_COLLECTION: &str = "resource";
