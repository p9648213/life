use std::{
    fs::{self, OpenOptions},
    io::{Seek, SeekFrom, Write},
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
};

use life::{
    constant::{
        COLLECTION_EXTENSION, INDEX_EXTENSION, STORAGE_DEAD_BYTES_OFFSET,
        STORAGE_HEADER_TOTAL_BYTES, STORAGE_MAGIC, STORAGE_NEXT_ID_OFFSET,
        STORAGE_PAYLOAD_FLAG_SIZE, STORAGE_PAYLOAD_FRAME_LIVE, STORAGE_PAYLOAD_FRAME_OFF,
        STORAGE_PAYLOAD_LEN_SIZE, STORAGE_RECORD_COUNT_OFFSET, STORAGE_VERSION,
        STORAGE_VERSION_OFFSET,
    },
    storage::{
        decode::{Decode, Decoder},
        encode::{Encode, Encoder},
        error::StoreError,
        store::Store,
    },
};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);
static DECODE_CALLS: AtomicUsize = AtomicUsize::new(0);
static DECODED_PAYLOAD_BYTES: AtomicUsize = AtomicUsize::new(0);

struct TestDirectory {
    root: PathBuf,
    storage_root: PathBuf,
}

impl TestDirectory {
    fn new() -> Self {
        let unique_id = NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("life-phase-09a-{}-{unique_id}", std::process::id()));
        fs::create_dir(&root).expect("create isolated Phase 09A test directory");
        let storage_root = root.join("storage");
        Self { root, storage_root }
    }

    fn connect(&self) -> Store {
        Store::connect(
            self.storage_root
                .to_str()
                .expect("temporary storage path is valid UTF-8"),
        )
        .expect("connect test store")
    }

    fn store_path(&self, collection: &str) -> PathBuf {
        self.storage_root
            .join(format!("{collection}.{COLLECTION_EXTENSION}"))
    }

    fn index_path(&self, collection: &str) -> PathBuf {
        self.storage_root
            .join(format!("{collection}.{INDEX_EXTENSION}"))
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        if self.root.starts_with(std::env::temp_dir()) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TestRecord {
    id: u32,
    name: String,
    number: u32,
}

impl TestRecord {
    fn new(name: impl Into<String>, number: u32) -> Self {
        Self {
            id: 0,
            name: name.into(),
            number,
        }
    }
}

impl Encode for TestRecord {
    fn encode(&self, id: u32) -> Result<Vec<u8>, StoreError> {
        let mut encoder = Encoder::new();
        encoder.write_u32(id);
        encoder.write_string(&self.name)?;
        encoder.write_u32(self.number);
        Ok(encoder.bytes)
    }
}

impl Decode for TestRecord {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, StoreError> {
        Ok(Self {
            id: decoder.read_u32()?,
            name: decoder.read_str()?.to_owned(),
            number: decoder.read_u32()?,
        })
    }
}

struct CountingRecord(TestRecord);

impl Encode for CountingRecord {
    fn encode(&self, id: u32) -> Result<Vec<u8>, StoreError> {
        self.0.encode(id)
    }
}

impl Decode for CountingRecord {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, StoreError> {
        DECODE_CALLS.fetch_add(1, Ordering::Relaxed);
        let bytes_before = decoder.bytes.len();
        let record = TestRecord::decode(decoder)?;
        DECODED_PAYLOAD_BYTES.fetch_add(bytes_before - decoder.bytes.len(), Ordering::Relaxed);
        Ok(Self(record))
    }
}

fn create_collection(directory: &TestDirectory, name: &str) -> Store {
    let store = directory.connect();
    store
        .create_collection(name)
        .expect("create test collection");
    store
}

fn encoded_payload(id: u32, name: &str, number: u32) -> Vec<u8> {
    TestRecord::new(name, number)
        .encode(id)
        .expect("encode test record")
}

fn storage_header(next_id: u32, record_count: u32, dead_bytes: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(STORAGE_HEADER_TOTAL_BYTES);
    bytes.extend_from_slice(STORAGE_MAGIC.as_bytes());
    bytes.push(STORAGE_VERSION);
    bytes.extend_from_slice(&next_id.to_be_bytes());
    bytes.extend_from_slice(&record_count.to_be_bytes());
    bytes.extend_from_slice(&dead_bytes.to_be_bytes());
    assert_eq!(bytes.len(), STORAGE_HEADER_TOTAL_BYTES);
    bytes
}

fn append_frame(bytes: &mut Vec<u8>, payload: &[u8]) {
    let payload_len = u32::try_from(payload.len()).expect("test payload length fits u32");
    bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_LIVE);
    bytes.extend_from_slice(&payload_len.to_be_bytes());
    bytes.extend_from_slice(payload);
}

fn write_store_bytes(directory: &TestDirectory, collection: &str, bytes: &[u8]) {
    fs::write(directory.store_path(collection), bytes).expect("write test store bytes");
}

fn assert_operation_returns_error_without_panicking<T>(
    operation: impl FnOnce() -> Result<T, StoreError>,
) {
    let outcome = panic::catch_unwind(AssertUnwindSafe(operation));
    assert!(outcome.is_ok(), "malformed storage must not panic");
    assert!(
        outcome.expect("outcome checked above").is_err(),
        "malformed storage must return an explicit error"
    );
}

fn file_contains(path: &Path, needle: &[u8]) -> bool {
    fs::read(path)
        .expect("read collection file")
        .windows(needle.len())
        .any(|window| window == needle)
}

#[test]
fn primitive_u32_values_round_trip_in_big_endian_order() {
    let values = [0, 1, 0x0102_0304, u32::MAX];
    let mut encoder = Encoder::new();

    for value in values {
        encoder.write_u32(value);
    }

    assert_eq!(&encoder.bytes[8..12], &[1, 2, 3, 4]);

    let mut decoder = Decoder::new(&encoder.bytes);
    for expected in values {
        assert_eq!(decoder.read_u32().unwrap(), expected);
    }
    assert!(decoder.bytes.is_empty());
}

#[test]
fn strings_with_phase_09a_boundary_content_round_trip() {
    let values = ["", "xin chào 🦀", "a=b&c|d", "first line\nsecond line"];
    let mut encoder = Encoder::new();

    for value in values {
        encoder.write_string(value).unwrap();
    }

    let mut decoder = Decoder::new(&encoder.bytes);
    for expected in values {
        assert_eq!(decoder.read_str().unwrap(), expected);
    }
    assert!(decoder.bytes.is_empty());
}

#[test]
fn concrete_record_round_trips_through_its_manual_codec() {
    let original = TestRecord::new("manual codec", 42);
    let payload = original.encode(7).unwrap();
    let mut decoder = Decoder::new(&payload);

    let decoded = TestRecord::decode(&mut decoder).unwrap();

    assert_eq!(decoded, TestRecord { id: 7, ..original });
    assert!(decoder.bytes.is_empty());
}

#[test]
fn multiple_records_survive_reopening_the_store() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "resources");
    let collection = store.collection::<TestRecord>("resources");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.insert_one(TestRecord::new("two 🦀", 2)).unwrap();
    drop(collection);
    drop(store);

    let reopened_store = directory.connect();
    let mut reopened = reopened_store.collection::<TestRecord>("resources");

    assert_eq!(
        reopened.list().unwrap(),
        vec![
            TestRecord {
                id: 1,
                name: "one".into(),
                number: 1,
            },
            TestRecord {
                id: 2,
                name: "two 🦀".into(),
                number: 2,
            },
        ]
    );
    assert_eq!(reopened.record_count().unwrap(), 2);
}

#[test]
fn uncached_record_count_reads_only_the_record_count_header_field() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "uncached_count");
    let collection = store.collection::<TestRecord>("uncached_count");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.insert_one(TestRecord::new("two", 2)).unwrap();

    assert_eq!(collection.record_count().unwrap(), 2);
}

#[test]
fn invalid_storage_magic_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "invalid_magic");
    let path = directory.store_path("invalid_magic");
    let mut bytes = fs::read(&path).unwrap();
    bytes[0] ^= 0x01;
    fs::write(path, bytes).unwrap();
    let mut collection = store.collection::<TestRecord>("invalid_magic");

    assert!(matches!(
        collection.list(),
        Err(StoreError::InvalidStorageFormat)
    ));
}

#[test]
fn unsupported_storage_version_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "unsupported_version");
    let path = directory.store_path("unsupported_version");
    let mut bytes = fs::read(&path).unwrap();
    bytes[STORAGE_VERSION_OFFSET] = STORAGE_VERSION.wrapping_add(1);
    fs::write(path, bytes).unwrap();
    let mut collection = store.collection::<TestRecord>("unsupported_version");

    assert!(matches!(
        collection.list(),
        Err(StoreError::UnsupportVersion)
    ));
}

#[test]
fn invalid_utf8_inside_a_record_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "invalid_utf8");
    let mut payload = Vec::new();
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.push(0xff);
    payload.extend_from_slice(&7u32.to_be_bytes());
    let mut bytes = storage_header(2, 1, 0);
    append_frame(&mut bytes, &payload);
    write_store_bytes(&directory, "invalid_utf8", &bytes);
    let mut collection = store.collection::<TestRecord>("invalid_utf8");

    assert!(matches!(collection.list(), Err(StoreError::InvalidUtf8(_))));
}

#[test]
fn truncated_record_payload_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "truncated_record");
    let mut bytes = storage_header(2, 1, 0);
    bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_LIVE);
    bytes.extend_from_slice(&12u32.to_be_bytes());
    bytes.extend_from_slice(&[0, 0, 0]);
    write_store_bytes(&directory, "truncated_record", &bytes);
    let mut collection = store.collection::<TestRecord>("truncated_record");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn truncated_primitive_returns_error_instead_of_panicking() {
    let mut decoder = Decoder::new(&[0, 0]);

    assert_operation_returns_error_without_panicking(|| decoder.read_u32());
}

#[test]
fn field_length_larger_than_its_payload_returns_error_instead_of_panicking() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "truncated_field");
    let mut payload = Vec::new();
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.extend_from_slice(&100u32.to_be_bytes());
    payload.extend_from_slice(b"short");
    let mut bytes = storage_header(2, 1, 0);
    append_frame(&mut bytes, &payload);
    write_store_bytes(&directory, "truncated_field", &bytes);
    let mut collection = store.collection::<TestRecord>("truncated_field");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn partial_frame_length_prefix_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "partial_prefix");
    let mut bytes = storage_header(2, 1, 0);
    bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_LIVE);
    bytes.extend_from_slice(&[0, 0, 0]);
    write_store_bytes(&directory, "partial_prefix", &bytes);
    let mut collection = store.collection::<TestRecord>("partial_prefix");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn record_count_larger_than_available_frames_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "missing_frame");
    let mut bytes = storage_header(3, 2, 0);
    append_frame(&mut bytes, &encoded_payload(1, "only", 1));
    write_store_bytes(&directory, "missing_frame", &bytes);
    let mut collection = store.collection::<TestRecord>("missing_frame");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn frames_beyond_the_declared_record_count_are_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "trailing_frame");
    let mut bytes = storage_header(3, 1, 0);
    append_frame(&mut bytes, &encoded_payload(1, "declared", 1));
    append_frame(&mut bytes, &encoded_payload(2, "trailing", 2));
    write_store_bytes(&directory, "trailing_frame", &bytes);
    let mut collection = store.collection::<TestRecord>("trailing_frame");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn trailing_bytes_inside_a_framed_record_are_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "trailing_payload");
    let mut payload = encoded_payload(1, "valid fields", 1);
    payload.push(0xff);
    let mut bytes = storage_header(2, 1, 0);
    append_frame(&mut bytes, &payload);
    write_store_bytes(&directory, "trailing_payload", &bytes);
    let mut collection = store.collection::<TestRecord>("trailing_payload");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn invalid_frame_flag_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "invalid_frame_flag");
    let mut bytes = storage_header(1, 0, 0);
    bytes.push(0b0000_0010);
    bytes.extend_from_slice(&0u32.to_be_bytes());
    write_store_bytes(&directory, "invalid_frame_flag", &bytes);
    let mut collection = store.collection::<TestRecord>("invalid_frame_flag");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn truncated_tombstoned_frame_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "truncated_tombstone");
    let declared_payload_len = 8u32;
    let dead_bytes = STORAGE_PAYLOAD_FLAG_SIZE as u64
        + STORAGE_PAYLOAD_LEN_SIZE as u64
        + u64::from(declared_payload_len);
    let mut bytes = storage_header(1, 0, dead_bytes);
    bytes.extend_from_slice(STORAGE_PAYLOAD_FRAME_OFF);
    bytes.extend_from_slice(&declared_payload_len.to_be_bytes());
    bytes.extend_from_slice(&[0, 0, 0]);
    write_store_bytes(&directory, "truncated_tombstone", &bytes);
    let mut collection = store.collection::<TestRecord>("truncated_tombstone");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn traversal_collection_identifier_is_rejected() {
    let directory = TestDirectory::new();
    let store = directory.connect();

    assert!(store.create_collection("../escaped").is_err());
    assert!(!directory.root.join("escaped.store").exists());
    assert!(!directory.root.join("escaped.idx").exists());
}

#[test]
fn absolute_collection_identifier_is_rejected() {
    let directory = TestDirectory::new();
    let store = directory.connect();
    let outside_collection = directory.root.join("absolute-escape");

    assert!(store
        .create_collection(outside_collection.to_str().unwrap())
        .is_err());
    assert!(!directory.root.join("absolute-escape.store").exists());
    assert!(!directory.root.join("absolute-escape.idx").exists());
}

#[test]
fn middle_deletion_tombstones_bytes_and_preserves_other_records() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "delete_middle");
    let mut collection = store.collection::<TestRecord>("delete_middle");
    collection
        .insert_one(TestRecord::new("record-one", 1))
        .unwrap();
    collection
        .insert_one(TestRecord::new("unique-deleted-record", 2))
        .unwrap();
    collection
        .insert_one(TestRecord::new("record-three", 3))
        .unwrap();
    let deleted_payload = encoded_payload(2, "unique-deleted-record", 2);
    let deleted_frame_len =
        (STORAGE_PAYLOAD_FLAG_SIZE + STORAGE_PAYLOAD_LEN_SIZE + deleted_payload.len()) as u64;
    let original_bytes = fs::read(directory.store_path("delete_middle")).unwrap();

    collection.delete_one(2).unwrap();

    assert_eq!(
        collection.list().unwrap(),
        vec![
            TestRecord {
                id: 1,
                name: "record-one".into(),
                number: 1,
            },
            TestRecord {
                id: 3,
                name: "record-three".into(),
                number: 3,
            },
        ]
    );
    let updated_bytes = fs::read(directory.store_path("delete_middle")).unwrap();
    assert_eq!(updated_bytes.len(), original_bytes.len());
    assert!(file_contains(
        &directory.store_path("delete_middle"),
        b"unique-deleted-record"
    ));
    assert_eq!(
        u64::from_be_bytes(
            updated_bytes[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .try_into()
                .unwrap()
        ),
        deleted_frame_len
    );
    let deleted_payload_offset = updated_bytes
        .windows(deleted_payload.len())
        .position(|window| window == deleted_payload)
        .unwrap();
    let deleted_flag_offset =
        deleted_payload_offset - STORAGE_PAYLOAD_LEN_SIZE - STORAGE_PAYLOAD_FLAG_SIZE;
    assert_eq!(
        &updated_bytes[deleted_flag_offset..deleted_flag_offset + STORAGE_PAYLOAD_FLAG_SIZE],
        STORAGE_PAYLOAD_FRAME_OFF
    );
}

#[test]
fn deleting_records_around_an_existing_index_hole_preserves_later_ids() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "delete_holes");
    let mut collection = store.collection::<TestRecord>("delete_holes");
    for number in 1..=4 {
        collection
            .insert_one(TestRecord::new(format!("record-{number}"), number))
            .unwrap();
    }

    collection.delete_one(2).unwrap();
    collection.delete_one(1).unwrap();

    assert!(
        collection.delete_one(3).is_ok(),
        "deleting earlier records must not turn surviving ID 3 into an empty index slot"
    );
    assert_eq!(
        collection.list().unwrap(),
        vec![TestRecord {
            id: 4,
            name: "record-4".into(),
            number: 4,
        }]
    );
}

#[test]
fn deleting_id_zero_returns_error_instead_of_panicking() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "delete_zero");
    let mut collection = store.collection::<TestRecord>("delete_zero");
    collection.insert_one(TestRecord::new("record", 1)).unwrap();

    assert_operation_returns_error_without_panicking(|| collection.delete_one(0));
}

#[test]
fn next_id_remains_monotonic_after_delete_and_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "stable_ids");
    let mut collection = store.collection::<TestRecord>("stable_ids");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.insert_one(TestRecord::new("two", 2)).unwrap();
    collection.delete_one(2).unwrap();
    drop(collection);
    drop(store);

    let reopened_store = directory.connect();
    let mut reopened = reopened_store.collection::<TestRecord>("stable_ids");
    reopened.insert_one(TestRecord::new("three", 3)).unwrap();

    assert_eq!(
        reopened
            .list()
            .unwrap()
            .into_iter()
            .map(|record| record.id)
            .collect::<Vec<_>>(),
        vec![1, 3]
    );
}

#[test]
fn exhausted_next_id_returns_error_instead_of_panicking_or_reusing_zero() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "id_overflow");
    let mut file = OpenOptions::new()
        .write(true)
        .open(directory.store_path("id_overflow"))
        .unwrap();
    file.seek(SeekFrom::Start(STORAGE_NEXT_ID_OFFSET as u64))
        .unwrap();
    file.write_all(&u32::MAX.to_be_bytes()).unwrap();
    drop(file);
    let collection = store.collection::<TestRecord>("id_overflow");

    assert_operation_returns_error_without_panicking(|| {
        collection.insert_one(TestRecord::new("must fail", 1))
    });
}

#[test]
fn cached_record_count_tracks_completed_deletion() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "cached_count");
    let mut collection = store.collection::<TestRecord>("cached_count");
    collection
        .insert_one(TestRecord::new("only record", 1))
        .unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    assert_eq!(collection.record_count().unwrap(), 1);

    collection.delete_one(1).unwrap();

    assert_eq!(collection.record_count().unwrap(), 0);
}

#[test]
fn index_write_failure_is_reported_and_inconsistent_reopen_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "failed_insert");
    let index_path = directory.index_path("failed_insert");
    fs::remove_file(&index_path).unwrap();
    fs::create_dir(&index_path).unwrap();
    let collection = store.collection::<TestRecord>("failed_insert");

    assert!(
        collection
            .insert_one(TestRecord::new("partially written", 1))
            .is_err(),
        "an index write failure must not be reported as success"
    );
    drop(collection);
    drop(store);

    let reopened_store = directory.connect();
    let mut reopened = reopened_store.collection::<TestRecord>("failed_insert");
    assert!(
        reopened.list().is_err(),
        "reopening must reject a store/index pair left inconsistent by a failed mutation"
    );
}

#[test]
fn corrupt_index_header_is_rejected_before_mutation() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "corrupt_index");
    let mut collection = store.collection::<TestRecord>("corrupt_index");
    collection
        .insert_one(TestRecord::new("must remain", 1))
        .unwrap();
    let index_path = directory.index_path("corrupt_index");
    let mut bytes = fs::read(&index_path).unwrap();
    bytes[0] ^= 0x01;
    fs::write(index_path, bytes).unwrap();

    assert_operation_returns_error_without_panicking(|| collection.delete_one(1));
    assert_eq!(
        collection.list().unwrap(),
        vec![TestRecord {
            id: 1,
            name: "must remain".into(),
            number: 1,
        }]
    );
}

#[test]
fn interrupted_insert_header_without_frame_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "interrupted_insert");
    let bytes = storage_header(2, 1, 0);
    write_store_bytes(&directory, "interrupted_insert", &bytes);
    let mut collection = store.collection::<TestRecord>("interrupted_insert");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn large_valid_file_decodes_each_payload_once() {
    const RECORDS: usize = 1_024;

    let directory = TestDirectory::new();
    let store = create_collection(&directory, "linear_decode");
    let collection = store.collection::<CountingRecord>("linear_decode");
    let mut expected_payload_bytes = 0;
    for number in 0..RECORDS {
        let record = TestRecord::new(format!("record-{number:04}"), number as u32);
        expected_payload_bytes += record.encode(number as u32 + 1).unwrap().len();
        collection.insert_one(CountingRecord(record)).unwrap();
    }
    DECODE_CALLS.store(0, Ordering::Relaxed);
    DECODED_PAYLOAD_BYTES.store(0, Ordering::Relaxed);
    let mut reopened = directory
        .connect()
        .collection::<CountingRecord>("linear_decode");

    assert_eq!(reopened.list().unwrap().len(), RECORDS);
    assert_eq!(DECODE_CALLS.load(Ordering::Relaxed), RECORDS);
    assert_eq!(
        DECODED_PAYLOAD_BYTES.load(Ordering::Relaxed),
        expected_payload_bytes
    );
}

#[test]
fn storage_header_fields_use_the_documented_offsets() {
    let bytes = storage_header(9, 7, 123);

    assert_eq!(
        u32::from_be_bytes(
            bytes[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET]
                .try_into()
                .unwrap()
        ),
        7
    );
    assert_eq!(
        u64::from_be_bytes(
            bytes[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .try_into()
                .unwrap()
        ),
        123
    );
}

#[test]
fn update_preserves_id_and_every_unaffected_record() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "update_record");
    let mut collection = store.collection::<TestRecord>("update_record");
    for number in 1..=3 {
        collection
            .insert_one(TestRecord::new(format!("record-{number}"), number))
            .unwrap();
    }

    collection
        .update_one(2, TestRecord::new("updated-record-2", 22))
        .unwrap();

    let mut records = collection.list().unwrap();
    records.sort_by_key(|record| record.id);
    assert_eq!(
        records,
        vec![
            TestRecord {
                id: 1,
                name: "record-1".into(),
                number: 1,
            },
            TestRecord {
                id: 2,
                name: "updated-record-2".into(),
                number: 22,
            },
            TestRecord {
                id: 3,
                name: "record-3".into(),
                number: 3,
            },
        ]
    );

    drop(collection);
    drop(store);
    let reopened_store = directory.connect();
    let mut reopened = reopened_store.collection::<TestRecord>("update_record");
    let mut reopened_records = reopened.list().unwrap();
    reopened_records.sort_by_key(|record| record.id);
    assert_eq!(reopened_records, records);
}

#[test]
fn update_adds_the_replaced_frame_to_dead_bytes() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "update_dead_bytes");
    let mut collection = store.collection::<TestRecord>("update_dead_bytes");
    collection
        .insert_one(TestRecord::new("original-record", 1))
        .unwrap();
    let replaced_payload = encoded_payload(1, "original-record", 1);
    let replaced_frame_len =
        (STORAGE_PAYLOAD_FLAG_SIZE + STORAGE_PAYLOAD_LEN_SIZE + replaced_payload.len()) as u64;

    collection
        .update_one(1, TestRecord::new("replacement-record", 2))
        .unwrap();

    let bytes = fs::read(directory.store_path("update_dead_bytes")).unwrap();
    assert_eq!(
        u64::from_be_bytes(
            bytes[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .try_into()
                .unwrap()
        ),
        replaced_frame_len
    );
}
