# MIDI2

A helper library for dealing with midi 2 byte data.

For detailed midi2 specification see [the documenation](https://midi.org/)
on which this library is based.

## **Note!**

This library is still in its early development phase and is not
recommended for production.

We would welcome contributions! 
Please refer to the [CONTRIBUTOR.md](CONTRIBUTOR.md)

## Features
    
### Ergonomic, semantic wrappers for midi2 message types.
Wrappers are "views" onto the underlying data
allowing for optimisation by avoiding copies.

#### Example

<!-- todo: add to doc tests -->
```rust
let note_on = message::midi2_channel_voice::NoteOnMessage::builder(&mut [0x0; 2])
    .note(u7::new(0x60))
    .velocity(0x4B57_u16)
    .attribute(Attribute::Pitch7_9{
        note: 0x60,
        pitch_up: u9::new(0xB6),
    })
    .build();
assert_eq!(note_on.group() == u4::default());
assert_eq!(note_on.channel() == u4::default());
```

### Midi2 Capability Inquiry message wrappers
Wrappers around the special midi2 Capability Inquiry.
These messages are represented by groups of either midi1 or midi2 
system exclusive messages.

### `#![no_std]`
The library is entirely no_std, which guarantees that 
it will never allocate memory under the hood.
This makes it suitable for use on realtime audio threads.

