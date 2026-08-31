use crate::storage::error::StoreError;

pub trait Decode: Sized {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, StoreError>;
}

pub struct Decoder<'a> {
    pub bytes: &'a [u8],
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], StoreError> {
        let (value, remaining) = if let Some(result) = self.bytes.split_at_checked(len) {
            result
        } else {
            return Err(StoreError::UnexpectedEndOfPayload);
        };
        self.bytes = remaining;
        Ok(value)
    }

    pub fn read_str(&mut self) -> Result<&'a str, StoreError> {
        let len = self.read_u32()? as usize;
        let bytes = self.read_bytes(len)?;
        let value = str::from_utf8(bytes)?;
        Ok(value)
    }

    pub fn read_u32(&mut self) -> Result<u32, StoreError> {
        let bytes: [u8; 4] = self.read_bytes(4)?.try_into()?;
        let value = u32::from_be_bytes(bytes);
        Ok(value)
    }
}
