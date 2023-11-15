const OP_CODE: u32 = 0b1010;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct KeyPressure {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    key_pressure_data: Property<u32, UmpSchema<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            KeyPressureOwned::builder()
                .group(u4::new(0xB))
                .channel(u4::new(0xC))
                .note(u7::new(0x59))
                .key_pressure_data(0xC0B83064)
                .build(),
            Ok(KeyPressureOwned(arr![0x4BAC_5900, 0xC0B83064, 0x0, 0x0])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            KeyPressureBorrowed::<Ump>::from_data(&[0x4BAC_5900, 0xC0B83064, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xB),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            KeyPressureBorrowed::<Ump>::from_data(&[0x4BAC_5900, 0xC0B83064, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0xC),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            KeyPressureBorrowed::<Ump>::from_data(&[0x4BAC_5900, 0xC0B83064, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x59),
        );
    }

    #[test]
    fn key_pressure_data() {
        assert_eq!(
            KeyPressureBorrowed::<Ump>::from_data(&[0x4BAC_5900, 0xC0B83064, 0x0, 0x0])
                .unwrap()
                .key_pressure_data(),
            0xC0B83064,
        );
    }
}
