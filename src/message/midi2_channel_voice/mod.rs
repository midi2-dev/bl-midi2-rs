pub mod key_pressure;
pub mod assignable_per_note_controller;
pub mod registered_per_note_controller;
pub mod per_note_management;

mod attribute;
mod controllers;
mod note;

pub use attribute::Attribute;
pub use controllers::Controller;
pub use note::note_on;
pub use note::note_off;

const TYPE_CODE: ux::u4 = ux::u4::new(0x4);