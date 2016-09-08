use std::error;
use std::fmt;
use std::mem;
use std::result;

pub type Result<T> = result::Result<T, EncodingError>;

#[derive(Debug)]
pub enum EncodingError {
    BufferTooSmall,
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for EncodingError {
    fn description(&self) -> &str {
        match self {
            &EncodingError::BufferTooSmall => "supplied buffer too small",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            _ => None,
        }
    }
}

pub const MAX_UVARINT_LEN: u32 = 9;

pub fn encode_uvarint_into(value: u64, buf: &mut [u8]) -> Result<usize> {
    let buf_len = buf.len();
    if buf_len < 2 {
        return Err(EncodingError::BufferTooSmall);
    }

    if value == 0 {
        buf[0] = 1;
        buf[1] = 0;
        return Ok(2);
    }

    let bits_per_byte = 8usize;
    let size_of_u64 = mem::size_of::<u64>();

    let leading_zero_bytes = (value.leading_zeros() as usize) / bits_per_byte;
    let size = size_of_u64 - leading_zero_bytes;
    if buf_len < size + 1 {
        return Err(EncodingError::BufferTooSmall);
    }

    buf[0] = size as u8;

    for idx in 0..size {
        let mask_shifts = bits_per_byte * (size - idx - 1);
        let mask = 0xFF << mask_shifts;
        buf[idx + 1] = ((value & mask) >> mask_shifts) as u8;
    }

    Ok(size + 1)
}

pub fn decode_uvarint(buf: &[u8]) -> Result<(u64, usize)> {
    let buf_len = buf.len();
    if buf_len < 2 {
        return Err(EncodingError::BufferTooSmall);
    }

    let size = buf[0];
    if buf_len < (size + 1) as usize {
        return Err(EncodingError::BufferTooSmall);
    }

    let mut value = 0u64;
    for idx in 1..size + 1 {
        value <<= 8;
        value += buf[idx as usize] as u64;
    }

    Ok((value, (size + 1) as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_uvarint_into() {
        let mut buf: [u8; 2] = [0; 2];
        assert_eq!(encode_uvarint_into(1, &mut buf).unwrap(), 2);
        assert_eq!(buf, [1, 1]);

        // Should return an error if the buffer is too small.
        encode_uvarint_into(0x01_FF, &mut buf).unwrap_err();

        let mut buf: [u8; 5] = [0; 5];
        assert_eq!(encode_uvarint_into(0xAB_CD_EF_01, &mut buf).unwrap(),
                   5);
        assert_eq!(buf, [4, 0xAB, 0xCD, 0xEF, 0x01]);
    }

    #[test]
    fn test_decode_uvarint() {
        assert_eq!(decode_uvarint(&[1, 1]).unwrap(), (1, 2 as usize));
        assert_eq!(decode_uvarint(&[4, 0xAB, 0xCD, 0xEF, 0x01]).unwrap(),
                   (0xAB_CD_EF_01, 5 as usize));
    }
}
