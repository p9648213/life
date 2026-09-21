use std::random::random;

pub trait HasId {
    fn id(&self) -> u32;
}