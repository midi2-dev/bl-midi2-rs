MIDI 1.0 Channel Voice Messages

## Basic Usage

```rust
use midi2::{
    prelude::*,
    channel_voice1::ControlChange,
};

let mut message = ControlChange::<[u32; 4]>::new();
message.set_channel(u4::new(0xA));
message.set_group(u4::new(0xC));
message.set_control(u7::new(0x36));
message.set_control_data(u7::new(0x37));

assert_eq!(message.data(), &[0x2CBA_3637]);
assert_eq!(message.channel(), u4::new(0xA));
assert_eq!(message.group(), u4::new(0xC));
assert_eq!(message.control(), u7::new(0x36));
assert_eq!(message.control_data(), u7::new(0x37));
```

## Channeled

`channel_voice1` messages are [Channeled](crate::Channeled).

## Grouped

`channel_voice1` messages are [Grouped](crate::Grouped) 
when backed with [Ump](crate::buffer::Ump) buffers.

## Aggregate Message

There is a single aggregate [ChannelVoice1] enum type which
can represent an arbitrary `channel_voice1` message.

```rust
use midi2::{
    prelude::*,
    channel_voice1::ChannelVoice1,
};

let mut message = ChannelVoice1::try_from(&[0x2CBA_3637_u32][..]).expect("Valid data");

match message {
    ChannelVoice1::ChannelPressure(m) => println!("channel_pressure {:?}", m.data()),
    ChannelVoice1::ControlChange(m) => println!("control_change {:?}", m.data()),
    ChannelVoice1::KeyPressure(m) => println!("key_pressure {:?}", m.data()),
    ChannelVoice1::NoteOff(m) => println!("note_off {:?}", m.data()),
    ChannelVoice1::NoteOn(m) => println!("note_on {:?}", m.data()),
    ChannelVoice1::PitchBend(m) => println!("pitch_bend {:?}", m.data()),
    ChannelVoice1::ProgramChange(m) => println!("program_change {:?}", m.data()),
}
```

## Generic Over [Unit](crate::buffer::Unit)

`channel_voice1` messages can also be represented with [Bytes](crate::buffer::Bytes) buffers
 as well as [Ump](crate::buffer::Ump) buffers.

```rust
use midi2::{
    prelude::*,
    channel_voice1::ControlChange,
};

let mut message = ControlChange::<[u8; 3]>::new();
message.set_channel(u4::new(0xA));
message.set_control(u7::new(0x36));
message.set_control_data(u7::new(0x37));

assert_eq!(message.data(), &[0xBA, 0x36, 0x37]);
```

## Fixed Size

All `channel_voice1` messages are Fixed size.

```rust
use midi2::channel_voice1::KeyPressure;


// All channel_voice1 bytes-backed messages fit into a `[u8; 3]`
let _ = KeyPressure::<[u8; 3]>::new();

// All channel_voice1 ump-backed messages fit into a `[u32; 1]`
let _ = KeyPressure::<[u32; 1]>::new();
```
