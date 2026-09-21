use std::{fmt::Debug, time::Duration};

use life::http::{
    cookie::{Cookie, SameSite, parse_cookie},
    request::Request,
    response::{Response, StatusCode},
};

// Keep these regression tests runnable while the infallible APIs gain an error path.
// Plain returns mean success; Result returns preserve the implementation's errors.
// Remove this adapter once the public error-handling API is settled.
trait TestOutcome {
    type Value;
    fn outcome(self) -> Result<Self::Value, String>;
}

impl TestOutcome for String {
    type Value = String;
    fn outcome(self) -> Result<String, String> {
        Ok(self)
    }
}

impl TestOutcome for Vec<(&str, &str)> {
    type Value = Self;
    fn outcome(self) -> Result<Self, String> {
        Ok(self)
    }
}

impl<T, E: Debug> TestOutcome for Result<T, E> {
    type Value = T;
    fn outcome(self) -> Result<T, String> {
        self.map_err(|error| format!("{error:?}"))
    }
}

fn cookie(name: &str, value: &str) -> Cookie {
    let mut cookie = Cookie::new();
    cookie.set_name_value(name, value).unwrap();
    cookie
}

fn append_cookie(response: &mut Response, cookie: &Cookie) -> Result<(), String> {
    let value = cookie.build().outcome()?;
    response.add_header("Set-Cookie", &value).outcome()
}

fn assert_name_value_rejected_without_mutation(name: &str, value: &str) {
    let mut cookie = cookie("existing", "keep");
    let before = cookie.build().outcome().unwrap();
    let result = cookie.set_name_value(name, value);
    assert!(result.is_err(), "invalid name/value must return an error");
    assert_eq!(
        cookie.build().outcome().unwrap(),
        before,
        "rejected name/value changed the existing cookie"
    );
}

fn assert_path_rejected_without_mutation(path: &str) {
    let mut cookie = cookie("theme", "dark");
    cookie.set_path("/account".to_owned()).unwrap();
    let before = cookie.build().outcome().unwrap();
    let result = cookie.set_path(path.to_owned());
    assert!(result.is_err(), "invalid path must return an error");
    assert_eq!(
        cookie.build().outcome().unwrap(),
        before,
        "rejected path changed the existing cookie"
    );
}

fn assert_domain_rejected_without_mutation(domain: &str) {
    let mut cookie = cookie("theme", "dark");
    cookie.set_domain("example.com").unwrap();
    let before = cookie.build().outcome().unwrap();
    let result = cookie.set_domain(domain);
    assert!(result.is_err(), "invalid domain must return an error");
    assert_eq!(
        cookie.build().outcome().unwrap(),
        before,
        "rejected domain changed the existing cookie"
    );
}

#[test]
fn rejects_attribute_injection_in_cookie_value() {
    assert_name_value_rejected_without_mutation("theme", "dark; Max-Age=0");
}

#[test]
fn rejects_invalid_cookie_names() {
    for name in ["", "bad name", "bad=name", "bad;name", "bad\tname"] {
        assert_name_value_rejected_without_mutation(name, "dark");
    }
}

#[test]
fn rejects_invalid_cookie_paths() {
    for path in ["", "account", "/; Max-Age=0"] {
        assert_path_rejected_without_mutation(path);
    }
}

#[test]
fn rejects_attribute_injection_in_domain() {
    assert_domain_rejected_without_mutation("example.com; Max-Age=0");
}

#[test]
fn rejects_cr_lf_in_every_outgoing_text_field_without_mutation() {
    for control in ["\r", "\n", "\r\n"] {
        let injected = format!("safe{control}X-Injected: yes");
        assert_name_value_rejected_without_mutation(&injected, "dark");
        assert_name_value_rejected_without_mutation("theme", &injected);
        assert_path_rejected_without_mutation(&format!("/{injected}"));
        assert_domain_rejected_without_mutation(&injected);
    }
}

#[test]
fn set_name_value_accepts_empty_values_and_permitted_punctuation() {
    let mut cookie = cookie("existing", "keep");
    for (name, value) in [("empty", ""), ("__Host-session", "abc==/+:%!")] {
        cookie.set_name_value(name, value).unwrap();
        let wire = cookie.build().outcome().unwrap();
        assert_eq!(wire.split(';').next().unwrap(), format!("{name}={value}"));
    }
}

#[test]
fn rejects_incoming_pair_without_equals() {
    assert!(parse_cookie("theme=dark; broken").outcome().is_err());
}

#[test]
fn rejects_incoming_empty_name() {
    assert!(parse_cookie("=dark").outcome().is_err());
}

#[test]
fn accepts_spaces_and_tabs_around_incoming_cookie_names_and_values() {
    let input = " \ttheme \t= \tdark\t ;\t empty= \t";
    let expected = vec![("theme", "dark"), ("empty", "")];
    assert_eq!(parse_cookie(input).outcome().unwrap(), expected);

    let raw = format!("GET / HTTP/1.1\r\nHost: localhost\r\nCookie:{input}\r\n\r\n");
    let request = Request::parse(raw.as_bytes()).unwrap();
    assert_eq!(request.get_cookie_value("theme"), Some("dark".to_owned()));
    assert_eq!(request.get_cookie_value("empty"), Some(String::new()));
}

#[test]
fn rejects_unsupported_whitespace_at_cookie_name_and_value_edges() {
    // These characters must reach validation, not disappear during trimming.
    for whitespace in [
        "\r", "\n", "\r\n", "\u{000b}", "\u{000c}", "\u{0085}", "\u{00a0}", "\u{2003}",
    ] {
        for input in [
            format!("{whitespace}theme=dark"),
            format!("theme{whitespace}=dark"),
            format!("theme={whitespace}dark"),
            format!("theme=dark{whitespace}"),
        ] {
            assert!(
                parse_cookie(&input).outcome().is_err(),
                "accepted {input:?}"
            );
        }
    }
}

#[test]
fn request_header_trimming_does_not_hide_invalid_cookie_whitespace() {
    // CR/LF delimit HTTP lines, so test other whitespace inside a single line.
    for whitespace in ["\u{000b}", "\u{000c}", "\u{0085}", "\u{00a0}", "\u{2003}"] {
        for value in [
            format!("{whitespace}theme=dark"),
            format!("theme=dark{whitespace}"),
        ] {
            let raw = format!("GET / HTTP/1.1\r\nHost: localhost\r\nCookie: {value}\r\n\r\n");
            // Either the header parser or cookie validation may reject it.
            if let Ok(request) = Request::parse(raw.as_bytes()) {
                assert!(
                    request.get_cookie_value("theme").is_none(),
                    "request accepted invalid cookie value {value:?}"
                );
            }
        }
    }
}

#[test]
fn preserves_duplicate_incoming_names_even_when_values_match() {
    for second_value in ["light", "dark"] {
        let input = format!("theme=dark; theme={second_value}");
        let parsed = parse_cookie(&input).outcome().unwrap();
        assert_eq!(parsed, vec![("theme", "dark"), ("theme", second_value)]);
    }
}

#[test]
fn request_cookie_access_returns_first_duplicate_value() {
    let raw =
        b"GET /account HTTP/1.1\r\nHost: localhost\r\nCookie: theme=light; theme=dark\r\n\r\n";
    let request = Request::parse(raw).unwrap();
    assert_eq!(request.get_cookie_value("theme"), Some("light".to_owned()));
}

#[test]
fn rejects_unsupported_incoming_name_characters() {
    for input in ["bad name=x", "bad\tname=x", "thème=x"] {
        assert!(parse_cookie(input).outcome().is_err(), "accepted {input:?}");
    }
}

#[test]
fn request_cookie_access_returns_none_for_malformed_input() {
    for input in [
        "theme=dark; broken",
        "broken; theme=dark",
        "theme=dark; =empty",
        "theme=dark;",
        "theme=dark;; other=value",
        "",
    ] {
        let raw = format!("GET / HTTP/1.1\r\nHost: localhost\r\nCookie: {input}\r\n\r\n");
        let request = Request::parse(raw.as_bytes()).unwrap();
        assert_eq!(
            request.get_cookie_value("theme"),
            None,
            "accepted {input:?}"
        );
    }
}

#[test]
fn missing_cookie_header_returns_none() {
    let request = Request::parse(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
    assert_eq!(request.get_cookie_value("theme"), None);
}

#[test]
fn request_cookie_access_matches_names_exactly_and_preserves_literal_values() {
    let raw = b"GET / HTTP/1.1\r\nHost: localhost\r\nCookie: theme=dark; Theme=light; token=abc==+%2F; empty=\r\n\r\n";
    let request = Request::parse(raw).unwrap();
    for (key, expected) in [
        ("theme", Some("dark")),
        ("Theme", Some("light")),
        ("token", Some("abc==+%2F")),
        ("empty", Some("")),
        ("missing", None),
        ("THEME", None),
        ("the", None),
        ("", None),
    ] {
        assert_eq!(
            request.get_cookie_value(key).as_deref(),
            expected,
            "key {key:?}"
        );
    }
}

#[test]
fn request_cookie_access_uses_last_cookie_header_case_insensitively() {
    let raw = b"GET / HTTP/1.1\r\nHost: localhost\r\nCookie: theme=light; old=value\r\ncOoKiE: theme=dark\r\n\r\n";
    let request = Request::parse(raw).unwrap();
    assert_eq!(request.get_cookie_value("theme"), Some("dark".to_owned()));
    assert_eq!(request.get_cookie_value("old"), None);
}

#[test]
fn separate_cookies_serialize_as_separate_headers_with_default_attributes() {
    let mut response = Response::new(StatusCode::Ok, vec![]);
    append_cookie(&mut response, &cookie("theme", "dark")).unwrap();
    append_cookie(&mut response, &cookie("language", "en")).unwrap();
    let wire = String::from_utf8(response.to_bytes()).unwrap();
    let headers: Vec<_> = wire
        .lines()
        .filter_map(|line| line.strip_prefix("Set-Cookie: "))
        .collect();
    assert_eq!(headers.len(), 2);
    for (header, pair) in headers.iter().zip(["theme=dark", "language=en"]) {
        let fields: Vec<_> = header.split(';').map(str::trim).collect();
        assert_eq!(fields, [pair, "Path=/", "HttpOnly", "SameSite=Lax"]);
    }
}

#[test]
fn max_age_distinguishes_unset_zero_and_seconds_and_preserves_scope() {
    let mut cookie = cookie("theme", "dark");
    cookie.set_path("/account".to_owned()).unwrap();
    cookie.set_domain("example.com").unwrap();
    assert!(!cookie.build().outcome().unwrap().contains("Max-Age="));
    for (duration, expected) in [
        (Duration::ZERO, "Max-Age=0"),
        (Duration::from_secs(3600), "Max-Age=3600"),
    ] {
        cookie.set_max_age(duration);
        let wire = cookie.build().outcome().unwrap();
        let fields: Vec<_> = wire.split(';').map(str::trim).collect();
        assert!(fields.contains(&expected));
        assert!(fields.contains(&"theme=dark"));
        assert!(fields.contains(&"Path=/account"));
        assert!(fields.contains(&"Domain=example.com"));
    }
}

#[test]
fn samesite_none_preserves_callers_secure_choice() {
    for secure in [false, true] {
        let mut cookie = cookie("theme", "dark");
        cookie.samesite(SameSite::None);
        cookie.set_secure(secure);
        let mut response = Response::new(StatusCode::Ok, vec![]);
        append_cookie(&mut response, &cookie).unwrap();
        let wire = cookie.build().outcome().unwrap();
        let fields: Vec<_> = wire.split(';').map(str::trim).collect();
        assert!(fields.contains(&"SameSite=None"));
        assert_eq!(fields.contains(&"Secure"), secure);
        assert!(!fields.iter().any(|field| field.starts_with("Secure=")));
    }
}
