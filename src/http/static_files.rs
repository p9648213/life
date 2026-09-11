use std::path::Path;

use crate::http::response::{Response, StatusCode};

pub fn serve_static(static_dir: &Path) -> Response {
    Response::new(StatusCode::Ok, vec![])
}
