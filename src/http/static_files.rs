use std::{fs::File, io::Read, path::Path};

use crate::{
    constant::{CONTENT_TYPE, MAX_ASSET_SIZE},
    http::{
        error::HttpError,
        response::{Response, StatusCode},
    },
};

enum StaticFileError {
    InvalidPath,
    NotFound,
    TooLarge,
    Io(std::io::Error),
    Header(HttpError),
}

impl StaticFileError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::InvalidPath => (StatusCode::BadRequest, "invalid part"),
            Self::NotFound => (StatusCode::NotFound, "Not Found"),
            Self::TooLarge => (StatusCode::InternalServerError, "Limit exceed"),
            Self::Io(err) => {
                eprintln!("Static file I/O error: {err}");
                (StatusCode::InternalServerError, "Internal Server Error")
            }
            Self::Header(err) => {
                eprintln!("Static file response header error: {err}");
                (StatusCode::InternalServerError, "Internal Server Error")
            }
        };
        Response::text_plain(status, message)
    }
}

pub fn serve_static(static_dir: &Path, asset_part: &str) -> Response {
    match try_serve_static(static_dir, asset_part) {
        Ok(response) => response,
        Err(err) => err.into_response(),
    }
}

fn try_serve_static(static_dir: &Path, asset_part: &str) -> Result<Response, StaticFileError> {
    if !validate_asset_part(asset_part) {
        return Err(StaticFileError::InvalidPath);
    }
    let resolved_root = static_dir.canonicalize().map_err(StaticFileError::Io)?;
    let resolved_file = resolved_root
        .join(asset_part)
        .canonicalize()
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory => {
                StaticFileError::NotFound
            }
            _ => StaticFileError::Io(err),
        })?;
    if !resolved_file.starts_with(&resolved_root) {
        return Err(StaticFileError::NotFound);
    }
    let metadata = resolved_file.metadata().map_err(StaticFileError::Io)?;
    if !metadata.is_file() {
        return Err(StaticFileError::NotFound);
    }
    let file = File::open(&resolved_file).map_err(StaticFileError::Io)?;
    let mut bytes = Vec::new();
    file.take((MAX_ASSET_SIZE + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(StaticFileError::Io)?;
    if bytes.len() > MAX_ASSET_SIZE {
        return Err(StaticFileError::TooLarge);
    }
    let mut response = Response::new(StatusCode::Ok, bytes);
    response
        .add_header(CONTENT_TYPE, content_type(&resolved_file))
        .map_err(StaticFileError::Header)?;
    Ok(response)
}

fn validate_asset_part(asset_part: &str) -> bool {
    !(asset_part.starts_with('/')
        || asset_part.contains(['\\', ':', '\0', '%'])
        || asset_part
            .split('/')
            .any(|segment| segment == "." || segment == ".."))
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("css") => "text/css; charset=utf-8",
        Some(ext) if ext.eq_ignore_ascii_case("js") => "text/javascript; charset=utf-8",
        Some(ext) if ext.eq_ignore_ascii_case("png") => "image/png",
        Some(ext) if ext.eq_ignore_ascii_case("jpg") => "image/jpeg",
        Some(ext) if ext.eq_ignore_ascii_case("jpeg") => "image/jpeg",
        Some(ext) if ext.eq_ignore_ascii_case("svg") => "image/svg+xml",
        Some(ext) if ext.eq_ignore_ascii_case("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}
