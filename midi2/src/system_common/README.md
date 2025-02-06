MIDI System Common and System Real Time messages.

## Abstract over [Buffer](crate::buffer::Buffer)

System Common and System Real Time messages can be represented
with classical MIDI byte arrays or with the MIDI 2.0 Universal 
Message Packet formats.

As such, types in this module are abstract over [Buffer](crate::buffer::Buffer).

When backed by a buffer with `Unit = u8` the underlying data is in the
byte stream format, and is backwards compatible with classical MIDI
standards.

```rust
use midi2::prelude::*;

let mut message = system_common::SongSelect::<[u8; 3]>::new();
message.set_song(u7::new(0x42));
assert_eq!(message.data(), &[0xF3, 0x42]);
```

And when backed by a buffer with `Unit = u32` the underlying data is 
encoded into Universal Message Packets.

```rust
use midi2::prelude::*;

let mut message = system_common::SongSelect::<[u32; 4]>::new();
message.set_song(u7::new(0x42));
message.set_group(u4::new(0x3));
assert_eq!(message.data(), &[0x13F3_4200]);
```

## Fixed Size

`system_common` messages always fit within an 
array of 'u32' size 1 or greater, when represented 
with a [Ump](crate::buffer::Ump) buffer.

```rust
use midi2::system_common::SongSelect;

let _ = SongSelect::<[u32; 1]>::new();
let _ = SongSelect::<[u32; 4]>::new();
```

`system_common` messages always fit within an 
array of 'u8' size 3 or greater, when represented 
with a [Bytes](crate::buffer::Bytes) buffer.

```rust
use midi2::system_common::SongSelect;

let _ = SongSelect::<[u8; 3]>::new();
```
