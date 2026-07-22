use life::http::{error::HttpError, request::Request};

fn form_request(body: &[u8], content_type: &str) -> Vec<u8> {
    let mut request = format!(
        "POST /demo/form HTTP/1.1\r\n\
         Host: localhost\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         \r\n",
        body.len()
    )
    .into_bytes();
    request.extend_from_slice(body);
    request
}

fn parse_ok(data: &[u8]) -> Request<'_> {
    Request::parse(data).expect("request should parse")
}

#[test]
fn extracts_two_normal_form_fields_in_requested_order() {
    let bytes = form_request(
        b"name=Rust&message=Learning",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["message", "name"]).unwrap();

    assert_eq!(values, ["Learning", "Rust"]);
}

#[test]
fn splits_form_value_on_the_first_equals_only() {
    let bytes = form_request(
        b"token=left=middle=right",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["token"]).unwrap();

    assert_eq!(values, ["left=middle=right"]);
}

#[test]
fn rejects_form_pair_without_equals() {
    let bytes = form_request(b"name=Rust&message", "application/x-www-form-urlencoded");
    let request = parse_ok(&bytes);

    assert!(request.extract_form(["name", "message"]).is_err());
}

#[test]
fn decodes_plus_without_decoding_percent_encoded_plus_as_space() {
    let bytes = form_request(
        b"message=hello+%2B+world",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["message"]).unwrap();

    assert_eq!(values, ["hello + world"]);
}

#[test]
fn percent_decodes_field_names_values_and_encoded_delimiters() {
    let bytes = form_request(
        b"full+name=Ada%26Bob%3Dteam%2flead",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["full name"]).unwrap();

    assert_eq!(values, ["Ada&Bob=team/lead"]);
}

#[test]
fn percent_decodes_multibyte_utf8_text() {
    let bytes = form_request(
        b"message=Ch%C3%A0o+Rust",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["message"]).unwrap();

    assert_eq!(values, ["Ch\u{00e0}o Rust"]);
}

#[test]
fn rejects_malformed_percent_escapes() {
    for body in [
        b"value=%".as_slice(),
        b"value=%2".as_slice(),
        b"value=%GG".as_slice(),
    ] {
        let bytes = form_request(body, "application/x-www-form-urlencoded");
        let request = parse_ok(&bytes);

        assert!(
            request.extract_form(["value"]).is_err(),
            "malformed form body should be rejected: {body:?}"
        );
    }
}

#[test]
fn rejects_percent_decoded_invalid_utf8() {
    let bytes = form_request(b"value=%FF", "application/x-www-form-urlencoded");
    let request = parse_ok(&bytes);

    assert!(request.extract_form(["value"]).is_err());
}

#[test]
fn rejects_empty_form_field_name() {
    let bytes = form_request(
        b"=orphan&message=Hello",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    assert!(request.extract_form(["message"]).is_err());
}

#[test]
fn duplicate_form_fields_keep_the_last_value() {
    let bytes = form_request(b"name=first&name=last", "application/x-www-form-urlencoded");
    let request = parse_ok(&bytes);

    let values = request.extract_form(["name"]).unwrap();

    assert_eq!(values, ["last"]);
}

#[test]
fn duplicate_policy_is_applied_after_field_names_are_decoded() {
    let bytes = form_request(
        b"name=first&na%6De=last",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["name"]).unwrap();

    assert_eq!(values, ["last"]);
}

#[test]
fn reports_the_missing_required_form_field() {
    let bytes = form_request(b"name=Rust", "application/x-www-form-urlencoded");
    let request = parse_ok(&bytes);

    let result = request.extract_form(["name", "message"]);

    assert!(matches!(
        result,
        Err(HttpError::FormMissingField(field)) if field == "message"
    ));
}

#[test]
fn allows_empty_unrequested_form_field() {
    let bytes = form_request(
        b"required=ok&optional=",
        "application/x-www-form-urlencoded",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["required"]).unwrap();

    assert_eq!(values, ["ok"]);
}

#[test]
fn rejects_non_form_content_type() {
    let bytes = form_request(b"name=Rust", "text/plain");
    let request = parse_ok(&bytes);

    assert!(request.extract_form(["name"]).is_err());
}

#[test]
fn accepts_case_insensitive_form_content_type_with_parameter() {
    let bytes = form_request(
        b"name=Rust",
        "Application/X-Www-Form-Urlencoded ; charset=UTF-8",
    );
    let request = parse_ok(&bytes);

    let values = request.extract_form(["name"]).unwrap();

    assert_eq!(values, ["Rust"]);
}
