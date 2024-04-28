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

pub fn replace_sysex_payload_range<
    B,
    S: SysexInternal<ByteType = B>,
    D: core::iter::Iterator<Item = B>,
    R: core::ops::RangeBounds<usize> + core::iter::Iterator<Item = usize>,
>(
    builder: &mut S,
    data: D,
    range: R,
) {
    let range_start = match range.start_bound() {
        core::ops::Bound::Unbounded => 0,
        core::ops::Bound::Included(&v) => v,
        core::ops::Bound::Excluded(&v) => v + 1,
    };
    let range_end = match range.end_bound() {
        core::ops::Bound::Unbounded => builder.payload_size(),
        core::ops::Bound::Included(&v) => v + 1,
        core::ops::Bound::Excluded(&v) => v,
    };
    if range_start > builder.payload_size() {
        // the requested range is invalid
        // grow the payload to fit
        builder.shift_tail_forward(builder.payload_size(), range_end - builder.payload_size())
    }
    let mut start_index_of_following_data = {
        let data_size_estimate = match data.size_hint() {
            (_, Some(upper)) => upper,
            (lower, None) => {
                // not the optimal case - could lead to additional copying
                lower
            }
        };
        if range_start + data_size_estimate < range_end {
            // we have room for the new data
            range_end
        } else {
            // we make room for the new data
            let distance = range_start + data_size_estimate - range_end;
            builder.shift_tail_forward(range_end, distance);
            range_end + distance
        }
    };

    // we write the data
    let mut last_index_written = 0;
    let mut shift_for_overflow_distance = 1;
    for (i, d) in (range_start..).zip(data) {
        if i >= start_index_of_following_data {
            // unplanned tail shifting!
            builder.shift_tail_forward(start_index_of_following_data, shift_for_overflow_distance);
            start_index_of_following_data += shift_for_overflow_distance;
            shift_for_overflow_distance *= 2;
        }
        builder.write_datum(d, i);
        last_index_written = i;
    }

    if last_index_written + 1 < start_index_of_following_data {
        // we shrink back down to fit the final payload size
        builder.shift_tail_backward(
            start_index_of_following_data,
            start_index_of_following_data - last_index_written - 1,
        );
    }
}
