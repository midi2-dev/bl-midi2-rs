#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Error {
    InvalidData,
    MissingFields,
    BufferOverflow,
}
