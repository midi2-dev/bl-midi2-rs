pub trait CiVersion<const VERSION: u8> {}

impl<T: CiVersion<2>> CiVersion<1> for T {}
