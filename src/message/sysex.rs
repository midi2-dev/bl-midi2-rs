pub trait SysexMessages {
    type Byte: core::convert::Into<u8>;
    type Builder;
    type PayloadIterator: core::iter::Iterator<Item = Self::Byte> + Clone;
    fn payload(&self) -> Self::PayloadIterator;
}
