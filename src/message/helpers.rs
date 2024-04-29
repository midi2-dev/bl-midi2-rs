use crate::traits::SysexInternal;

#[cfg(any(feature = "sysex7", feature = "sysex8"))]
pub fn group_from_packet(p: &[u32]) -> crate::numeric_types::u4 {
    use crate::util::BitOps;
    p[0].nibble(1)
}

pub const ERR_INCONSISTENT_GROUPS: &str = "Inconsistent groups across packets";

#[cfg(any(feature = "sysex7", feature = "sysex8"))]
pub fn sysex_group_consistent_groups(buffer: &[u32], stride: usize) -> crate::result::Result<()> {
    use group_from_packet as gfp;
    if buffer
        .chunks_exact(stride)
        .all(|chunk| gfp(chunk) == gfp(buffer))
    {
        Ok(())
    } else {
        Err(crate::error::Error::InvalidData(ERR_INCONSISTENT_GROUPS))
    }
}

#[cfg(any(feature = "ump-stream", feature = "flex-data"))]
pub fn check_flex_data_or_ump_stream_consistent_packet_formats(
    buffer: &[u32],
    format_crumb_index: usize,
) -> crate::result::Result<()> {
    use crate::{error::Error, numeric_types::*, util::BitOps};
    // complete message
    if buffer.len() == 4 && buffer[0].crumb(format_crumb_index) != u2::new(0b00) {
        return Err(Error::InvalidData);
    } else if buffer.len() > 4 {
        // composite message
        let mut packets = buffer.chunks_exact(4).peekable();
        // start
        if packets.next().unwrap()[0].crumb(format_crumb_index) != u2::new(0b01) {
            return Err(Error::InvalidData);
        }

        while let Some(packet) = packets.next() {
            if packets.peek().is_some() {
                // continue
                if packet[0].crumb(format_crumb_index) != u2::new(0b10) {
                    return Err(Error::InvalidData);
                }
            } else {
                // end
                if packet[0].crumb(format_crumb_index) != u2::new(0b11) {
                    return Err(Error::InvalidData);
                }
            }
        }
    }

    Ok(())
}

pub const ERR_SYSEX_EXPECTED_COMPLETE: &str =
    "A one-packet sysex message should have Complete status";
pub const ERR_SYSEX_EXPECTED_BEGIN: &str =
    "The first packet of a multi-packet sysex message should have status Begin";
pub const ERR_SYSEX_EXPECTED_CONTINUE: &str =
    "The packet statuses between first and last should be Continue";
pub const ERR_SYSEX_EXPECTED_END: &str =
    "The last packet of a multi-packet sysex message should have status End";

// assumes that buffer contains valid messages
#[cfg(any(feature = "sysex7", feature = "sysex8"))]
pub fn validate_sysex_group_statuses<
    IsComplete: Fn(crate::numeric_types::u4) -> bool,
    IsBegin: Fn(crate::numeric_types::u4) -> bool,
    IsContinue: Fn(crate::numeric_types::u4) -> bool,
    IsEnd: Fn(crate::numeric_types::u4) -> bool,
>(
    buffer: &[u32],
    is_complete: IsComplete,
    is_begin: IsBegin,
    is_continue: IsContinue,
    is_end: IsEnd,
    stride: usize,
) -> crate::result::Result<()> {
    use crate::{error::Error, util::BitOps};

    let mut iter = buffer.chunks(stride).peekable();
    let first_status = iter.next().unwrap()[0].nibble(2);

    if iter.peek().is_none() {
        if is_complete(first_status) {
            return Ok(());
        } else {
            return Err(Error::InvalidData(ERR_SYSEX_EXPECTED_COMPLETE));
        }
    }

    if !is_begin(first_status) {
        return Err(Error::InvalidData(ERR_SYSEX_EXPECTED_BEGIN));
    }

    while let Some(chunk) = iter.next() {
        let status = chunk[0].nibble(2);
        if iter.peek().is_some() && !is_continue(status) {
            return Err(Error::InvalidData(ERR_SYSEX_EXPECTED_CONTINUE));
        }
        if iter.peek().is_none() && !is_end(status) {
            return Err(Error::InvalidData(ERR_SYSEX_EXPECTED_END));
        }
    }

    Ok(())
}

pub fn try_set_sysex_data<
    B: crate::buffer::Buffer + crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    S: SysexInternal<B>,
    D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
>(
    sysex: &mut S,
    data: D,
) -> core::result::Result<(), crate::error::BufferOverflow> {
    match detail::try_set_sysex_data(sysex, data, |s, sz| s.try_resize(sz)) {
        Err(e) => {
            // if the write failed we reset the message
            // back to zero data
            sysex.try_resize(0)?;
            Err(e)
        }
        Ok(()) => Ok(()),
    }
}

pub fn set_sysex_data<
    B: crate::buffer::Buffer + crate::buffer::BufferMut + crate::buffer::BufferResize,
    S: SysexInternal<B>,
    D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
>(
    sysex: &mut S,
    data: D,
) {
    detail::try_set_sysex_data(sysex, data, |s, sz| {
        s.resize(sz);
        Ok(())
    })
    .expect("Resizable buffers should not fail here")
}

mod detail {
    pub fn try_set_sysex_data<
        B: crate::buffer::Buffer + crate::buffer::BufferMut,
        S: crate::traits::SysexInternal<B>,
        D: core::iter::Iterator<Item = <S as crate::traits::Sysex<B>>::Byte>,
        R: Fn(&mut S, usize) -> core::result::Result<(), crate::error::BufferOverflow>,
    >(
        sysex: &mut S,
        mut data: D,
        resize: R,
    ) -> core::result::Result<(), crate::error::BufferOverflow> {
        let mut running_data_size_estimate: Option<usize> = None;
        let mut written = 0;
        let mut additional_size_for_overflow = 1;
        'main: loop {
            let mut size = match running_data_size_estimate.as_mut() {
                None => {
                    // make an initial estimate
                    let init_size = match data.size_hint() {
                        (_, Some(upper)) => upper,
                        // not the optimal case - could lead to additional copying
                        (lower, None) => lower,
                    };
                    running_data_size_estimate = Some(init_size);
                    init_size
                }
                Some(v) => {
                    // we underestimated.
                    // resize to make more space
                    *v += additional_size_for_overflow;
                    additional_size_for_overflow *= 2;
                    *v
                }
            };

            let mut should_exit = false;
            if let Err(e) = resize(sysex, size) {
                size = sysex.payload_size();
                if size <= written {
                    return Err(e);
                } else {
                    should_exit = true;
                }
            }

            for _ in written..size {
                match data.next() {
                    Some(v) => {
                        sysex.write_datum(v, written);
                        written += 1;
                    }
                    None => {
                        break 'main;
                    }
                }
            }

            if should_exit {
                break;
            }
        }

        if let Some(estimate) = running_data_size_estimate {
            if written < estimate {
                // we shrink the buffer back down to the correct size
                resize(sysex, written)?;
            }
        }

        Ok(())
    }
}
