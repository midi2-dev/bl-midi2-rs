# System Exclusive 7Bit

A semantic wrapper type around midi2 system exclusive 7bit data.

## Abstract over [Buffer](crate::buffer::Buffer)

Use it with a [Ump](crate::buffer::Ump) buffer.

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

Or use it with a [Bytes](crate::buffer::Bytes) buffer.

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

## Borrowed workflow

You can create a [Sysex7](crate::message::sysex7::Sysex7) from
borrowed data and avoid copying or allocating if you have the data already existing
in a buffer.

```rust
use midi2::prelude::*;

let buffer = [
    0x3416_0001_u32,
    0x0203_0405_u32,
    0x3426_0607_u32,
    0x0809_0A0B_u32,
    0x3433_0C0D_u32,
    0x0E00_0000_u32,
];

let borrowed = sysex7::Sysex7::try_from(&buffer[..])
    .expect("Data is valid");

assert_eq!(
    borrowed.data(),
    &[
        0x3416_0001_u32,
        0x0203_0405_u32,
        0x3426_0607_u32,
        0x0809_0A0B_u32,
        0x3433_0C0D_u32,
        0x0E00_0000_u32,
    ],
);

// Borrowed messages are immutable and their liftimes are
// tied to the original buffer. 
//
// To create an owned version use the `Rebuffer` traits.

let mut owned: sysex7::Sysex7::<Vec<u32>> = borrowed.rebuffer_into();
owned.set_group(u4::new(0x5));

assert_eq!(
    owned.data(),
    &[
        0x3516_0001_u32,
        0x0203_0405_u32,
        0x3526_0607_u32,
        0x0809_0A0B_u32,
        0x3533_0C0D_u32,
        0x0E00_0000_u32,
    ],
);
```

## Fixed size buffers

Use with fixed size, or fallible buffers.

```rust
use midi2::prelude::*;

let mut message = sysex7::Sysex7::<[u8; 22]>::try_new()
    .expect("Buffer is large enough");

// only fallible methods are available
assert_eq!(message.try_set_payload((0u8..20u8).map(u7::new)), Ok(()));
assert_eq!(
    message.data(), 
    &[
        0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
        0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
    ],
);

// setting payloads larger than the available size will fail
assert_eq!(
    message.try_set_payload((0u8..30u8).map(u7::new)),
    Err(midi2::BufferOverflow),
);
assert_eq!(message.data(), &[0xF0, 0xF7]);
```
