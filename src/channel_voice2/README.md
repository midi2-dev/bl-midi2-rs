MIDI 2.0 Channel Voice Messages

## Basic Usage

```rust
use midi2::{
    prelude::*,
    channel_voice2::NoteOn,
};

let mut message = NoteOn::<[u32; 4]>::new();
message.set_group(u4::new(0x8));
message.set_channel(u4::new(0x8));
message.set_note(u7::new(0x5E));
message.set_velocity(0x6A14);
message.set_attribute(Some(channel_voice2::NoteAttribute::Pitch7_9 {
    note: u7::new(0x74),
    pitch_up: u9::new(0x18A),
}));

assert_eq!(message.data(), &[0x4898_5E03, 0x6A14_E98A]);
assert_eq!(message.group(), u4::new(0x8));
assert_eq!(message.channel(), u4::new(0x8));
assert_eq!(message.note(), u7::new(0x5E));
assert_eq!(message.velocity(), 0x6A14);
assert_eq!(
    message.attribute(), 
    Some(channel_voice2::NoteAttribute::Pitch7_9 {
        note: u7::new(0x74),
        pitch_up: u9::new(0x18A),
    }
));
```

## Channeled

`channel_voice2` messages are [Channeled](crate::Channeled).

## Grouped

`channel_voice2` messages are [Grouped](crate::Grouped).

## Aggregate Message

There is a single aggregate [ChannelVoice2] enum type which
can represent an arbitrary `channel_voice2` message.

```rust
use midi2::{
    prelude::*,
    channel_voice2::ChannelVoice2,
};

let mut message = ChannelVoice2::try_from(&[0x4898_5E03, 0x6A14_E98A][..]).expect("Valid data");

match message {
    ChannelVoice2::AssignableController(m) => println!("assignable_controller {:?}", m.data()),
    ChannelVoice2::AssignablePerNoteController(m) => println!("assignable_per_note_controller {:?}", m.data()),
    ChannelVoice2::ChannelPitchBend(m) => println!("channel_pitch_bend {:?}", m.data()),
    ChannelVoice2::ChannelPressure(m) => println!("channel_pressure {:?}", m.data()),
    ChannelVoice2::ControlChange(m) => println!("control_change {:?}", m.data()),
    ChannelVoice2::KeyPressure(m) => println!("key_pressure {:?}", m.data()),
    ChannelVoice2::NoteOff(m) => println!("note_off {:?}", m.data()),
    ChannelVoice2::NoteOn(m) => println!("note_on {:?}", m.data()),
    ChannelVoice2::PerNoteManagement(m) => println!("per_note_management {:?}", m.data()),
    ChannelVoice2::PerNotePitchBend(m) => println!("per_note_pitch_bend {:?}", m.data()),
    ChannelVoice2::ProgramChange(m) => println!("program_change {:?}", m.data()),
    ChannelVoice2::RegisteredController(m) => println!("registered_controller {:?}", m.data()),
    ChannelVoice2::RegisteredPerNoteController(m) => println!("registered_per_note_controller {:?}", m.data()),
    ChannelVoice2::RelativeAssignableController(m) => println!("relative_assignable_controller {:?}", m.data()),
    ChannelVoice2::RelativeRegisteredController(m) => println!("relative_registered_controller {:?}", m.data()),
}
```

## Fixed Size

All `channel_voice1` messages are Fixed size and will fit 
into an array of [u32] size 2 or greater.

```rust
use midi2::channel_voice2::NoteOn;

let _ = NoteOn::<[u32; 2]>::new();
let _ = NoteOn::<[u32; 4]>::new();
```

Arrays smaller than two are invalid backing buffers.

```rust,compile_fail,E0080
use midi2::channel_voice2::NoteOn;

let _ = NoteOn::<[u32; 1]>::new(); // compile err - buffer too short
```
