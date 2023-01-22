#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidData,
    MissingFields,
    BufferOverflow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidData();
