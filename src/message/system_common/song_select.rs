use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;

const OP_CODE: u32 = 0xF3;

#[midi2_attr::generate_message]
struct SongSelect {
    ump_type: Property<
        NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<
        NumericalConstant<OP_CODE>,
        UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
        BytesSchema<0xFF, 0x0, 0x0>,
    >,
    song: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            SongSelectMessage::<Ump>::builder(&mut Ump::random_buffer::<4>())
                .group(u4::new(0xA))
                .song(u7::new(0x4F))
                .build(),
            Ok(SongSelectMessage::<Ump>(&[0x1AF3_4F00, 0x0, 0x0, 0x0])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            SongSelectMessage::<Ump>::from_data(&[0x1AF3_4F00, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn song() {
        assert_eq!(
            SongSelectMessage::<Ump>::from_data(&[0x1AF3_4F00, 0x0, 0x0, 0x0])
                .unwrap()
                .song(),
            u7::new(0x4F),
        );
    }

    #[test]
    fn bytes_builder() {
        assert_eq!(
            SongSelectMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>())
                .song(u7::new(0x4F))
                .build(),
            Ok(SongSelectMessage::<Bytes>(&[0xF3, 0x4F, 0x00])),
        );
    }

    #[test]
    fn bytes_song() {
        assert_eq!(
            SongSelectMessage::<Bytes>::from_data(&[0xF3, 0x4F, 0x00])
                .unwrap()
                .song(),
            u7::new(0x4F),
        );
    }
}
