use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;

const OP_CODE: u32 = 0xF2;

#[midi2_attr::generate_message]
struct SongPositionPointer {
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
    position: Property<u14, UmpSchema<0x0000_7F7F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x7F>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            SongPositionPointerMessage::<Ump>::builder(&mut Ump::random_buffer::<4>())
                .group(u4::new(0xA))
                .position(u14::new(0x367D))
                .build(),
            Ok(SongPositionPointerMessage::<Ump>(&[
                0x1AF2_7D6C,
                0x0,
                0x0,
                0x0
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            SongPositionPointerMessage::<Ump>::from_data(&[0x1AF2_7D6C, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn position() {
        assert_eq!(
            SongPositionPointerMessage::<Ump>::from_data(&[0x1AF2_7D6C, 0x0, 0x0, 0x0])
                .unwrap()
                .position(),
            u14::new(0x367D),
        );
    }

    #[test]
    fn bytes_builder() {
        assert_eq!(
            SongPositionPointerMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>())
                .position(u14::new(0x367D))
                .build(),
            Ok(SongPositionPointerMessage::<Bytes>(&[0xF2, 0x7D, 0x6C])),
        );
    }

    #[test]
    fn bytes_position() {
        assert_eq!(
            SongPositionPointerMessage::<Bytes>::from_data(&[0xF2, 0x7D, 0x6C])
                .unwrap()
                .position(),
            u14::new(0x367D),
        );
    }
}
