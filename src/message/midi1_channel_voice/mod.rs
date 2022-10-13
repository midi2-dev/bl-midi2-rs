pub mod channel_pressure;
pub mod control_change;
pub mod key_pressure;
pub mod pitch_bend;
pub mod program_change;

mod note;

pub use note::note_off;
pub use note::note_on;

const TYPE_CODE: ux::u4 = ux::u4::new(0x2);