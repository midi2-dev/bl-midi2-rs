// pub mod delta_clock_stamp;
pub mod no_op;
// pub mod time_stamp;

const UMP_MESSAGE_TYPE: u8 = 0x0;

// #[derive(
//     midi2_proc::UmpDebug,
//     derive_more::From,
//     midi2_proc::Data,
//     midi2_proc::Grouped,
//     Clone,
//     PartialEq,
//     Eq,
// )]
// #[non_exhaustive]
// pub enum UtilityMessage<'a> {
//     NoOp(NoOpMessage<'a>),
//     TimeStamp(TimeStampMessage<'a>),
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pretty_assertions::assert_eq;
//
//     #[test]
//     fn builder() {
//         assert_eq!(
//             UtilityMessage::builder()
//                 .time_stamp()
//                 .time_stamp(u20::new(0x1))
//                 .build(),
//             Ok(UtilityMessage::TimeStamp(
//                 TimeStampMessage::builder()
//                     .time_stamp(u20::new(0x1))
//                     .build()
//                     .unwrap()
//             ))
//         )
//     }
// }
