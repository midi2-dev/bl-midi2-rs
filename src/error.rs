#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BufferOverflow,
    InvalidData,
}
