pub trait SysexMessage: Sized {
    fn group(&self) -> ux::u4;
    fn set_group(&mut self, group: ux::u4);
    fn datum(&self, i: usize) -> u8;
    fn set_datum(&mut self, d: u8, i: usize);
    fn len(&self) -> usize;
    fn max_len() -> usize;
    fn status(&self) -> Status;
    fn set_status(&mut self, status: Status);
}

#[derive(PartialEq, Eq)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    End,
}

pub struct SysexMessages<'a, M: SysexMessage>(pub &'a [M]);

impl<'a, M: SysexMessage> SysexMessages<'a, M> {
    pub fn valid(&self) -> bool {
        messages_valid(self.0)
    }
    pub fn len(&self) -> usize {
        self.0.iter().fold(0, |sum, m| sum + m.len())
    }
    pub fn max_len(&self) -> usize {
        M::max_len() * self.0.len()
    }
    pub fn group(&self) -> ux::u4 {
        if self.0.is_empty() {
            panic!()
        }
        self.0[0].group()
    }
    pub fn datum(&self, i: usize) -> u8 {
        // todo
        // optimise this index function
        let mut running_index = 0_usize;
        for m in self.0 {
            if running_index + m.len() > i {
                return m.datum(i - running_index);
            } else {
                running_index += m.len();
            }
        }
        unreachable!()
    }
}

pub struct SysexMessagesMut<'a, M: SysexMessage>(pub &'a mut [M]);

impl<'a, M: SysexMessage> SysexMessagesMut<'a, M> {
    pub fn builder(buffer: &'a mut [M], group: ux::u4) -> SysexMessagesMutBuilder<'a, M> {
        SysexMessagesMutBuilder::new(buffer, group)
    }
    pub fn _valid(&self) -> bool {
        messages_valid(self.0)
    }
}

fn messages_valid<M: SysexMessage>(messages: &[M]) -> bool {
    let statuses_correct = match messages.len() {
        0 => true,
        1 => messages[0].status() == Status::Complete,
        l => {
            let mut ret = true;
            ret &= messages[0].status() == Status::Begin;
            for m in &messages[1..l - 1] {
                ret &= m.status() == Status::Continue;
            }
            ret &= messages[l - 1].status() == Status::End;
            ret
        }
    };
    let groups_consistent = messages.windows(2).all(|w| w[0].group() == w[1].group());
    groups_consistent && statuses_correct
}

pub struct SysexMessagesMutBuilder<'a, M: SysexMessage> {
    group: ux::u4,
    buffer: &'a mut [M],
    message_index: usize,
    data_index: usize,
}

impl<'a, M: SysexMessage> SysexMessagesMutBuilder<'a, M> {
    pub fn new(buffer: &'a mut [M], group: ux::u4) -> Self {
        SysexMessagesMutBuilder {
            group,
            buffer,
            message_index: 0,
            data_index: 0,
        }
    }
    pub fn datum(&mut self, d: u8) {
        if self.message_index == 0 && self.data_index == 0 {
            self.buffer[self.message_index].set_status(Status::Complete);
            self.buffer[self.message_index].set_group(self.group);
        }
        if self.data_index == M::max_len() {
            if self.buffer[self.message_index].status() == Status::Complete {
                self.buffer[self.message_index].set_status(Status::Begin);
            } else {
                self.buffer[self.message_index].set_status(Status::Continue);
            }
            self.message_index += 1;
            self.data_index = 0;
            self.buffer[self.message_index].set_status(Status::End);
            self.buffer[self.message_index].set_group(self.group);
        }
        self.buffer[self.message_index].set_datum(d, self.data_index);
        self.data_index += 1;
    }
    pub fn build(self) -> SysexMessagesMut<'a, M> {
        SysexMessagesMut(&mut self.buffer[..self.message_index + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::system_exclusive_8bit as sysex8;

    #[test]
    fn len() {
        assert_eq!(
            SysexMessages(&[
                sysex8::Message::builder()
                    .group(ux::u4::new(0x0))
                    .stream_id(0x0)
                    .data(&[0x0, 0x0, 0x0])
                    .status(sysex8::Status::Begin)
                    .build(),
                sysex8::Message::builder()
                    .group(ux::u4::new(0x0))
                    .stream_id(0x0)
                    .data(&[0x0, 0x0, 0x0, 0x0])
                    .status(sysex8::Status::Continue)
                    .build(),
                sysex8::Message::builder()
                    .group(ux::u4::new(0x0))
                    .stream_id(0x0)
                    .data(&[0x0])
                    .status(sysex8::Status::End)
                    .build(),
            ])
            .len(),
            8,
        );
    }

    #[test]
    fn datum() {
        let messages = [
            sysex8::Message::builder()
                .group(ux::u4::new(0x0))
                .stream_id(0x0)
                .data(&[0x0, 0x0, 0x1])
                .status(sysex8::Status::Begin)
                .build(),
            sysex8::Message::builder()
                .group(ux::u4::new(0x0))
                .stream_id(0x0)
                .data(&[0x0, 0x2, 0x0, 0x0])
                .status(sysex8::Status::Continue)
                .build(),
            sysex8::Message::builder()
                .group(ux::u4::new(0x0))
                .stream_id(0x0)
                .data(&[0x3])
                .status(sysex8::Status::End)
                .build(),
        ];
        let messages = SysexMessages(&messages);
        assert_eq!(messages.datum(2), 0x1);
        assert_eq!(messages.datum(4), 0x2);
        assert_eq!(messages.datum(7), 0x3);
    }
}
