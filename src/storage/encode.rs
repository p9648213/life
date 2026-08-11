use crate::storage::error::StoreError;

pub trait Encode {
    fn encode(&self, id: u32) -> Result<Vec<u8>, StoreError>;
}

pub struct Encoder {
    pub bytes: Vec<u8>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn write_u32(&mut self, number: u32) {
        self.bytes.extend_from_slice(&number.to_be_bytes());
    }

    pub fn write_string(&mut self, text: &str) -> Result<(), StoreError> {
        let length =
            u32::try_from(text.len()).map_err(|err| StoreError::FieldTooLarge(err.to_string()))?;
        self.write_u32(length);
        self.bytes.extend_from_slice(text.as_bytes());
        Ok(())
    }
}
