use std::io::{Error, Read};

use crate::constant::{CONTENT_LENGTH, MAX_BUFFER_SIZE, MAX_REQUEST_BYTES};

pub fn read_one_request(reader: &mut impl Read) -> std::io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut expected_length = None;
    let mut index = 0;
    loop {
        let mut buffer = [0u8; MAX_BUFFER_SIZE];
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            if let Some(expected_length) = expected_length
                && data.len() < expected_length
            {
                return Err(Error::other("Request body incomplete"));
            }
            if expected_length.is_none() {
                return Err(Error::other("Partial header"));
            }
            break;
        }
        if let Some(expected_length) = expected_length
            && expected_length - data.len() <= MAX_BUFFER_SIZE
            && expected_length - data.len() <= bytes_read
        {
            data.extend_from_slice(&buffer[..(expected_length - data.len())]);
        } else {
            let remaining = MAX_REQUEST_BYTES - data.len();
            let bytes_after_extend = data
                .len()
                .checked_add(bytes_read)
                .ok_or_else(|| Error::other("Request size overflow"))?;
            if bytes_after_extend > MAX_REQUEST_BYTES {
                data.extend_from_slice(&buffer[..remaining]);
            } else {
                data.extend_from_slice(&buffer[..bytes_read]);
            }
        }
        if expected_length.is_none() {
            while index + 4 <= data.len() && expected_length.is_none() {
                if data.get(index..index + 4) == Some(&[13, 10, 13, 10]) {
                    let headers = str::from_utf8(&data[..index])
                        .map_err(|err| Error::other(err.to_string()))?;
                    let headers_iters = headers.split("\r\n");
                    for (header_index, header) in headers_iters.enumerate() {
                        if header_index == 0 {
                            continue;
                        }
                        if let Some((name, value)) = header.split_once(":") {
                            if name.is_empty() || name.contains(' ') {
                                return Err(Error::other("Request header invalid"));
                            }
                            if name.eq_ignore_ascii_case(CONTENT_LENGTH) {
                                let value = value
                                    .trim()
                                    .parse::<usize>()
                                    .map_err(|err| Error::other(err.to_string()))?;
                                let sum = value
                                    .checked_add(index + 4)
                                    .ok_or_else(|| Error::other("Request size overflow"))?;
                                expected_length = Some(sum);
                            }
                        }
                    }
                    if expected_length.is_none() {
                        expected_length = Some(index + 4);
                    }
                    if let Some(expected_length) = expected_length
                        && expected_length > MAX_REQUEST_BYTES
                    {
                        return Err(Error::other("Request size overflow"));
                    }
                }
                index += 1;
            }
            if expected_length.is_none() && data.len() == MAX_REQUEST_BYTES {
                return Err(Error::other("Request size overflow"));
            }
        }
        if let Some(expected_length) = expected_length {
            if data.len() == expected_length {
                break;
            }
            if data.len() > expected_length {
                data.truncate(expected_length);
                break;
            }
        }
    }
    Ok(data)
}
