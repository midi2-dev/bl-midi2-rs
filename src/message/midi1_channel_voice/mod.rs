mod channel_pressure;
mod control_change;
mod key_pressure;
mod note;
mod pitch_bend;
mod program_change;

use note::note_off;
use note::note_on;

const TYPE_CODE: ux::u4 = ux::u4::new(0x2);

pub use channel_pressure::Builder as ChannelPressureMessageBuilder;
pub use channel_pressure::Message as ChannelPressureMessage;
pub use control_change::Builder as ControlChangeMessageBuilder;
pub use control_change::Message as ControlChangeMessage;
pub use key_pressure::Builder as KeyPressureMessageBuilder;
pub use key_pressure::Message as KeyPressureMessage;
pub use note_off::Builder as NoteOffMessageBuilder;
pub use note_off::Message as NoteOffMessage;
pub use note_on::Builder as NoteOnMessageBuilder;
pub use note_on::Message as NoteOnMessage;
pub use pitch_bend::Builder as PitchBendMessageBuilder;
pub use pitch_bend::Message as PitchBendMessage;
pub use program_change::Builder as ProgramChangeMessageBuilder;
pub use program_change::Message as ProgramChangeMessage;
