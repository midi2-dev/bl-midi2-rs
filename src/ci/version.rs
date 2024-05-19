pub trait CiVersion<const VERSION: u8> {}

impl<T: CiVersion<2>> CiVersion<1> for T {}

pub(crate) struct ValidCiVersion<const VERSION: u8>;

impl<const VERSION: u8> ValidCiVersion<VERSION> {
    pub const VALID: () = match VERSION {
        0x1 => {}
        0x2 => {}
        _ => panic! {"Invalid CI Version"},
    };
}
