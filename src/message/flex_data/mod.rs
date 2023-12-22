use crate::Data;

pub mod set_key_signature;
pub mod set_metronome;
pub mod set_tempo;
pub mod set_time_signature;

const TYPE_CODE: u32 = 0xD;
const SETUP_AND_PERFORMANCE_BANK: u32 = 0x0;
const _METADATA_TEXT_BANK: u32 = 0x0;
const _PERFORMANCE_TEXT_BANK: u32 = 0x0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bank {
    SetupAndPerformance,
    MetadataText,
    PerformanceText,
}

pub trait Banked: Data {
    fn bank(&self) -> Bank {
        use Bank::*;
        match (self.data()[0] & 0x0000_FF00) >> 8 {
            0x0 => SetupAndPerformance,
            0x1 => MetadataText,
            0x2 => PerformanceText,
            _ => panic!(),
        }
    }
}
