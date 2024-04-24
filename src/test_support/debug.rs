// use crate::traits::{ByteData as ByteDataTrait, Data as DataTrait};
//
// // dev tool for hex printing of u32 buffers
// // helps to debug on failed test output
// #[derive(midi2_proc::UmpDebug, PartialEq, Eq)]
// pub struct Data<'a>(pub &'a [u32]);
//
// #[derive(midi2_proc::BytesDebug, PartialEq, Eq)]
// pub struct ByteData<'a>(pub &'a [u8]);
//
// impl<'a> DataTrait for Data<'a> {
//     fn data(&self) -> &[u32] {
//         self.0
//     }
// }
//
// impl<'a> ByteDataTrait for ByteData<'a> {
//     fn byte_data(&self) -> &[u8] {
//         self.0
//     }
// }
