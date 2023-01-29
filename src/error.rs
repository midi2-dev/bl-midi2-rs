#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidData,
    MissingFields,
    BufferOverflow,
}
