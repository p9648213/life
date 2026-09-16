use std::fmt::Write;
use std::time::Duration;

use crate::http::error::HttpError;

pub enum SameSite {
    Lax,
    Strict,
    None,
}

pub struct Cookie {
    name: String,
    value: String,
    path: String,
    domain: String,
    http_only: bool,
    secure: bool,
    max_age: Option<Duration>,
    samesite: SameSite,
}

impl Cookie {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            path: String::from("/"),
            domain: String::new(),
            http_only: true,
            secure: false,
            max_age: None,
            samesite: SameSite::Lax,
        }
    }

    pub fn set_name_value(&mut self, name: &str, value: &str) -> Result<(), HttpError> {
        if is_valid_cookie_name(name) && is_valid_cookie_value(value) {
            self.name = name.to_string();
            self.value = value.to_string();
            Ok(())
        } else {
            Err(HttpError::InvalidCookie)
        }
    }

    pub fn set_path(&mut self, path: String) -> Result<(), HttpError> {
        if is_valid_cookie_path(&path) {
            self.path = path;
            Ok(())
        } else {
            Err(HttpError::InvalidCookie)
        }
    }

    pub fn set_domain(&mut self, domain: &str) -> Result<(), HttpError> {
        if is_valid_cookie_domain(domain) {
            self.domain = domain.to_string();
            Ok(())
        } else {
            Err(HttpError::InvalidCookie)
        }
    }

    pub fn set_http_only(&mut self, http_only: bool) {
        self.http_only = http_only
    }

    pub fn set_secure(&mut self, secure: bool) {
        self.secure = secure;
    }

    pub fn set_max_age(&mut self, max_age: Duration) {
        self.max_age = Some(max_age);
    }

    pub fn samesite(&mut self, samesite: SameSite) {
        self.samesite = samesite
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        write!(&mut result, "{}={};", self.name, self.value).unwrap_or_default();
        write!(&mut result, "Path={};", self.path).unwrap_or_default();
        if !self.domain.is_empty() {
            write!(&mut result, "Domain={};", self.domain).unwrap_or_default();
        }
        if self.http_only {
            result.push_str("HttpOnly;");
        }
        if self.secure {
            result.push_str("Secure;");
        }
        if let Some(max_age) = self.max_age {
            write!(&mut result, "Max-Age={};", max_age.as_secs()).unwrap_or_default();
        }
        match self.samesite {
            SameSite::Lax => result.push_str("SameSite=Lax"),
            SameSite::Strict => result.push_str("SameSite=Strict"),
            SameSite::None => result.push_str("SameSite=None"),
        }
        result
    }
}

impl Default for Cookie {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_cookie(value: &str) -> Result<Vec<(&str, &str)>, HttpError> {
    let mut kv = Vec::new();
    for pair in value.split(";") {
        if let Some((key, value)) = pair.split_once("=") {
            let key = key.trim_matches([' ', '\t']);
            let value = value.trim_matches([' ', '\t']);
            if is_valid_cookie_name(key) && is_valid_cookie_value(value) {
                kv.push((key, value));
            } else {
                return Err(HttpError::InvalidCookie);
            }
        } else {
            return Err(HttpError::InvalidCookie);
        }
    }
    Ok(kv)
}

fn is_valid_cookie_name(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(
                    byte,
                    b'!' | b'#'
                        | b'$'
                        | b'%'
                        | b'&'
                        | b'\''
                        | b'*'
                        | b'+'
                        | b'-'
                        | b'.'
                        | b'^'
                        | b'_'
                        | b'`'
                        | b'|'
                        | b'~'
                )
        })
}

fn is_valid_cookie_value(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| (0x21..=0x7E).contains(&byte) && !matches!(byte, b'"' | b',' | b';' | b'\\'))
}

fn is_valid_cookie_domain(domain: &str) -> bool {
    if domain.is_empty() {
        return true;
    }

    let domain = domain.strip_prefix('.').unwrap_or(domain);

    if domain.is_empty() || domain.len() > 253 {
        return false;
    }

    domain.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    })
}

fn is_valid_cookie_path(path: &str) -> bool {
    path.starts_with('/')
        && path
            .bytes()
            .all(|byte| (0x20..=0x7E).contains(&byte) && byte != b';')
}
