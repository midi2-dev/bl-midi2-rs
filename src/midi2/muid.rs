use crate::helpers::mask;

#[derive(Debug, PartialEq, Clone)]
pub struct Muid([u8; 4]);

impl Muid {
    pub fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let v = oorandom::Rand32::new(seed).rand_u32();
        Muid([
            mask(v >> 26), 
            mask((v & 0x00FF_0000) >> 16),
            mask((v & 0x0000_FF00) >> 8),
            mask(v & 0x0000_00FF), 
        ])
    }

    pub fn value<'a>(&'a self) -> &'a [u8; 4] {
        &self.0
    }
}
