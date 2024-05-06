# MIDI2

A helper library for dealing with midi 2 byte data.

For detailed midi2 specification see [the documentation](https://midi.org/)
on which this library is based.

## **Note!**

This library is still in its early development phase and is not
recommended for production.

We would welcome contributions! 
Please refer to the [CONTRIBUTOR.md](CONTRIBUTOR.md)

## Ergonomic, semantic wrappers for midi2 message types.

Create immutable messages using the builder pattern, 
which offer an intuitive and easy to use api over the
underlying byte data.

```rust
use midi2::prelude::*;

#[cfg(feature = "midi2-channel-voice")]
{
    let message = Message::builder()
        .channel_voice2()
        .note_on()
        .group(u4::new(0xD))
        .note(u7::new(0x60))
        .velocity(0x4B57)
        .build()
        .unwrap();

    assert_eq!(message.data(), &[0x4D90_6000, 0x4B57_0000]);
}
```

## Borrowed & Owned Messages

There are two types which can be used to represent each message from the midi2 standard.
Use the `Borrowed` type to get a 'view' onto the underlying data,
and use the `Owned` type to make a message with an independent lifetime.

```rust
use midi2::prelude::*;

#[cfg(feature = "std")]
#[cfg(feature = "midi2-channel-voice")]
{
    let owned = {
        let buffer = [0x4405_6C07, 0xE1E3_5E92];
        let borrowed = Message::from_data(&buffer).unwrap();
        borrowed.into_owned()
    };

    assert_eq!(owned.data(), &[0x4405_6C07, 0xE1E3_5E92]);
}
```

## Backwards Compatible

Messages can be created from legacy Midi1 byte data.

```rust
use midi2::prelude::*;

#[cfg(feature = "midi1-channel-voice")]
{
    let message = Message::from_byte_data(&[0xAB, 0x60, 0x33]);

    assert_eq!(
        message,
        Message::builder()
            .channel_voice1()
            .key_pressure()
            .channel(u4::new(0xB))
            .note(u7::new(0x60))
            .pressure(u7::new(0x33))
            .build(),
    );

    // data is converted to ump format
    assert_eq!(message.unwrap().data(), &[0x20AB_6033]);
}
```

Serialise ump messages back into Midi1 byte data.

```rust
use midi2::prelude::*;

#[cfg(feature = "midi1-channel-voice")]
assert_eq!(
    Message::from_data(&[0x20AB_6033])
        .unwrap()
        .try_write_byte_data(&mut [0x0; 3])
        .unwrap(),
    &[0xAB, 0x60, 0x33],
);
```

## Pretty Printing

Each message implements debug formatting so that the underlying 
data can be easily read in a hexadecimal format.

```rust
use midi2::prelude::*;

#[cfg(feature = "system-common")]
{
    let message = Message::builder()
        .system_common()
        .song_select()
        .group(u4::new(0xA))
        .song(u7::new(0x4F))
        .build()
        .unwrap();

    assert_eq!(
        format!("{:?}", message),
        "Message(0x1AF34F00)",
    );
}
```

## Midi2 Capability Inquiry message wrappers
Wrappers around the special midi2 Capability Inquiry.
These messages are represented by groups of either midi1 or midi2 
system exclusive messages.

## Allocation Free
The library is entirely `no_std`, which guarantees that 
it will never allocate memory under the hood.
This makes it suitable for use on realtime audio threads
or embedded environments.
