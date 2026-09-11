use std::{fs::File, io::Read, path::Path};

use crate::{
    constant::{CONTENT_TYPE, MAX_ASSET_SIZE},
    http::response::{Response, StatusCode},
};

pub fn serve_static(static_dir: &Path, asset_part: &str) -> Response {
    if validate_asset_part(asset_part) {
        let resolved_root = match static_dir.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                return Response::text_plain(StatusCode::InternalServerError, &err.to_string());
            }
        };
        let resolved_file = match resolved_root.join(asset_part).canonicalize() {
            Ok(path) => path,
            Err(err) => {
                let status = match err.kind() {
                    std::io::ErrorKind::NotFound => StatusCode::NotFound,
                    _ => StatusCode::InternalServerError,
                };
                return Response::text_plain(status, &err.to_string());
            }
        };
        if resolved_file.starts_with(&resolved_root) {
            match resolved_file.metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        match File::open(&resolved_file) {
                            Ok(file) => {
                                let mut bytes = Vec::new();
                                match file
                                    .take((MAX_ASSET_SIZE + 1) as u64)
                                    .read_to_end(&mut bytes)
                                {
                                    Ok(bytes_read) => {
                                        if bytes_read > MAX_ASSET_SIZE {
                                            Response::text_plain(
                                                StatusCode::InternalServerError,
                                                "Limit exceed",
                                            )
                                        } else {
                                            let mut response = Response::new(StatusCode::Ok, bytes);
                                            let content_type = content_type(&resolved_file);
                                            match response.add_header(CONTENT_TYPE, content_type) {
                                                Ok(_) => {}
                                                Err(err) => {
                                                    return Response::text_plain(
                                                        StatusCode::InternalServerError,
                                                        &err.to_string(),
                                                    );
                                                }
                                            }
                                            response
                                        }
                                    }
                                    Err(err) => Response::text_plain(
                                        StatusCode::InternalServerError,
                                        &err.to_string(),
                                    ),
                                }
                            }
                            Err(err) => Response::text_plain(
                                StatusCode::InternalServerError,
                                &err.to_string(),
                            ),
                        }
                    } else {
                        Response::text_plain(StatusCode::NotFound, "Not Found")
                    }
                }
                Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
            }
        } else {
            Response::text_plain(StatusCode::NotFound, "Not Found")
        }
    } else {
        Response::text_plain(StatusCode::BadRequest, "invalid part")
    }
}

fn validate_asset_part(asset_part: &str) -> bool {
    !(asset_part.is_empty()
        || asset_part.starts_with('/')
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
