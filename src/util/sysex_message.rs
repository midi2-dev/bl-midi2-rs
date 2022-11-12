use crate::message::{
    system_exclusive_7bit,
};

pub trait SysexMessage : Sized {
    fn set_group(&mut self, group: ux::u4);
    fn set_datum(&mut self, d: u8, i: usize);
    fn max_len() -> usize;
    fn is_complete(&self) -> bool;
    fn set_complete(&mut self);
    fn set_start(&mut self);
    fn set_continue(&mut self);
    fn set_end(&mut self);
}

impl SysexMessage for system_exclusive_7bit::Message {
    fn set_group(&mut self, _group: ux::u4) {
        todo!()
    }
    fn set_datum(&mut self, _d: u8, _i: usize) {
        todo!()
    }
    fn max_len() -> usize {
        6
    }
    fn is_complete(&self) -> bool {
        todo!()
    }
    fn set_complete(&mut self) {
        todo!()
    }
    fn set_start(&mut self) {
        todo!()
    }
    fn set_continue(&mut self) {
        todo!()
    }
    fn set_end(&mut self) {
        todo!()
    }
}

pub struct SysexMessages<'a, M: SysexMessage>{
    group: ux::u4,
    buffer: &'a mut [M],
    message_index: usize,
    data_index: usize,
}

impl<'a, M: SysexMessage> SysexMessages<'a, M> {
    pub fn new(buffer: &'a mut [M], group: ux::u4) -> Self {
        SysexMessages { 
            group,
            buffer,
            message_index: 0,
            data_index: 0,
        }
    }
    pub fn set_datum(&mut self, d: u8) {
        if self.message_index == 0 && self.data_index == 0 {
            self.buffer[self.message_index].set_complete();
            self.buffer[self.message_index].set_group(self.group);
        }
        if self.data_index == M::max_len() {
            if self.buffer[self.message_index].is_complete() {
                self.buffer[self.message_index].set_start();
            } else {
                self.buffer[self.message_index].set_continue();
            }
            self.message_index += 1;
            self.data_index = 0;
            self.buffer[self.message_index].set_end();
            self.buffer[self.message_index].set_group(self.group);
        } 
        self.buffer[self.message_index].set_datum(d, self.data_index);
        self.data_index += 1;
    }
    pub fn messages(self) -> &'a mut [M] {
        &mut self.buffer[..self.message_index + 1]
    }
}