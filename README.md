# MIDI2

Ergonomic, versatile, strong types wrapping MIDI 2.0 message data.

For detailed midi2 specification see [the documentation](https://midi.org/)
on which this library is based.

## ⚠️  **Note!** ⚠️  

This crate is still in its alpha phase and is not
recommended for production.

We would welcome contributions! 
Please refer to the [CONTRIBUTOR.md](CONTRIBUTOR.md)

## Quick Start

todo

## Strongly Typed Message Wrappers

A strongly typed message wrapper is provided for every message in the MIDI 2.0 specification 
(version 1.1 as the time of writing).


```rust
use midi2::prelude::*;

// Messages have a simple setter / getter interface
let mut note_on = channel_voice2::NoteOn::new_arr();
note_on.set_group(u4::new(0x8));
note_on.set_channel(u4::new(0xA));
note_on.set_note(u7::new(0x5E));
note_on.set_velocity(0x6A14);

assert_eq!(note_on.group(), u4::new(0x8));
assert_eq!(note_on.channel(), u4::new(0xA));
assert_eq!(note_on.note(), u7::new(0x5E));
assert_eq!(note_on.velocity(), 0x6A14);

// Messages wrap an underlying buffer of data which can be read as an
// ordinary slice.
let mut composer_name = flex_data::ComposerName::<Vec<u32>>::new();
composer_name.set_name("Pinch b2b Peverelist");
assert_eq!(
    composer_name.data(), 
    &[
        0xD0500105,
        0x50696E63,
        0x68206232,
        0x62205065,
        0xD0D00105,
        0x76657265,
        0x6C697374,
        0x0000_0000,
]);

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

## Jitter Ruduction Support

All ump messages can have a jitter deduction header
prepended before its message packets.

```rust
use midi2::prelude::*;

let mut message = channel_voice1::ChannelPressure::new_arr();
message.set_jitter_reduction(Some(JitterReduction::Timestamp(0x1234)));
assert_eq!(
    message.data(),
    &[0x0020_1234, 0x20D0_0000],
);
```

## Almost Entirely no_std Friendly

todo

## Borrowed Messages

todo

## Generic Representation

todo

## Supports For Classical MIDI Byte Stream Messages

todo

## Cargo Features

Almost all message categories are opt-in.
