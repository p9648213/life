use std::{
    fs::{self, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
};

use life::{
    constant::{
        COLLECTION_EXTENSION, INDEX_EXTENSION, INDEX_HEADER_TOTAL_BYTES, INDEX_RECORD_COUNT_OFFSET,
        INDEX_RECORD_LEN, INDEX_VERSION_OFFSET, STORAGE_DEAD_BYTES_OFFSET,
        STORAGE_HEADER_TOTAL_BYTES, STORAGE_MAGIC, STORAGE_NEXT_ID_OFFSET,
        STORAGE_PAYLOAD_FLAG_SIZE, STORAGE_PAYLOAD_FRAME_LIVE, STORAGE_PAYLOAD_FRAME_OFF,
        STORAGE_PAYLOAD_LEN_SIZE, STORAGE_RECORD_COUNT_OFFSET, STORAGE_VERSION,
        STORAGE_VERSION_OFFSET,
    },
    storage::{
        collection::Colection,
        decode::{Decode, Decoder},
        encode::{Encode, Encoder},
        error::StoreError,
        store::Store,
        util::HasId,
    },
};

mod support;

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

impl HasId for TestRecord {
    fn id(&self) -> u32 {
        self.id
    }
}

struct CountingRecord(TestRecord);

impl HasId for CountingRecord {
    fn id(&self) -> u32 {
        self.0.id()
    }
}

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
    let mut collection = store
        .collection::<TestRecord>("resources")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.insert_one(TestRecord::new("two 🦀", 2)).unwrap();
    drop(collection);
    drop(store);

    let reopened_store = directory.connect();
    let mut reopened = reopened_store
        .collection::<TestRecord>("resources")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("uncached_count")
        .expect("select collection with a valid name");
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
    let mut collection = store
        .collection::<TestRecord>("invalid_magic")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("unsupported_version")
        .expect("select collection with a valid name");

    assert!(matches!(
        collection.list(),
        Err(StoreError::UnsupportStorageVersion)
    ));
}

#[test]
fn invalid_utf8_inside_a_record_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "invalid_utf8");
    let mut collection = store
        .collection::<TestRecord>("invalid_utf8")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("a", 7)).unwrap();
    drop(collection);

    let mut bytes = fs::read(directory.store_path("invalid_utf8")).unwrap();
    let payload_start =
        STORAGE_HEADER_TOTAL_BYTES + STORAGE_PAYLOAD_FLAG_SIZE + STORAGE_PAYLOAD_LEN_SIZE;
    let name_offset = payload_start + 4 + 4; // Skip the ID and name length.
    assert_eq!(bytes[name_offset], b'a');
    bytes[name_offset] = 0xff;
    write_store_bytes(&directory, "invalid_utf8", &bytes);
    let mut collection = store
        .collection::<TestRecord>("invalid_utf8")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("truncated_record")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("truncated_field")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("partial_prefix")
        .expect("select collection with a valid name");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn partial_frame_length_after_declared_records_is_rejected() {
    for flag in [STORAGE_PAYLOAD_FRAME_LIVE, STORAGE_PAYLOAD_FRAME_OFF] {
        for prefix_len in 0..STORAGE_PAYLOAD_LEN_SIZE {
            let directory = TestDirectory::new();
            let store = create_collection(&directory, "trailing_partial_prefix");
            let mut collection = store
                .collection::<TestRecord>("trailing_partial_prefix")
                .expect("select collection with a valid name");
            collection
                .insert_one(TestRecord::new("complete", 1))
                .expect("insert complete record with a matching index entry");
            let mut bytes = fs::read(directory.store_path("trailing_partial_prefix")).unwrap();

            // EOF between frames is valid, and the live count already matches.
            assert_eq!(collection.list().expect("read complete frame").len(), 1);

            bytes.extend_from_slice(flag);
            bytes.extend_from_slice(&8u32.to_be_bytes()[..prefix_len]);
            write_store_bytes(&directory, "trailing_partial_prefix", &bytes);

            // A flag starts another frame, which requires all four length bytes.
            assert!(matches!(collection.list(), Err(StoreError::TruncatedFrame)));
        }
    }
}

#[test]
fn record_count_larger_than_available_frames_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "missing_frame");
    let mut collection = store
        .collection::<TestRecord>("missing_frame")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("only", 1)).unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);

    // Keep the record and index valid; only the declared live count is wrong.
    let mut bytes = fs::read(directory.store_path("missing_frame")).unwrap();
    bytes[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET]
        .copy_from_slice(&2u32.to_be_bytes());
    write_store_bytes(&directory, "missing_frame", &bytes);

    assert!(matches!(
        collection.list(),
        Err(StoreError::RecordCountMismatch)
    ));
}

#[test]
fn frames_beyond_the_declared_record_count_are_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "trailing_frame");
    let mut collection = store
        .collection::<TestRecord>("trailing_frame")
        .expect("select collection with a valid name");
    collection
        .insert_one(TestRecord::new("declared", 1))
        .unwrap();
    collection
        .insert_one(TestRecord::new("trailing", 2))
        .unwrap();
    assert_eq!(collection.list().unwrap().len(), 2);

    // Both frames have valid index entries, so an index error cannot mask this check.
    let mut bytes = fs::read(directory.store_path("trailing_frame")).unwrap();
    bytes[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET]
        .copy_from_slice(&1u32.to_be_bytes());
    write_store_bytes(&directory, "trailing_frame", &bytes);

    assert!(matches!(
        collection.list(),
        Err(StoreError::RecordCountMismatch)
    ));
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
    let mut collection = store
        .collection::<TestRecord>("trailing_payload")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("invalid_frame_flag")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("truncated_tombstone")
        .expect("select collection with a valid name");

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

    assert!(
        store
            .create_collection(outside_collection.to_str().unwrap())
            .is_err()
    );
    assert!(!directory.root.join("absolute-escape.store").exists());
    assert!(!directory.root.join("absolute-escape.idx").exists());
}

#[test]
fn middle_deletion_tombstones_bytes_and_preserves_other_records() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "delete_middle");
    let mut collection = store
        .collection::<TestRecord>("delete_middle")
        .expect("select collection with a valid name");
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
    let mut collection = store
        .collection::<TestRecord>("delete_holes")
        .expect("select collection with a valid name");
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
    let mut collection = store
        .collection::<TestRecord>("delete_zero")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("record", 1)).unwrap();

    assert_operation_returns_error_without_panicking(|| collection.delete_one(0));
}

#[test]
fn next_id_remains_monotonic_after_delete_and_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "stable_ids");
    let mut collection = store
        .collection::<TestRecord>("stable_ids")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.insert_one(TestRecord::new("two", 2)).unwrap();
    collection.delete_one(2).unwrap();
    drop(collection);
    drop(store);

    let reopened_store = directory.connect();
    let mut reopened = reopened_store
        .collection::<TestRecord>("stable_ids")
        .expect("select collection with a valid name");
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
    let mut collection = store
        .collection::<TestRecord>("id_overflow")
        .expect("select collection with a valid name");

    assert_operation_returns_error_without_panicking(|| {
        collection.insert_one(TestRecord::new("must fail", 1))
    });
}

#[test]
fn cached_record_count_tracks_completed_deletion() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "cached_count");
    let mut collection = store
        .collection::<TestRecord>("cached_count")
        .expect("select collection with a valid name");
    collection
        .insert_one(TestRecord::new("only record", 1))
        .unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    assert_eq!(collection.record_count().unwrap(), 1);

    collection.delete_one(1).unwrap();

    assert_eq!(collection.record_count().unwrap(), 0);
}

#[test]
fn opening_collection_with_directory_instead_of_index_is_rejected() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "failed_insert");
    let index_path = directory.index_path("failed_insert");
    let store_path = directory.store_path("failed_insert");
    let original_store_bytes = fs::read(&store_path).unwrap();
    fs::remove_file(&index_path).unwrap();
    fs::create_dir(&index_path).unwrap();

    // Collection construction now opens both persistent file handles, so the
    // invalid index must be rejected before a mutation can be attempted.
    assert_operation_returns_error_without_panicking(|| {
        store.collection::<TestRecord>("failed_insert")
    });
    drop(store);

    let reopened_store = directory.connect();
    assert_operation_returns_error_without_panicking(|| {
        reopened_store.collection::<TestRecord>("failed_insert")
    });
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert!(
        index_path.is_dir(),
        "opening must not replace the invalid index"
    );
}

#[test]
fn corrupt_index_header_is_rejected_before_mutation() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "corrupt_index");
    let mut collection = store
        .collection::<TestRecord>("corrupt_index")
        .expect("select collection with a valid name");
    collection
        .insert_one(TestRecord::new("must remain", 1))
        .unwrap();
    let original_store_bytes = fs::read(directory.store_path("corrupt_index")).unwrap();
    let index_path = directory.index_path("corrupt_index");
    let mut bytes = fs::read(&index_path).unwrap();
    bytes[0] ^= 0x01;
    fs::write(&index_path, &bytes).unwrap();

    assert_operation_returns_error_without_panicking(|| collection.delete_one(1));
    // The index remains corrupt, so check preservation without requiring list()
    // to accept the inconsistent collection.
    assert_eq!(
        fs::read(directory.store_path("corrupt_index")).unwrap(),
        original_store_bytes
    );
    assert_eq!(fs::read(index_path).unwrap(), bytes);
}

#[test]
fn interrupted_insert_header_without_frame_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "interrupted_insert");
    let bytes = storage_header(2, 1, 0);
    write_store_bytes(&directory, "interrupted_insert", &bytes);
    let mut collection = store
        .collection::<TestRecord>("interrupted_insert")
        .expect("select collection with a valid name");

    assert_operation_returns_error_without_panicking(|| collection.list());
}

#[test]
fn large_valid_file_decodes_each_payload_once() {
    const RECORDS: usize = 1_024;

    let directory = TestDirectory::new();
    let store = create_collection(&directory, "linear_decode");
    let mut collection = store
        .collection::<CountingRecord>("linear_decode")
        .expect("select collection with a valid name");
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
        .collection::<CountingRecord>("linear_decode")
        .expect("select collection with a valid name");

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
    let mut collection = store
        .collection::<TestRecord>("update_record")
        .expect("select collection with a valid name");
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
    let mut reopened = reopened_store
        .collection::<TestRecord>("update_record")
        .expect("select collection with a valid name");
    let mut reopened_records = reopened.list().unwrap();
    reopened_records.sort_by_key(|record| record.id);
    assert_eq!(reopened_records, records);
}

#[test]
fn update_adds_the_replaced_frame_to_dead_bytes() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "update_dead_bytes");
    let mut collection = store
        .collection::<TestRecord>("update_dead_bytes")
        .expect("select collection with a valid name");
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

fn assert_collection_lookup_cannot_escape_storage_root(absolute: bool) {
    let directory = TestDirectory::new();
    // Both roots are inside the isolated temporary directory. The outer store
    // supplies an existing target that an invalid lookup could otherwise reach.
    let outside = Store::connect(directory.root.to_str().unwrap()).unwrap();
    outside.create_collection("escaped").unwrap();
    let mut target = outside
        .collection::<TestRecord>("escaped")
        .expect("select collection with a valid name");
    target.insert_one(TestRecord::new("outside", 1)).unwrap();
    let store_path = directory.root.join("escaped.store");
    let index_path = directory.root.join("escaped.idx");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let original_index_bytes = fs::read(&index_path).unwrap();
    let name = if absolute {
        directory.root.join("escaped").to_str().unwrap().to_owned()
    } else {
        "../escaped".to_owned()
    };
    let inside = directory.connect();

    // Reject the name during selection, before a handle can expose any reads
    // or mutations of the outside collection.
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| inside.collection::<TestRecord>(&name)));
    assert!(
        matches!(outcome, Ok(Err(StoreError::InvalidCollectionName))),
        "an escaping collection name must return InvalidCollectionName without panicking"
    );
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert_eq!(fs::read(index_path).unwrap(), original_index_bytes);
}

#[test]
fn traversal_collection_lookup_cannot_read_or_mutate_outside_storage_root() {
    assert_collection_lookup_cannot_escape_storage_root(false);
}

#[test]
fn absolute_collection_lookup_cannot_read_or_mutate_outside_storage_root() {
    assert_collection_lookup_cannot_escape_storage_root(true);
}

#[test]
fn cached_record_count_tracks_mutations_through_another_handle() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "shared_count");
    let mut first = store
        .collection::<TestRecord>("shared_count")
        .expect("select collection with a valid name");
    assert!(first.list().unwrap().is_empty());
    let mut second = store
        .collection::<TestRecord>("shared_count")
        .expect("select collection with a valid name");

    second.insert_one(TestRecord::new("one", 1)).unwrap();
    assert_eq!(
        first.record_count().unwrap(),
        1,
        "a completed insertion through another handle must be reflected in the count"
    );

    second.delete_one(1).unwrap();
    assert_eq!(
        first.record_count().unwrap(),
        0,
        "a completed deletion through another handle must be reflected in the count"
    );
}

#[test]
fn deletion_after_insertion_through_another_handle_does_not_panic() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "shared_delete");
    let mut first = store
        .collection::<TestRecord>("shared_delete")
        .expect("select collection with a valid name");
    assert!(first.list().unwrap().is_empty());
    let mut second = store
        .collection::<TestRecord>("shared_delete")
        .expect("select collection with a valid name");
    second.insert_one(TestRecord::new("one", 1)).unwrap();
    let original_bytes = fs::read(directory.store_path("shared_delete")).unwrap();

    // These operations are sequential. An existing record remains deletable
    // even when this handle last observed the collection while it was empty.
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| first.delete_one(1)));
    assert!(
        outcome.is_ok(),
        "a stale cached count must not cause a panic"
    );
    outcome.unwrap().expect("delete the existing record");
    assert_eq!(first.record_count().unwrap(), 0);

    let bytes = fs::read(directory.store_path("shared_delete")).unwrap();
    assert_eq!(
        u64::from_be_bytes(
            bytes[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .try_into()
                .unwrap()
        ),
        (original_bytes.len() - STORAGE_HEADER_TOTAL_BYTES) as u64,
        "the successful deletion must finish updating dead_bytes"
    );
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("shared_delete")
        .expect("select collection with a valid name");
    assert!(reopened.list().unwrap().is_empty());
    assert_eq!(reopened.record_count().unwrap(), 0);
}

#[test]
fn interrupted_delete_after_clearing_index_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "interrupted_delete_index");
    let mut collection = store
        .collection::<TestRecord>("interrupted_delete_index")
        .expect("select collection with a valid name");
    collection
        .insert_one(TestRecord::new("still live", 1))
        .unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    drop(collection);
    drop(store);

    // Simulate interruption after clearing ID 1's index offset, before its
    // live frame or collection metadata has been changed.
    let index_path = directory.index_path("interrupted_delete_index");
    let mut bytes = fs::read(&index_path).unwrap();
    bytes[INDEX_HEADER_TOTAL_BYTES + 4..INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN]
        .copy_from_slice(&0u64.to_be_bytes());
    fs::write(index_path, bytes).unwrap();

    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("interrupted_delete_index")
        .expect("select collection with a valid name");
    assert_operation_returns_error_without_panicking(|| reopened.list());
}

#[test]
fn index_offset_for_another_record_is_rejected_before_deletion() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "wrong_index_offset");
    let mut collection = store
        .collection::<TestRecord>("wrong_index_offset")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.insert_one(TestRecord::new("two", 2)).unwrap();
    let original_store_bytes = fs::read(directory.store_path("wrong_index_offset")).unwrap();
    let index_path = directory.index_path("wrong_index_offset");
    let mut bytes = fs::read(&index_path).unwrap();
    let second_entry = INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN;
    let second_offset: [u8; 8] = bytes[second_entry + 4..second_entry + INDEX_RECORD_LEN]
        .try_into()
        .unwrap();
    // Keep the entry's ID as 1, but redirect it to ID 2's valid live frame.
    bytes[INDEX_HEADER_TOTAL_BYTES + 4..INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN]
        .copy_from_slice(&second_offset);
    fs::write(&index_path, &bytes).unwrap();

    assert_operation_returns_error_without_panicking(|| collection.delete_one(1));
    assert_eq!(
        fs::read(directory.store_path("wrong_index_offset")).unwrap(),
        original_store_bytes,
        "rejecting a bad offset must preserve both records"
    );
    assert_eq!(fs::read(index_path).unwrap(), bytes);
}

fn collection_with_regressed_next_id(directory: &TestDirectory, name: &str) -> Store {
    let store = create_collection(directory, name);
    let mut collection = store
        .collection::<TestRecord>(name)
        .expect("select collection with a valid name");
    collection
        .insert_one(TestRecord::new("original", 1))
        .unwrap();
    assert_eq!(collection.list().unwrap()[0].id, 1);
    drop(collection);

    let path = directory.store_path(name);
    let mut bytes = fs::read(&path).unwrap();
    // Only next_id is corrupted; the frame, live count, and index still agree.
    bytes[STORAGE_NEXT_ID_OFFSET..STORAGE_RECORD_COUNT_OFFSET].copy_from_slice(&1u32.to_be_bytes());
    fs::write(path, bytes).unwrap();
    store
}

#[test]
fn regressed_next_id_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = collection_with_regressed_next_id(&directory, "regressed_next_id");
    drop(store);
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("regressed_next_id")
        .expect("select collection with a valid name");

    assert_operation_returns_error_without_panicking(|| reopened.list());
}

#[test]
fn insertion_rejects_regressed_next_id_before_reusing_an_id() {
    let directory = TestDirectory::new();
    let store = collection_with_regressed_next_id(&directory, "duplicate_id");
    drop(store);
    let store_path = directory.store_path("duplicate_id");
    let index_path = directory.index_path("duplicate_id");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let original_index_bytes = fs::read(&index_path).unwrap();
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("duplicate_id")
        .expect("select collection with a valid name");

    // Insertion must be safe even when the caller has not called list() first.
    assert_operation_returns_error_without_panicking(|| {
        reopened.insert_one(TestRecord::new("must not reuse ID 1", 2))
    });
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert_eq!(fs::read(index_path).unwrap(), original_index_bytes);
}

#[test]
fn creating_collection_with_missing_index_does_not_silently_reset_it() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "missing_index");
    let mut collection = store
        .collection::<TestRecord>("missing_index")
        .expect("select collection with a valid name");
    collection
        .insert_one(TestRecord::new("original", 1))
        .unwrap();
    drop(collection);
    let store_path = directory.store_path("missing_index");
    let original_store_bytes = fs::read(&store_path).unwrap();
    fs::remove_file(directory.index_path("missing_index")).unwrap();

    let result = store.create_collection("missing_index");
    assert_eq!(fs::read(&store_path).unwrap(), original_store_bytes);
    // Explicit rejection is sufficient. If initialization succeeds, the
    // original IDs and subsequent mutations must all remain usable.
    if result.is_err() {
        return;
    }
    drop(store);
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("missing_index")
        .expect("select collection with a valid name");
    assert_eq!(
        reopened
            .list()
            .expect("read the successfully initialized collection"),
        vec![TestRecord {
            id: 1,
            name: "original".into(),
            number: 1,
        }]
    );
    reopened.insert_one(TestRecord::new("second", 2)).unwrap();
    assert_eq!(
        reopened
            .list()
            .unwrap()
            .into_iter()
            .map(|record| record.id)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    reopened
        .delete_one(2)
        .expect("the new ID must have a usable index entry");
    reopened
        .delete_one(1)
        .expect("the original ID must retain a usable index entry");
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("missing_index")
        .expect("select collection with a valid name");
    assert!(reopened.list().unwrap().is_empty());
}

#[test]
fn inconsistent_dead_bytes_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "inconsistent_dead_bytes");
    let mut collection = store
        .collection::<TestRecord>("inconsistent_dead_bytes")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.delete_one(1).unwrap();
    assert!(collection.list().unwrap().is_empty());
    drop(collection);
    drop(store);

    let path = directory.store_path("inconsistent_dead_bytes");
    let mut bytes = fs::read(&path).unwrap();
    // This is also the state left if deletion updates the index, frame state,
    // and live count, but is interrupted before writing dead_bytes.
    bytes[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
        .copy_from_slice(&0u64.to_be_bytes());
    fs::write(path, bytes).unwrap();

    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("inconsistent_dead_bytes")
        .expect("select collection with a valid name");
    assert_operation_returns_error_without_panicking(|| reopened.list());
}

#[test]
fn index_offset_for_another_record_is_rejected_before_update() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "wrong_update_offset");
    let mut collection = store
        .collection::<TestRecord>("wrong_update_offset")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    collection.insert_one(TestRecord::new("two", 2)).unwrap();
    assert_eq!(collection.list().unwrap().len(), 2);
    let store_path = directory.store_path("wrong_update_offset");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let index_path = directory.index_path("wrong_update_offset");
    let mut bytes = fs::read(&index_path).unwrap();
    let second_entry = INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN;
    let second_offset: [u8; 8] = bytes[second_entry + 4..second_entry + INDEX_RECORD_LEN]
        .try_into()
        .unwrap();
    // Updating ID 1 must not tombstone ID 2 or append another live ID 1.
    bytes[INDEX_HEADER_TOTAL_BYTES + 4..INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN]
        .copy_from_slice(&second_offset);
    fs::write(&index_path, &bytes).unwrap();

    assert_operation_returns_error_without_panicking(|| {
        collection.update_one(1, TestRecord::new("replacement", 11))
    });
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert_eq!(fs::read(index_path).unwrap(), bytes);
}

#[test]
fn deletion_rejects_truncated_payload_before_allocating_declared_length() {
    // Large enough to distinguish the payload allocation from ordinary I/O,
    // but small enough to run the regression safely while the defect exists.
    const DECLARED_PAYLOAD_LEN: u32 = 8 * 1024 * 1024;

    let directory = TestDirectory::new();
    let store = create_collection(&directory, "truncated_delete_payload");
    let mut collection = store
        .collection::<TestRecord>("truncated_delete_payload")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    let store_path = directory.store_path("truncated_delete_payload");
    let index_path = directory.index_path("truncated_delete_payload");
    let original_index_bytes = fs::read(&index_path).unwrap();
    let mut bytes = fs::read(&store_path).unwrap();
    let length_start = STORAGE_HEADER_TOTAL_BYTES + STORAGE_PAYLOAD_FLAG_SIZE;
    bytes[length_start..length_start + STORAGE_PAYLOAD_LEN_SIZE]
        .copy_from_slice(&DECLARED_PAYLOAD_LEN.to_be_bytes());
    fs::write(&store_path, &bytes).unwrap();
    assert!(bytes.len() < DECLARED_PAYLOAD_LEN as usize);

    let (_, largest_allocation) = support::measure_largest_allocation(|| {
        assert_operation_returns_error_without_panicking(|| collection.delete_one(1));
    });
    assert!(
        largest_allocation < DECLARED_PAYLOAD_LEN as usize,
        "deletion allocated {largest_allocation} bytes for a payload extending beyond EOF"
    );
    assert_eq!(fs::read(store_path).unwrap(), bytes);
    assert_eq!(fs::read(index_path).unwrap(), original_index_bytes);
}

#[test]
fn corrupt_index_header_is_rejected_before_insertion() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "corrupt_index_insert");
    let mut collection = store
        .collection::<TestRecord>("corrupt_index_insert")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    let store_path = directory.store_path("corrupt_index_insert");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let index_path = directory.index_path("corrupt_index_insert");
    let mut bytes = fs::read(&index_path).unwrap();
    bytes[0] ^= 0x01;
    fs::write(&index_path, &bytes).unwrap();

    // A lookup error caused by corruption must not authorize another insertion.
    assert_operation_returns_error_without_panicking(|| {
        collection.insert_one(TestRecord::new("must not be appended", 2))
    });
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert_eq!(fs::read(index_path).unwrap(), bytes);
}

fn collection_with_regressed_next_id_after_deletion(
    directory: &TestDirectory,
    name: &str,
) -> Store {
    let store = create_collection(directory, name);
    let mut collection = store
        .collection::<TestRecord>(name)
        .expect("select collection with a valid name");
    collection
        .insert_one(TestRecord::new("original", 1))
        .unwrap();
    collection.delete_one(1).unwrap();
    assert!(collection.list().unwrap().is_empty());
    drop(collection);

    let path = directory.store_path(name);
    let mut bytes = fs::read(&path).unwrap();
    assert_eq!(
        u32::from_be_bytes(
            bytes[STORAGE_NEXT_ID_OFFSET..STORAGE_RECORD_COUNT_OFFSET]
                .try_into()
                .unwrap()
        ),
        2
    );
    // ID 1 remains issued even though its frame is now a tombstone.
    bytes[STORAGE_NEXT_ID_OFFSET..STORAGE_RECORD_COUNT_OFFSET].copy_from_slice(&1u32.to_be_bytes());
    fs::write(path, bytes).unwrap();
    store
}

#[test]
fn regressed_next_id_after_deletion_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = collection_with_regressed_next_id_after_deletion(&directory, "deleted_next_id");
    drop(store);

    assert_collection_rejects_without_changes(&directory, "deleted_next_id", |collection| {
        collection.list()
    });
}

#[test]
fn insertion_rejects_regressed_next_id_before_reusing_a_deleted_id() {
    let directory = TestDirectory::new();
    let store = collection_with_regressed_next_id_after_deletion(&directory, "reuse_deleted_id");
    drop(store);
    let store_path = directory.store_path("reuse_deleted_id");
    let index_path = directory.index_path("reuse_deleted_id");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let original_index_bytes = fs::read(&index_path).unwrap();
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("reuse_deleted_id")
        .expect("select collection with a valid name");

    // Exercise insertion directly so rejection cannot depend on a prior list().
    assert_operation_returns_error_without_panicking(|| {
        reopened.insert_one(TestRecord::new("must not reuse deleted ID 1", 2))
    });
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert_eq!(fs::read(index_path).unwrap(), original_index_bytes);
}

#[test]
fn interrupted_update_after_redirecting_index_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "interrupted_update_index");
    let mut collection = store
        .collection::<TestRecord>("interrupted_update_index")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("old", 1)).unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    drop(collection);
    drop(store);

    let store_path = directory.store_path("interrupted_update_index");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let index_path = directory.index_path("interrupted_update_index");
    let mut bytes = fs::read(&index_path).unwrap();
    // Simulate stopping after redirecting the index to the future replacement,
    // before appending it. The old frame and all storage metadata still agree.
    bytes[INDEX_HEADER_TOTAL_BYTES + 4..INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN]
        .copy_from_slice(&(original_store_bytes.len() as u64).to_be_bytes());
    fs::write(&index_path, &bytes).unwrap();
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("interrupted_update_index")
        .expect("select collection with a valid name");

    assert_operation_returns_error_without_panicking(|| reopened.list());
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert_eq!(fs::read(index_path).unwrap(), bytes);
}

#[test]
fn unsupported_index_version_is_rejected_on_reopen() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "unsupported_index_version");
    let mut collection = store
        .collection::<TestRecord>("unsupported_index_version")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    drop(collection);
    drop(store);

    let path = directory.index_path("unsupported_index_version");
    let mut bytes = fs::read(&path).unwrap();
    bytes[INDEX_VERSION_OFFSET] = u8::MAX;
    fs::write(path, bytes).unwrap();
    let mut reopened = directory
        .connect()
        .collection::<TestRecord>("unsupported_index_version")
        .expect("select collection with a valid name");

    assert_operation_returns_error_without_panicking(|| reopened.list());
}

#[test]
fn reopening_empty_collection_with_missing_index_returns_error() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "empty_missing_index");
    let mut collection = store
        .collection::<TestRecord>("empty_missing_index")
        .expect("select collection with a valid name");
    assert!(collection.list().unwrap().is_empty());
    drop(collection);
    drop(store);
    let store_path = directory.store_path("empty_missing_index");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let index_path = directory.index_path("empty_missing_index");
    fs::remove_file(&index_path).unwrap();
    let reopened_store = directory.connect();
    assert_operation_returns_error_without_panicking(|| {
        reopened_store.collection::<TestRecord>("empty_missing_index")
    });
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert!(
        !index_path.exists(),
        "opening must not silently recreate the missing index"
    );
}

#[test]
fn deletion_with_zero_live_count_returns_error_without_mutation() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "zero_count_delete");
    let mut collection = store
        .collection::<TestRecord>("zero_count_delete")
        .expect("select collection with a valid name");
    collection.insert_one(TestRecord::new("one", 1)).unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    let store_path = directory.store_path("zero_count_delete");
    let index_path = directory.index_path("zero_count_delete");
    let original_index_bytes = fs::read(&index_path).unwrap();
    let mut bytes = fs::read(&store_path).unwrap();
    bytes[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET]
        .copy_from_slice(&0u32.to_be_bytes());
    fs::write(&store_path, &bytes).unwrap();

    assert_operation_returns_error_without_panicking(|| collection.delete_one(1));
    assert_eq!(fs::read(store_path).unwrap(), bytes);
    assert_eq!(fs::read(index_path).unwrap(), original_index_bytes);
}

#[test]
fn large_index_id_returns_error_without_panicking_or_mutating_files() {
    let directory = TestDirectory::new();
    let store = create_collection(&directory, "large_index_id");
    let mut collection = store
        .collection::<TestRecord>("large_index_id")
        .expect("select collection with a valid name");
    assert!(collection.list().unwrap().is_empty());
    let store_path = directory.store_path("large_index_id");
    let original_store_bytes = fs::read(&store_path).unwrap();
    let index_path = directory.index_path("large_index_id");
    let mut bytes = fs::read(&index_path).unwrap();
    // The file remains tiny. Its count must not make (id - 1) * slot_size panic.
    bytes[INDEX_RECORD_COUNT_OFFSET..INDEX_HEADER_TOTAL_BYTES]
        .copy_from_slice(&u32::MAX.to_be_bytes());
    fs::write(&index_path, &bytes).unwrap();

    assert_operation_returns_error_without_panicking(|| collection.delete_one(u32::MAX));
    assert_eq!(fs::read(store_path).unwrap(), original_store_bytes);
    assert_eq!(fs::read(index_path).unwrap(), bytes);
}

// These fixtures keep both files valid until the test introduces one specific
// inconsistency. Opening may reject it early; otherwise the operation must do so.
#[track_caller]
fn assert_collection_rejects_without_changes<R>(
    directory: &TestDirectory,
    name: &str,
    operation: impl FnOnce(&mut Colection<TestRecord>) -> Result<R, StoreError>,
) {
    let store_path = directory.store_path(name);
    let index_path = directory.index_path(name);
    let store_bytes = fs::read(&store_path).unwrap();
    let index_bytes = fs::read(&index_path).unwrap();

    assert_operation_returns_error_without_panicking(|| {
        let mut collection = directory.connect().collection::<TestRecord>(name)?;
        // Do not call list() first: mutations must validate their own input.
        operation(&mut collection)
    });

    assert_eq!(fs::read(store_path).unwrap(), store_bytes, ".store changed");
    assert_eq!(fs::read(index_path).unwrap(), index_bytes, ".idx changed");
}

fn assert_single_record_mutation_rejects_corruption(
    corrupt: impl FnOnce(&mut Vec<u8>, &mut Vec<u8>),
    operation: impl FnOnce(&mut Colection<TestRecord>) -> Result<(), StoreError>,
) {
    let directory = TestDirectory::new();
    let name = "corrupt_mutation";
    let store = create_collection(&directory, name);
    let mut collection = store.collection::<TestRecord>(name).unwrap();
    collection
        .insert_one(TestRecord::new("original", 1))
        .unwrap();
    assert_eq!(collection.list().unwrap().len(), 1);
    drop(collection);
    drop(store);

    let mut store_bytes = fs::read(directory.store_path(name)).unwrap();
    let mut index_bytes = fs::read(directory.index_path(name)).unwrap();
    corrupt(&mut store_bytes, &mut index_bytes);
    fs::write(directory.store_path(name), store_bytes).unwrap();
    fs::write(directory.index_path(name), index_bytes).unwrap();

    assert_collection_rejects_without_changes(&directory, name, operation);
}

fn assert_empty_collection_rejects_corrupt_index(corrupt: impl FnOnce(&mut Vec<u8>)) {
    let directory = TestDirectory::new();
    let name = "empty_corrupt_index";
    let store = create_collection(&directory, name);
    let mut collection = store.collection::<TestRecord>(name).unwrap();
    assert!(collection.list().unwrap().is_empty());
    drop(collection);
    drop(store);

    let path = directory.index_path(name);
    let mut bytes = fs::read(&path).unwrap();
    corrupt(&mut bytes);
    fs::write(path, bytes).unwrap();

    assert_collection_rejects_without_changes(&directory, name, |collection| collection.list());
}

#[test]
fn empty_collection_with_corrupt_index_magic_is_rejected_on_reopen() {
    assert_empty_collection_rejects_corrupt_index(|bytes| bytes[0] ^= 1);
}

#[test]
fn empty_collection_with_unsupported_index_version_is_rejected_on_reopen() {
    assert_empty_collection_rejects_corrupt_index(|bytes| bytes[INDEX_VERSION_OFFSET] = u8::MAX);
}

#[test]
fn empty_collection_with_truncated_index_header_is_rejected_on_reopen() {
    // Cover every incomplete header, including a missing/partial count field
    // after otherwise valid magic and version bytes.
    for length in 0..INDEX_HEADER_TOTAL_BYTES {
        assert_empty_collection_rejects_corrupt_index(|bytes| bytes.truncate(length));
    }
}

#[test]
fn empty_collection_with_index_count_without_entries_is_rejected_on_reopen() {
    assert_empty_collection_rejects_corrupt_index(|bytes| {
        bytes[INDEX_RECORD_COUNT_OFFSET..INDEX_HEADER_TOTAL_BYTES]
            .copy_from_slice(&1u32.to_be_bytes());
    });
}

fn assert_index_only_insertion_is_rejected(existing_records: u32) {
    let directory = TestDirectory::new();
    let name = "index_only_insert";
    let store = create_collection(&directory, name);
    let mut collection = store.collection::<TestRecord>(name).unwrap();
    for number in 0..existing_records {
        collection
            .insert_one(TestRecord::new("existing", number))
            .unwrap();
    }
    assert_eq!(collection.list().unwrap().len(), existing_records as usize);
    drop(collection);
    drop(store);

    let path = directory.index_path(name);
    let mut bytes = fs::read(&path).unwrap();
    let id = existing_records + 1;
    let frame_offset = fs::metadata(directory.store_path(name)).unwrap().len();
    // Stop immediately after insert_index(): the index count/slot are written,
    // but the store header and the future frame at EOF have not been written.
    bytes[INDEX_RECORD_COUNT_OFFSET..INDEX_HEADER_TOTAL_BYTES].copy_from_slice(&id.to_be_bytes());
    bytes.extend_from_slice(&id.to_be_bytes());
    bytes.extend_from_slice(&frame_offset.to_be_bytes());
    fs::write(path, bytes).unwrap();

    assert_collection_rejects_without_changes(&directory, name, |collection| collection.list());
}

#[test]
fn interrupted_insert_after_index_write_into_empty_collection_is_rejected_on_reopen() {
    assert_index_only_insertion_is_rejected(0);
}

#[test]
fn interrupted_insert_after_index_write_into_nonempty_collection_is_rejected_on_reopen() {
    assert_index_only_insertion_is_rejected(1);
}

#[test]
fn insertion_rejects_corrupt_storage_magic_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[0] ^= 1,
        |collection| collection.insert_one(TestRecord::new("must not append", 2)),
    );
}

#[test]
fn deletion_rejects_corrupt_storage_magic_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[0] ^= 1,
        |collection| collection.delete_one(1),
    );
}

#[test]
fn update_rejects_corrupt_storage_magic_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[0] ^= 1,
        |collection| collection.update_one(1, TestRecord::new("must not replace", 2)),
    );
}

#[test]
fn insertion_rejects_unsupported_storage_version_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[STORAGE_VERSION_OFFSET] = u8::MAX,
        |collection| collection.insert_one(TestRecord::new("must not append", 2)),
    );
}

#[test]
fn deletion_rejects_unsupported_storage_version_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[STORAGE_VERSION_OFFSET] = u8::MAX,
        |collection| collection.delete_one(1),
    );
}

#[test]
fn update_rejects_unsupported_storage_version_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[STORAGE_VERSION_OFFSET] = u8::MAX,
        |collection| collection.update_one(1, TestRecord::new("must not replace", 2)),
    );
}

#[test]
fn deletion_rejects_invalid_frame_flag_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[STORAGE_HEADER_TOTAL_BYTES] = 2,
        |collection| collection.delete_one(1),
    );
}

#[test]
fn update_rejects_invalid_frame_flag_without_mutation() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| store[STORAGE_HEADER_TOTAL_BYTES] = 2,
        |collection| collection.update_one(1, TestRecord::new("must not replace", 2)),
    );
}

fn assert_mutation_rejects_index_pointing_to_tombstone(
    operation: impl FnOnce(&mut Colection<TestRecord>) -> Result<(), StoreError>,
) {
    let directory = TestDirectory::new();
    let name = "stale_tombstone_offset";
    let store = create_collection(&directory, name);
    let mut collection = store.collection::<TestRecord>(name).unwrap();
    collection.insert_one(TestRecord::new("old", 1)).unwrap();
    collection
        .update_one(1, TestRecord::new("current", 2))
        .unwrap();
    assert_eq!(
        collection.list().unwrap(),
        vec![TestRecord {
            id: 1,
            name: "current".into(),
            number: 2,
        }]
    );
    drop(collection);
    drop(store);

    let path = directory.index_path(name);
    let mut bytes = fs::read(&path).unwrap();
    // Both old and current payloads contain ID 1. Only the frame state reveals
    // that this offset points to the obsolete version; an ID check is not enough.
    bytes[INDEX_HEADER_TOTAL_BYTES + 4..INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN]
        .copy_from_slice(&(STORAGE_HEADER_TOTAL_BYTES as u64).to_be_bytes());
    fs::write(path, bytes).unwrap();

    assert_collection_rejects_without_changes(&directory, name, operation);
}

#[test]
fn deletion_rejects_index_pointing_to_tombstoned_version_without_mutation() {
    assert_mutation_rejects_index_pointing_to_tombstone(|collection| collection.delete_one(1));
}

#[test]
fn update_rejects_index_pointing_to_tombstoned_version_without_mutation() {
    assert_mutation_rejects_index_pointing_to_tombstone(|collection| {
        collection.update_one(1, TestRecord::new("must not replace", 3))
    });
}

#[test]
fn deletion_rejects_maximum_index_offset_without_panicking_or_mutating_files() {
    assert_single_record_mutation_rejects_corruption(
        |_store, index| {
            index[INDEX_HEADER_TOTAL_BYTES + 4..INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN]
                .copy_from_slice(&u64::MAX.to_be_bytes());
        },
        |collection| collection.delete_one(1),
    );
}

#[test]
fn update_rejects_maximum_index_offset_without_panicking_or_mutating_files() {
    assert_single_record_mutation_rejects_corruption(
        |_store, index| {
            index[INDEX_HEADER_TOTAL_BYTES + 4..INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN]
                .copy_from_slice(&u64::MAX.to_be_bytes());
        },
        |collection| collection.update_one(1, TestRecord::new("must not replace", 2)),
    );
}

#[test]
fn deletion_rejects_dead_bytes_overflow_without_panicking_or_mutating_files() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| {
            store[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .copy_from_slice(&u64::MAX.to_be_bytes());
        },
        |collection| collection.delete_one(1),
    );
}

#[test]
fn update_rejects_dead_bytes_overflow_without_panicking_or_mutating_files() {
    assert_single_record_mutation_rejects_corruption(
        |store, _index| {
            store[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .copy_from_slice(&u64::MAX.to_be_bytes());
        },
        |collection| collection.update_one(1, TestRecord::new("must not replace", 2)),
    );
}

fn assert_large_index_mutation_does_not_wrap(
    operation: impl FnOnce(&mut Colection<TestRecord>, u32) -> Result<(), StoreError>,
    expected_success: impl FnOnce(u32, &mut Vec<u8>, &mut Vec<u8>),
) {
    let directory = TestDirectory::new();
    let name = "large_sparse_index";
    let store = create_collection(&directory, name);
    drop(store);

    // This slot exists beyond u32::MAX, so lookup can reach the mutation helper.
    // Unlike the earlier tiny-file test, it cannot fail merely on reading EOF.
    let id = 357_913_942u32;
    let mut store_bytes = storage_header(id + 1, 1, 0);
    append_frame(&mut store_bytes, &encoded_payload(id, "original", 1));
    write_store_bytes(&directory, name, &store_bytes);
    let index_path = directory.index_path(name);
    let mut index_header = fs::read(&index_path).unwrap();
    index_header[INDEX_RECORD_COUNT_OFFSET..INDEX_HEADER_TOTAL_BYTES]
        .copy_from_slice(&id.to_be_bytes());
    let slot_position =
        INDEX_HEADER_TOTAL_BYTES as u64 + (u64::from(id) - 1) * INDEX_RECORD_LEN as u64;
    assert!(slot_position > u64::from(u32::MAX));
    let mut slot = Vec::with_capacity(INDEX_RECORD_LEN);
    slot.extend_from_slice(&id.to_be_bytes());
    slot.extend_from_slice(&(STORAGE_HEADER_TOTAL_BYTES as u64).to_be_bytes());

    // Seeking creates a sparse hole: only the header and last slot are written.
    // Intervening IDs are deliberately unpopulated, so full validation may
    // reject this fixture. If a targeted mutation succeeds, it must modify the
    // exact high slot and frame, without touching the prefix. Never read the
    // whole file: this test isolates position arithmetic, not a full index scan.
    let mut file = OpenOptions::new().write(true).open(&index_path).unwrap();
    file.write_all(&index_header).unwrap();
    file.seek(SeekFrom::Start(slot_position)).unwrap();
    file.write_all(&slot).unwrap();
    drop(file);
    let index_length = slot_position + INDEX_RECORD_LEN as u64;

    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut collection = directory.connect().collection::<TestRecord>(name)?;
        operation(&mut collection, id)
    }));
    assert!(outcome.is_ok(), "large index positions must not panic");
    if outcome.unwrap().is_ok() {
        expected_success(id, &mut store_bytes, &mut slot);
    }

    // An error must preserve the original bytes; success must produce exactly
    // the requested mutation. Both branches reject wrapped writes near byte 0.
    assert_eq!(fs::read(directory.store_path(name)).unwrap(), store_bytes);
    let mut file = fs::File::open(index_path).unwrap();
    assert_eq!(file.metadata().unwrap().len(), index_length);
    // Include the first empty slot: a wrapped u32 position writes near here.
    let mut prefix = vec![0; INDEX_HEADER_TOTAL_BYTES + INDEX_RECORD_LEN];
    file.read_exact(&mut prefix).unwrap();
    index_header.resize(prefix.len(), 0);
    assert_eq!(prefix, index_header);
    let mut actual_slot = [0; INDEX_RECORD_LEN];
    file.seek(SeekFrom::Start(slot_position)).unwrap();
    file.read_exact(&mut actual_slot).unwrap();
    assert_eq!(actual_slot.as_slice(), slot.as_slice());
}

#[test]
fn deletion_with_large_present_index_id_does_not_wrap_or_panic() {
    assert_large_index_mutation_does_not_wrap(
        |collection, id| collection.delete_one(id),
        |_id, store, slot| {
            let dead_bytes = (store.len() - STORAGE_HEADER_TOTAL_BYTES) as u64;
            store[STORAGE_HEADER_TOTAL_BYTES] = STORAGE_PAYLOAD_FRAME_OFF[0];
            store[STORAGE_RECORD_COUNT_OFFSET..STORAGE_DEAD_BYTES_OFFSET]
                .copy_from_slice(&0u32.to_be_bytes());
            store[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .copy_from_slice(&dead_bytes.to_be_bytes());
            slot[4..].copy_from_slice(&0u64.to_be_bytes());
        },
    );
}

#[test]
fn update_with_large_present_index_id_does_not_wrap_or_panic() {
    assert_large_index_mutation_does_not_wrap(
        |collection, id| collection.update_one(id, TestRecord::new("replacement", 2)),
        |id, store, slot| {
            let replacement_offset = store.len() as u64;
            let dead_bytes = (store.len() - STORAGE_HEADER_TOTAL_BYTES) as u64;
            store[STORAGE_HEADER_TOTAL_BYTES] = STORAGE_PAYLOAD_FRAME_OFF[0];
            store[STORAGE_DEAD_BYTES_OFFSET..STORAGE_HEADER_TOTAL_BYTES]
                .copy_from_slice(&dead_bytes.to_be_bytes());
            append_frame(store, &encoded_payload(id, "replacement", 2));
            slot[4..].copy_from_slice(&replacement_offset.to_be_bytes());
        },
    );
}
