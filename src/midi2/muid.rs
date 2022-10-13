use crate::util::truncate;

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
            truncate(v >> 26), 
            truncate((v & 0x00FF_0000) >> 16),
            truncate((v & 0x0000_FF00) >> 8),
            truncate(v & 0x0000_00FF), 
        ])
    }

    pub fn value<'a>(&'a self) -> &'a [u8; 4] {
        &self.0
    }
}

pub enum Index {
    Byte1,
    Byte2,
    Byte3,
    Byte4,
}

impl std::ops::Index<Index> for Muid {
    type Output = u8;

    fn index(&self, i: Index) -> &Self::Output {
        match i {
            Index::Byte1 => &self.0[0],
            Index::Byte2 => &self.0[1],
            Index::Byte3 => &self.0[2],
            Index::Byte4 => &self.0[3],
        }
    }
}
