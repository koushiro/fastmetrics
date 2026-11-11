use std::time::Duration;

use prost::encoding::{self, WireType};

#[inline]
pub(super) fn uint64(buffer: &mut Vec<u8>, tag: u32, value: u64) {
    encoding::uint64::encode(tag, &value, buffer);
}

#[inline]
pub(super) fn int64(buffer: &mut Vec<u8>, tag: u32, value: i64) {
    encoding::int64::encode(tag, &value, buffer);
}

#[inline]
pub(super) fn enumeration(buffer: &mut Vec<u8>, tag: u32, value: i32) {
    encoding::int32::encode(tag, &value, buffer);
}

#[inline]
pub(super) fn double(buffer: &mut Vec<u8>, tag: u32, value: f64) {
    encoding::double::encode(tag, &value, buffer);
}

#[inline]
pub(super) fn string(buffer: &mut Vec<u8>, tag: u32, value: &str) {
    message(buffer, tag, value.as_bytes());
}

#[inline]
pub(super) fn uint64_len(tag: u32, value: u64) -> usize {
    encoding::uint64::encoded_len(tag, &value)
}

#[inline]
pub(super) fn double_len(tag: u32) -> usize {
    encoding::double::encoded_len(tag, &0.0)
}

#[inline]
pub(super) fn message_len(tag: u32, len: usize) -> usize {
    encoding::key_len(tag) + encoding::encoded_len_varint(len as u64) + len
}

#[inline]
pub(super) fn message_header(buffer: &mut Vec<u8>, tag: u32, len: usize) {
    encoding::encode_key(tag, WireType::LengthDelimited, buffer);
    encoding::encode_varint(len as u64, buffer);
}

#[inline]
pub(super) fn message(buffer: &mut Vec<u8>, tag: u32, payload: &[u8]) {
    message_header(buffer, tag, payload.len());
    buffer.extend_from_slice(payload);
}

#[inline]
pub(super) fn delimited(buffer: &mut Vec<u8>, payload: &[u8]) {
    encoding::encode_varint(payload.len() as u64, buffer);
    buffer.extend_from_slice(payload);
}

pub(super) fn timestamp(buffer: &mut Vec<u8>, tag: u32, duration: Duration) {
    let timestamp = prost_types::Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
    };
    encoding::message::encode(tag, &timestamp, buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_headers_match_runtime_encoding_at_length_boundaries() {
        for tag in [1, 15, 16, 2047, 2048, (1 << 29) - 1] {
            for len in [0, 1, 127, 128, 16383, 16384] {
                let payload = vec![0; len];
                let mut expected = Vec::new();
                encoding::bytes::encode(tag, &payload, &mut expected);
                let mut actual = Vec::new();
                message(&mut actual, tag, &payload);
                assert_eq!(actual, expected);
                assert_eq!(actual.len(), message_len(tag, len));
            }
        }
    }
}
