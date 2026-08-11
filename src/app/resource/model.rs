use crate::storage::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder},
    error::StoreError,
};

#[derive(Debug)]
pub struct Resource {
    pub id: u32,
    pub name: String,
    pub number: u32,
}

impl Encode for Resource {
    fn encode(&self, id: u32) -> Result<Vec<u8>, StoreError> {
        let mut encoder = Encoder::new();
        encoder.write_u32(id);
        encoder.write_string(&self.name)?;
        encoder.write_u32(self.number);
        Ok(encoder.bytes)
    }
}

impl Decode for Resource {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, StoreError> {
        let id = decoder.read_u32()?;
        let name = decoder.read_str()?.to_owned();
        let number = decoder.read_u32()?;
        Ok(Self { id, name, number })
    }
}

impl Resource {
    pub fn new(name: String, number: u32) -> Self {
        Self {
            id: 0,
            name,
            number,
        }
    }
}
