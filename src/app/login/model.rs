use crate::storage::{
    decode::Decode,
    encode::{Encode, Encoder},
    util::HasId,
};

pub struct User {
    id: u32,
    username: String,
    password: String,
    session: String,
}

impl Encode for User {
    fn encode(&self, id: u32) -> Result<Vec<u8>, crate::storage::error::StoreError> {
        let mut encoder = Encoder::new();
        encoder.write_u32(id);
        encoder.write_string(&self.username)?;
        encoder.write_string(&self.password)?;
        encoder.write_string(&self.session)?;
        Ok(encoder.bytes)
    }
}

impl Decode for User {
    fn decode(
        decoder: &mut crate::storage::decode::Decoder<'_>,
    ) -> Result<Self, crate::storage::error::StoreError> {
        let id = decoder.read_u32()?;
        let username = decoder.read_str()?.to_owned();
        let password = decoder.read_str()?.to_owned();
        let session = decoder.read_str()?.to_owned();
        Ok(Self {
            id,
            username,
            password,
            session,
        })
    }
}

impl HasId for User {
    fn id(&self) -> u32 {
        self.id
    }
}
