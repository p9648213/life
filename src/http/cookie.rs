use std::fmt::Write;
use std::{collections::HashMap, time::Duration};

pub enum SameSite {
    Lax,
    Strict,
    None,
}

pub struct Cookie {
    values: HashMap<String, String>,
    path: String,
    domain: String,
    http_only: bool,
    secure: bool,
    max_age: Duration,
    expires: Duration,
    samesite: SameSite,
}

impl Cookie {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            path: String::from("/"),
            domain: String::new(),
            http_only: true,
            secure: false,
            max_age: Duration::from_hours(48),
            expires: Duration::from_millis(0),
            samesite: SameSite::Lax,
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = domain;
    }

    pub fn set_http_only(&mut self, http_only: bool) {
        self.http_only = http_only
    }

    pub fn set_secure(&mut self, secure: bool) {
        self.secure = secure;
    }

    pub fn set_max_age(&mut self, max_age: Duration) {
        self.max_age = max_age;
    }

    pub fn set_expire(&mut self, expire: Duration) {
        self.expires = expire;
    }

    pub fn samesite(&mut self, samesite: SameSite) {
        self.samesite = samesite
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        for (key, value) in &self.values {
            write!(&mut result, "{}:{};", key, value).unwrap_or_default();
        }
        write!(&mut result, "Path:{};", self.path).unwrap_or_default();
        if !self.domain.is_empty() {
            write!(&mut result, "Domain:{};", self.domain).unwrap_or_default();
        }
        if self.http_only {
            result.push_str("HttpOnly:true;");
        } else {
            result.push_str("HttpOnly:false;");
        }
        if self.secure {
            result.push_str("Secure:true");
        } else {
            result.push_str("Secure:false");
        }
        if !self.max_age.is_zero() {
            write!(&mut result, "Max-Age:{};", self.max_age.as_millis()).unwrap_or_default();
        } else {
            if !self.expires.is_zero() {
                write!(&mut result, "Expires:{};", self.expires.as_millis()).unwrap_or_default();
            }
        }
        match self.samesite {
            SameSite::Lax => result.push_str("SameSite:Lax"),
            SameSite::Strict => result.push_str("SameSite:Strict"),
            SameSite::None => result.push_str("SameSite:None"),
        }
        result
    }
}

impl Default for Cookie {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_cookie(value: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in value.split(";") {
        if let Some((key, value)) = pair.split_once("=") {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    map
}
