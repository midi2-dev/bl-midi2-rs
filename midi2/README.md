# 🎹 MIDI2 🎹

[![crates.io](https://img.shields.io/crates/v/midi2.svg)](https://crates.io/crates/midi2)
[![docs.rs](https://docs.rs/midi2/badge.svg)](https://docs.rs/midi2)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)
![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)

Ergonomic, versatile, strong types wrapping MIDI 2.0 message data.

This implementation of MIDI 2.0 is based on the 1.1 revision of the specifications.
See [the official MIDI 2.0 specification](https://midi.org/)
for more details on the data protocol standard.

> [!CAUTION]
> 
> This project is still in early development.  
> Expect breaking changes and bugs, and please report any issues you encounter.

We would welcome contributions! 
Please refer to the [CONTRIBUTOR.md](CONTRIBUTOR.md)

## Strongly Typed Message Wrappers

A strongly typed message wrapper is provided for every message in the MIDI 2.0 specification.


```rust
use midi2::prelude::*;

// Messages have a simple setter / getter interface
let mut note_on = channel_voice2::NoteOn::<[u32; 4]>::new();
note_on.set_group(u4::new(0x8));
note_on.set_channel(u4::new(0xA));
note_on.set_note_number(u7::new(0x5E));
note_on.set_velocity(0x6A14);

assert_eq!(note_on.group(), u4::new(0x8));
assert_eq!(note_on.channel(), u4::new(0xA));
assert_eq!(note_on.note_number(), u7::new(0x5E));
assert_eq!(note_on.velocity(), 0x6A14);
assert_eq!(note_on.data(), &[0x489A_5E00, 0x6A14_0000]);

// Messages wrap an underlying buffer of data which can be read as an
// ordinary slice.
let mut composer_name = flex_data::ComposerName::<Vec<u32>>::new();
composer_name.set_name("Pinch b2b Peverelist");
assert_eq!(
    composer_name.data(), 
    &[
        0xD050_0105,
        0x5069_6E63,
        0x6820_6232,
        0x6220_5065,
        0xD0D0_0105,
        0x7665_7265,
        0x6C69_7374,
        0x0000_0000,
    ]
);
```

## Aggregate Message Types

All message wrappers are grouped into aggregate enum types.
There's a top level enum type which can represent all messages,
and there's sub enum types for each different UMP type specified
by the MIDI 2.0 specification.

```rust
fn handle_message(buffer: &[u32]) {
    use midi2::prelude::*;

    match UmpMessage::try_from(buffer) {
        Ok(UmpMessage::ChannelVoice2(m)) => {
            println!("Channel Voice2: channel: {}", m.channel());
            match m {
                channel_voice2::ChannelVoice2::NoteOn(m) => {
                    println!("Note On! note: {}, velocity: {}", m.note_number(), m.velocity());
                }
                channel_voice2::ChannelVoice2::NoteOff(m) => {
                    println!("Note Off! note: {}, velocity: {}", m.note_number(), m.velocity());
                }
                _ => {}
            }
        }
        Ok(UmpMessage::Sysex7(m)) => {
            println!(
                "Sysex 7bit: payload: {:?}",
                m.payload().collect::<Vec<u7>>()
            );
        }
        Ok(UmpMessage::FlexData(m)) => {
            use midi2::flex_data::FlexDataMessage;

            println!("FlexData: bank: {:?}", m.bank());
            match m {
                _ => {}, // further matching on different flex data types
            }
        }
        // further matching on other message types
        Err(e) => {
            println!("Error parsing ump buffer: {:?}", e);
        }
        _ => {}
    }
}
```

## Full Sysex Support

Sysex message can be represented with MIDI 2.0 Universal Message Packets.

```rust
use midi2::prelude::*;

let mut message = sysex7::Sysex7::<Vec<u32>>::new();
message.set_payload((0u8..30u8).map(u7::new));
message.set_group(u4::new(0xA));

assert_eq!(
    message.data(),
    &[
        0x3A16_0001,
        0x0203_0405,
        0x3A26_0607,
        0x0809_0A0B,
        0x3A26_0C0D,
        0x0E0F_1011,
        0x3A26_1213,
        0x1415_1617,
        0x3A36_1819,
        0x1A1B_1C1D,
    ],
);
```

Or with classical MIDI 2.0 byte streams.

```rust
use midi2::prelude::*;

let mut message = sysex7::Sysex7::<Vec<u8>>::new();
message.set_payload((0u8..30u8).map(u7::new));

assert_eq!(
    message.data(),
    &[
        0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
        0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A,
        0x1B, 0x1C, 0x1D, 0xF7,
    ],
);
```

## Almost Entirely `#![no_std]` Friendly

`#![no_std]` is a first class use case in midi2.
All message types can be read and written without allocation,
even messages of arbitrary length, like sysex or flex-data.

You'll want to setup midi2 without default features to compile
without the `std` feature.

```toml
midi2 = { version = "0.8.0", default-features = false, features = ["channel-voice2", "sysex7"],  }
```

### Generic Representation

All messages are generic over their representation.
For example, a simple non-allocating use case would be to
represent messages within a fixed size array.

```rust
use midi2::prelude::*;

let mut message = sysex8::Sysex8::<[u32; 16]>::new();

// in this mode methods which would require a 
// buffer resize are fallible
assert_eq!(message.try_set_payload(0..50), Ok(()));

// if there's not enough room in the buffer to 
// accommodate the resize then an overflow error is returned.
assert_eq!(message.try_set_payload(0..60), Err(midi2::error::BufferOverflow));
```

A more advanced use case might be to make a custom buffer which
uses an arena allocator to back your messages.
See the [buffer] docs for more info.

### Borrowed Messages

When reading messages from an existing buffer, the message wrappers
own a borrowed reference to the data, so no copying or allocation takes place.
In this case the generic message buffer type is `&[u32]`.

```rust
use midi2::prelude::*;

let buffer = [
    0xD050_0100_u32,
    0x4469_6769,
    0x7461_6C20,
    0x4175_6469,
    0xD090_0100,
    0x6F20_576F,
    0x726B_7374,
    0x6174_696F,
    0xD0D0_0100,
    0x6E20_2D20,
    0x4441_5733,
    0x362D_3136,
];
let message = UmpMessage::try_from(&buffer[..]).expect("Valid data");
```

Of course this means that such borrowed messages are immutable
and also have their lifetimes tied to the original buffer.

To remedy this messages can be `rebuffered` into a different
generic backing buffer type.

```rust
use midi2::{
    prelude::*,
    channel_voice2::NoteOn,
};

let mut owned: NoteOn::<[u32; 4]> = {
    let buffer = [0x4898_5E03_u32, 0x6A14_E98A];
    // the borrowed message is immutable and cannot outlive `buffer`
    let borrowed = NoteOn::try_from(&buffer[..]).expect("Data is valid");
    borrowed.array_rebuffer_into()
};

// the owned message is mutable and liberated from the buffer lifetime.
owned.set_channel(u4::new(0x9));
assert_eq!(owned.data(), &[0x4899_5E03, 0x6A14_E98A])
```

## Support For Classical MIDI Byte Stream Messages

Messages which can be represented in classical MIDI byte stream format are also supported. 
To do this simply use a backing buffer over `u8` instead of `u32`! ✨🎩

```rust
use midi2::prelude::*;

let mut message = channel_voice1::ChannelPressure::<[u8; 3]>::new();
message.set_channel(u4::new(0x6));
message.set_pressure(u7::new(0x09));

assert_eq!(message.data(), &[0xD6, 0x09]);
```

Messages represented in bytes can be transformed to ump and back using conversion traits.

```rust
use midi2::{
    prelude::*,
    channel_voice1::ChannelPressure,
};

let message = ChannelPressure::<[u8; 3]>::new();
let message: ChannelPressure<[u32; 4]> = message.into_ump();

assert_eq!(message.data(), &[0x20D0_0000]);
```

## Cargo Features

Several compile-time features are provided that you can enable or disable to customize
functionality according to your needs.

Here's a list of available features:

- `default`:
  - **std** - Include [buffer] integration for `std::vec::Vec` and enable allocating getters for values which return `std::string::String` values.
  - **channel-voice2** — Include message wrappers for the MIDI 2.0 channel voice message type.

- `optional`: These features are not enabled by default and can be included by adding them to your `Cargo.toml`.
  - **flex-data** - Include message wrappers for the MIDI 2.0 Flex Data message type.
  - **channel-voice1** - Include message wrappers for the classical MIDI channel voice message type.
  - **sysex7** — Include message wrappers for the MIDI 7bit system exclusive message type.
  - **sysex8** - Include message wrappers for the MIDI 2.0 System Exclusive 8bit message type.
  - **system-common** - Include message wrappers for the MIDI 2.0 System Common / System Real Time message type.
  - **ump-stream** - Include message wrappers for the MIDI 2.0 Ump Stream message type.
  - **ci** — 🚧 WIP 🚧
