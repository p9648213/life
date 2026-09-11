mod support;

use std::{
    fs::{self, File},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use life::{
    constant::{MAX_ASSET_SIZE, MAX_REQUEST_BYTES},
    http::{request::Request, response::Response, router::Router, static_files::serve_static},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    base: PathBuf,
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let base = std::env::temp_dir().join(format!("life-phase-10-{}-{id}", std::process::id()));
        fs::create_dir(&base).unwrap();
        let root = base.join("static");
        fs::create_dir(&root).unwrap();
        Self { base, root }
    }

    fn write(&self, path: &str, bytes: &[u8]) {
        let path = self.root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn router(&self) -> Router<()> {
        let mut router = Router::new();
        router.static_files("/static/", self.root.to_str().unwrap());
        router
    }

    fn get(&self, target: &str) -> Vec<u8> {
        dispatch(&self.router(), "GET", target)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

fn dispatch(router: &Router<()>, method: &str, target: &str) -> Vec<u8> {
    let raw = format!("{method} {target} HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n");
    let request = Request::parse(raw.as_bytes()).expect("parse static request");
    router.handle_request(&request, &mut ()).to_bytes()
}

// Decode only the headers: the body deliberately may contain invalid UTF-8.
fn parts(wire: &[u8]) -> (&str, &[u8]) {
    let end = wire
        .windows(4)
        .position(|bytes| bytes == b"\r\n\r\n")
        .unwrap();
    (str::from_utf8(&wire[..end]).unwrap(), &wire[end + 4..])
}

fn header<'a>(headers: &'a str, name: &str) -> &'a str {
    let values: Vec<_> = headers
        .lines()
        .skip(1)
        .filter_map(|line| {
            let (key, value) = line.split_once(':')?;
            key.eq_ignore_ascii_case(name).then_some(value.trim())
        })
        .collect();
    assert_eq!(values.len(), 1, "expected one {name}: {headers}");
    values[0]
}

fn assert_status(wire: &[u8], expected: &str) {
    let (headers, body) = parts(wire);
    assert_eq!(
        headers.lines().next().unwrap(),
        format!("HTTP/1.1 {expected}")
    );
    assert_eq!(header(headers, "Content-Length"), body.len().to_string());
}

fn assert_asset(wire: &[u8], content_type: &str, expected: &[u8]) {
    assert_status(wire, "200 OK");
    let (headers, body) = parts(wire);
    assert_eq!(header(headers, "Content-Type"), content_type);
    assert_eq!(body, expected);
}

#[test]
fn css_javascript_images_and_fonts_have_exact_bytes_and_metadata() {
    let fixture = Fixture::new();
    for (name, mime, bytes) in [
        (
            "app.css",
            "text/css; charset=utf-8",
            "/* tiếng Việt */\nbody {}".as_bytes(),
        ),
        (
            "app.js",
            "text/javascript; charset=utf-8",
            b"const template = '{{ value }}';".as_slice(),
        ),
        (
            "pixel.png",
            "image/png",
            b"\x89PNG\r\n\x1a\n\0\xff".as_slice(),
        ),
        ("photo.jpg", "image/jpeg", b"\xff\xd8\0\xff\xd9".as_slice()),
        ("photo.jpeg", "image/jpeg", b"\xff\xd8\0\xff\xd9".as_slice()),
        ("icon.svg", "image/svg+xml", b"<svg></svg>".as_slice()),
        ("font.woff2", "font/woff2", b"wOF2\0\xff\x80".as_slice()),
    ] {
        fixture.write(name, bytes);
        assert_asset(&fixture.get(&format!("/static/{name}")), mime, bytes);
    }
}

#[test]
fn content_type_extensions_are_case_insensitive() {
    let fixture = Fixture::new();
    fixture.write("APP.CsS", b"body {}");
    assert_asset(
        &fixture.get("/static/APP.CsS"),
        "text/css; charset=utf-8",
        b"body {}",
    );
}

#[test]
fn unknown_and_absent_extensions_preserve_every_byte_value() {
    let fixture = Fixture::new();
    let bytes: Vec<u8> = (0..=255).collect();
    for name in ["data.custom", "LICENSE"] {
        fixture.write(name, &bytes);
        assert_asset(
            &fixture.get(&format!("/static/{name}")),
            "application/octet-stream",
            &bytes,
        );
    }
}

#[test]
fn nested_assets_and_queries_use_the_same_file() {
    let fixture = Fixture::new();
    fixture.write("css/components/card.css", b".card {}");
    let plain = fixture.get("/static/css/components/card.css");
    let query = fixture.get("/static/css/components/card.css?v=1&path=../../secret");
    assert_asset(&plain, "text/css; charset=utf-8", b".card {}");
    assert_eq!(query, plain);
}

#[test]
fn files_added_after_mounting_need_no_new_route() {
    let fixture = Fixture::new();
    let router = fixture.router();
    assert_status(
        &dispatch(&router, "GET", "/static/new.css"),
        "404 Not Found",
    );
    fixture.write("new.css", b"new {}");
    assert_asset(
        &dispatch(&router, "GET", "/static/new.css"),
        "text/css; charset=utf-8",
        b"new {}",
    );
}

#[test]
fn static_root_directory_returns_404() {
    let fixture = Fixture::new();
    fixture.write("index.html", b"must not be served automatically");
    assert_status(&fixture.get("/static/"), "404 Not Found");
}

#[test]
fn path_beneath_a_regular_file_returns_404() {
    let fixture = Fixture::new();
    fixture.write("app.css", b"body {}");
    assert_status(&fixture.get("/static/app.css/child.css"), "404 Not Found");
}

#[test]
fn missing_files_and_directories_return_404_even_with_an_index() {
    let fixture = Fixture::new();
    fixture.write("folder/index.html", b"must not be served automatically");
    for path in [
        "/static/missing.css",
        "/static/missing/file.css",
        "/static/folder",
        "/static/folder/",
    ] {
        assert_status(&fixture.get(path), "404 Not Found");
    }
}

#[test]
fn post_does_not_serve_an_existing_asset() {
    let fixture = Fixture::new();
    fixture.write("private.css", b"asset contents");
    let wire = dispatch(&fixture.router(), "POST", "/static/private.css");
    assert_status(&wire, "405 Method Not Allowed");
    assert!(parts(&wire).1.is_empty());
}

#[test]
fn other_methods_are_rejected_before_routing() {
    for method in ["HEAD", "PUT", "DELETE", "PATCH", "OPTIONS"] {
        let raw = format!("{method} /static/app.css HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(Request::parse(raw.as_bytes()).is_err(), "{method}");
    }
}

#[test]
fn static_mount_does_not_capture_similarly_named_paths() {
    let fixture = Fixture::new();
    fixture.write("app.css", b"body {}");
    for path in ["/staticish/app.css", "/static.css", "/app.css"] {
        assert_status(&fixture.get(path), "404 Not Found");
    }
}

#[test]
fn traversal_and_platform_escape_forms_are_rejected() {
    let fixture = Fixture::new();
    fs::write(fixture.base.join("secret"), b"outside secret").unwrap();
    for part in [
        "../secret",
        "sub/../../secret",
        "./secret",
        "sub/../secret",
        "/etc/passwd",
        "//server/share",
        "..\\secret",
        "C:/secret",
        "C:secret",
        "\\\\server\\share",
        "secret\0.css",
    ] {
        let wire = serve_static(&fixture.root, part).to_bytes();
        assert_status(&wire, "400 Bad Request");
        assert_ne!(parts(&wire).1, b"outside secret", "{part:?}");
    }
}

#[test]
fn encoded_and_malformed_paths_follow_the_reject_all_percent_policy() {
    let fixture = Fixture::new();
    fs::write(fixture.base.join("secret"), b"outside secret").unwrap();
    fixture.write("app.css", b"body {}");
    for part in [
        "%2e%2e/secret",
        "%2E%2E%2Fsecret",
        "%252e%252e%252fsecret",
        "%2fetc/passwd",
        "..%5csecret",
        "app.css%00",
        "%",
        "%2",
        "%GG",
        "%61pp.css",
    ] {
        assert_status(&fixture.get(&format!("/static/{part}")), "400 Bad Request");
    }
}

#[test]
fn harmless_dots_in_filenames_are_allowed() {
    let fixture = Fixture::new();
    fixture.write("app..min.css", b"body {}");
    assert_asset(
        &fixture.get("/static/app..min.css"),
        "text/css; charset=utf-8",
        b"body {}",
    );
}

#[cfg(unix)]
#[test]
fn file_and_directory_symlinks_cannot_escape_to_a_sibling_with_the_same_prefix() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    let outside = fixture.base.join("static-private");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("secret.css"), b"outside secret").unwrap();
    symlink(outside.join("secret.css"), fixture.root.join("link.css")).unwrap();
    symlink(&outside, fixture.root.join("linked-dir")).unwrap();
    for path in ["/static/link.css", "/static/linked-dir/secret.css"] {
        let wire = fixture.get(path);
        assert_status(&wire, "404 Not Found");
        assert_ne!(parts(&wire).1, b"outside secret");
    }
}

#[cfg(unix)]
#[test]
fn symlink_to_an_asset_inside_the_root_is_allowed_and_broken_link_is_404() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    fixture.write("nested/app.css", b"body {}");
    symlink("nested/app.css", fixture.root.join("alias.css")).unwrap();
    symlink("missing.css", fixture.root.join("broken.css")).unwrap();
    assert_asset(
        &fixture.get("/static/alias.css"),
        "text/css; charset=utf-8",
        b"body {}",
    );
    assert_status(&fixture.get("/static/broken.css"), "404 Not Found");
}

#[test]
fn empty_file_has_zero_content_length() {
    let fixture = Fixture::new();
    fixture.write("empty.css", b"");
    assert_asset(
        &fixture.get("/static/empty.css"),
        "text/css; charset=utf-8",
        b"",
    );
}

#[test]
fn file_exactly_at_the_limit_is_served() {
    let fixture = Fixture::new();
    let bytes: Vec<u8> = (0..MAX_ASSET_SIZE).map(|i| (i % 256) as u8).collect();
    fixture.write("limit.bin", &bytes);
    assert_asset(
        &fixture.get("/static/limit.bin"),
        "application/octet-stream",
        &bytes,
    );
}

#[test]
fn one_byte_over_the_limit_returns_500_without_partial_file_contents() {
    let fixture = Fixture::new();
    fixture.write("large.bin", &vec![0xff; MAX_ASSET_SIZE + 1]);
    let wire = fixture.get("/static/large.bin");
    // Current error policy: TooLarge maps to 500 with this text body.
    assert_status(&wire, "500 Internal Server Error");
    assert_eq!(parts(&wire).1, b"Limit exceed");
}

#[test]
fn very_large_sparse_file_does_not_cause_a_file_sized_allocation() {
    let fixture = Fixture::new();
    let file = File::create(fixture.root.join("huge.bin")).unwrap();
    file.set_len((MAX_ASSET_SIZE as u64) * 1024).unwrap();
    let (response, largest) =
        support::measure_largest_allocation(|| serve_static(&fixture.root, "huge.bin"));
    assert_status(&response.to_bytes(), "500 Internal Server Error");
    // Allow Vec growth slack; fail if a read buffers the whole oversized file.
    assert!(
        largest <= 4 * (MAX_ASSET_SIZE + 1),
        "largest allocation: {largest}"
    );
}

#[test]
fn missing_static_root_maps_io_failure_to_500() {
    let fixture = Fixture::new();
    let response: Response = serve_static(&fixture.base.join("missing-root"), "app.css");
    let wire = response.to_bytes();
    assert_status(&wire, "500 Internal Server Error");
    assert_eq!(parts(&wire).1, b"Internal Server Error");
}

#[test]
fn request_limit_sized_path_with_late_traversal_has_bounded_allocation() {
    let fixture = Fixture::new();
    let prefix = "GET /static/";
    let suffix = " HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let path_len = MAX_REQUEST_BYTES - prefix.len() - suffix.len();
    let mut path = "a/".repeat((path_len - 3) / 2);
    path.push_str(&"a".repeat(path_len - 3 - path.len()));
    path.push_str("/..");
    let raw = format!("{prefix}{path}{suffix}");
    assert_eq!(raw.len(), MAX_REQUEST_BYTES);
    let request = Request::parse(raw.as_bytes()).unwrap();
    let router = fixture.router();
    let (response, largest) =
        support::measure_largest_allocation(|| router.handle_request(&request, &mut ()));
    assert_status(&response.to_bytes(), "400 Bad Request");
    // Invalid paths should be checked without building a segment Vec or path copy.
    assert!(largest <= 4096, "largest allocation: {largest}");
}
