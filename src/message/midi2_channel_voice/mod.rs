pub mod assignable_per_note_controller;
pub mod control_change;
pub mod per_note_management;
pub mod program_change;
pub mod registered_per_note_controller;

mod attribute;
mod channel_effect;
mod controller;
mod controllers;
mod note;
mod per_note_effect;

pub use attribute::Attribute;
pub use channel_effect::pitch_bend;
pub use channel_effect::pressure as channel_pressure;
pub use controller::assignable as assignable_controller;
pub use controller::registered as registered_controller;
pub use controller::relative_assignable as relative_assignable_controller;
pub use controller::relative_registered as relative_registered_controller;
pub use controllers::Controller;
pub use note::note_off;
pub use note::note_on;
pub use per_note_effect::key_pressure;
pub use per_note_effect::pitch_bend as per_note_pitch_bend;

const TYPE_CODE: ux::u4 = ux::u4::new(0x4);
