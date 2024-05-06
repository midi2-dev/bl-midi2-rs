use crate::message::{common_properties, ump_stream, ump_stream::UMP_MESSAGE_TYPE};

pub(crate) const STATUS: u16 = 0x0;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(4))]
struct EndpointName {
    #[property(crate::message::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::message::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(ump_stream::TextWriteStrProperty)]
    #[writeonly]
    #[resize]
    name: &str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = EndpointName::new();
        message.set_name("Gimme some signal 🔊 🙌");
        assert_eq!(
            message,
            EndpointName(std::vec![
                0x0, //jr
                0xF403_4769,
                0x6D6D_6520,
                0x736F_6D65,
                0x2073_6967,
                0xFC03_6E61,
                0x6C20_F09F,
                0x948A_20F0,
                0x9F99_8C00,
            ]),
        );
    }

    // #[test]
    // fn borrowed_builder() {
    //     assert_eq!(
    //         debug::Data(
    //             EndpointNameBorrowed::builder(&mut Ump::random_buffer::<8>())
    //                 .name("Gimme some signal 🔊 🙌")
    //                 .build()
    //                 .unwrap()
    //                 .data()
    //         ),
    //         debug::Data(&[
    //             0xF403_4769,
    //             0x6D6D_6520,
    //             0x736F_6D65,
    //             0x2073_6967,
    //             0xFC03_6E61,
    //             0x6C20_F09F,
    //             0x948A_20F0,
    //             0x9F99_8C00,
    //         ]),
    //     );
    // }
    //
    // #[test]
    // #[cfg(feature = "std")]
    // fn name() {
    //     assert_eq!(
    //         EndpointNameMessage::from_data(&[
    //             0xF403_4769,
    //             0x6D6D_6520,
    //             0x736F_6D65,
    //             0x2073_6967,
    //             0xFC03_6E61,
    //             0x6C20_F09F,
    //             0x948A_20F0,
    //             0x9F99_8C00,
    //         ])
    //         .unwrap()
    //         .name(),
    //         Ok(std::string::String::from("Gimme some signal 🔊 🙌")),
    //     );
    //     assert_eq!(
    //         EndpointNameMessage::from_data(&[
    //             0xF403_4769,
    //             0x6D6D_6520,
    //             0x736F_6D65,
    //             0x206D_6F72,
    //             0xF803_6520,
    //             0x7369_676E,
    //             0x616C_20F0,
    //             0x9F94_8A20,
    //             0xFC03_F09F,
    //             0x998C_0000,
    //             0x0000_0000,
    //             0x0000_0000,
    //         ])
    //         .unwrap()
    //         .name(),
    //         Ok(std::string::String::from("Gimme some more signal 🔊 🙌")),
    //     );
    // }
    //
    // #[test]
    // #[cfg(feature = "std")]
    // fn name_bytes() {
    //     assert_eq!(
    //         EndpointNameMessage::from_data(&[
    //             0xF403_4769,
    //             0x6D6D_6520,
    //             0x736F_6D65,
    //             0x2073_6967,
    //             0xFC03_6E61,
    //             0x6C20_F09F,
    //             0x948A_20F0,
    //             0x9F99_8C00,
    //         ])
    //         .unwrap()
    //         .name_bytes()
    //         .collect::<std::vec::Vec<u8>>(),
    //         std::vec![
    //             0x47, 0x69, 0x6D, 0x6D, 0x65, 0x20, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x69, 0x67,
    //             0x6E, 0x61, 0x6C, 0x20, 0xF0, 0x9F, 0x94, 0x8A, 0x20, 0xF0, 0x9F, 0x99, 0x8C,
    //         ],
    //     );
    // }
}
