// Copied from bincode source
// https://github.com/bincode-org/bincode/blob/e0ac3245162ba668ba04591897dd88ff5b3096b8/src/config/int.rs

use bytes::{Buf, BufMut, BytesMut};

const SINGLE_BYTE_MAX: u8 = 250;
const U16_BYTE: u8 = 251;
const U32_BYTE: u8 = 252;
const U64_BYTE: u8 = 253;

pub fn get_varint_le(src: &mut BytesMut) -> Result<u64, anyhow::Error> {
    let discriminant = src.get_u8();

    let out = match discriminant {
        byte @ 0..=SINGLE_BYTE_MAX => byte as u64,
        U16_BYTE => src.get_u16_le() as u64,
        U32_BYTE => src.get_u32_le() as u64,
        U64_BYTE => src.get_u64_le() as u64,
        _ => anyhow::bail!("Invalid discriminant = {}", discriminant),
    };

    Ok(out)
}

pub fn put_varint_le(src: &mut BytesMut, val: u64) {
    if val <= SINGLE_BYTE_MAX as u64 {
        src.put_u8(val as u8);
    } else if val <= u16::MAX as u64 {
        src.put_u8(U16_BYTE);
        src.put_u16_le(val as u16);
    } else if val <= u32::MAX as u64 {
        src.put_u8(U32_BYTE);
        src.put_u32_le(val as u32);
    } else {
        src.put_u8(U64_BYTE);
        src.put_u64_le(val);
    }
}
