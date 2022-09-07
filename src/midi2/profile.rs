#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum Id {
    Standard {
        bank: u8,
        number: u8,
        version: u8,
        level: SupportLevel,
    },
    Manufacturer {
        id: [u8; 3],
        data: [u8; 2],
    },
}

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub enum SupportLevel {
    Partial,
    Minimum,
    Extended(ux::u7),
    Highest,
}

