use crate::storage::codec::Encode;

pub struct Resource {
    pub name: String,
    pub number: u32,
}

impl Encode for Resource {
    fn encode(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(self.name.as_bytes());
        data.extend_from_slice(&self.number.to_be_bytes());
        data
    }
}
