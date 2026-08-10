use crate::storage::{
    decode::{Decode, Decoder},
    encode::{Encode, Encoder},
    error::StoreError,
};

#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub number: u32,
}

impl Encode for Resource {
    fn encode(&self) -> Result<Vec<u8>, StoreError> {
        let mut encoder = Encoder::new();
        encoder.write_string(&self.name)?;
        encoder.write_u32(self.number);
        Ok(encoder.bytes)
    }
}

impl Decode for Resource {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, StoreError> {
        let name = decoder.read_str()?;
        let number = decoder.read_u32()?;
        Ok(Self {
            name: name.to_owned(),
            number,
        })
    }
}
