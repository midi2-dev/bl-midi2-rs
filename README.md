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

```rust
use midi2::prelude::*;

let message = MessageOwned::builder()
    .midi2_channel_voice()
    .note_on()
    .note(u7::new(0x60))
    .velocity(0x4B57)
    .build()
    .unwrap();

assert_eq!(message.data(), &[0x4090_6000, 0x4B57_0000, 0x0, 0x0]);
```

### Borrowed & Owned Messages

There are two types which can be used to represent each message from the midi2 standard.
Use the `Borrowed` type to get a 'view' onto the underlying data,
and use the `Owned` type to make a message with an independent lifetime.

#### Example

```rust
use midi2::prelude::*;

let owned = {
    let buffer = [0x4405_6C07, 0xE1E3_5E92, 0x0, 0x0];
    let borrowed = MessageBorrowed::from_data(&buffer).unwrap();
    borrowed.to_owned()
};

assert_eq!(owned.data(), &[0x4405_6C07, 0xE1E3_5E92, 0x0, 0x0]);
```


### Midi2 Capability Inquiry message wrappers
Wrappers around the special midi2 Capability Inquiry.
These messages are represented by groups of either midi1 or midi2 
system exclusive messages.

### `#![no_std]`
The library is entirely no_std, which guarantees that 
it will never allocate memory under the hood.
This makes it suitable for use on realtime audio threads
or embedded environments.
