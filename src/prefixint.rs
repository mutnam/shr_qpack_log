use bytes::Buf;

use crate::error::DecodeError;


pub fn decode<B: Buf>(buf: &mut B,size: u8) -> Result<usize,DecodeError> {
    let byte = buf.get_u8();
    let mut pre_len: usize = match size {
        8 => {
            if byte & 0b1111_1111 == 0b1111_1111 {
                0b1111_1111 as usize
            } else {
                return Ok((byte & 0b1111_1111) as usize);
            }
        }
        7 => {
            if byte & 0b0111_1111 == 0b0111_1111 {
                0b0111_1111 as usize
            } else {
                return Ok((byte & 0b0111_1111) as usize);
            }
        }
        6 => {
            if byte & 0b0011_1111 == 0b0011_1111 {
                0b0011_1111 as usize
            } else {
                return Ok((byte & 0b0011_1111) as usize);
            }
        }
        5 => {
            if byte & 0b0001_1111 == 0b0001_1111 {
                0b0001_1111 as usize
            } else {
                return Ok((byte & 0b0001_1111) as usize);
            }
        }
        4 => {
            if byte & 0b0000_1111 == 0b0000_1111 {
                0b0000_1111 as usize
            } else {
                return Ok((byte & 0b0000_1111) as usize);
            }
        }
        3 => {
            if byte & 0b0000_0111 == 0b0000_0111 {
                0b0000_0111 as usize
            } else {
                return Ok((byte & 0b0000_0111) as usize);
            }
        }
        2 => {
            if byte & 0b0000_0011 == 0b0000_0011 {
                0b0000_0011 as usize
            } else {
                return Ok((byte & 0b0000_0011) as usize);
            }
        }
        _ => {
            return Err(DecodeError::new("prefixint", "size在2..=8之间"));
        }
    };
    let mut power = 0usize;
    loop {
        let byte1 = buf.get_u8() as usize;
        pre_len+=(byte1 &127)<<power;
        power +=7;
        if byte1 & 128 == 128 {
            break;
        }
    }
    return Ok(pre_len);
}


pub fn decode_byte<B: Buf>(buf: &mut B, byte: u8, size: u8) -> Result<usize,DecodeError> {
    let mut pre_len: usize = match size {
        8 => {
            if byte & 0b1111_1111 == 0b1111_1111 {
                0b1111_1111 as usize
            } else {
                return Ok((byte & 0b1111_1111) as usize);
            }
        }
        7 => {
            if byte & 0b0111_1111 == 0b0111_1111 {
                0b0111_1111 as usize
            } else {
                return Ok((byte & 0b0111_1111) as usize);
            }
        }
        6 => {
            if byte & 0b0011_1111 == 0b0011_1111 {
                0b0011_1111 as usize
            } else {
                return Ok((byte & 0b0011_1111) as usize);
            }
        }
        5 => {
            if byte & 0b0001_1111 == 0b0001_1111 {
                0b0001_1111 as usize
            } else {
                return Ok((byte & 0b0001_1111) as usize);
            }
        }
        4 => {
            if byte & 0b0000_1111 == 0b0000_1111 {
                0b0000_1111 as usize
            } else {
                return Ok((byte & 0b0000_1111) as usize);
            }
        }
        3 => {
            if byte & 0b0000_0111 == 0b0000_0111 {
                0b0000_0111 as usize
            } else {
                return Ok((byte & 0b0000_0111) as usize);
            }
        }
        2 => {
            if byte & 0b0000_0011 == 0b0000_0011 {
                0b0000_0011 as usize
            } else {
                return Ok((byte & 0b0000_0011) as usize);
            }
        }
        _ => {
            return Err(DecodeError::new("prefixint", "size在2..=8之间"));
        }
    };
    let mut power = 0usize;
    loop {
        let byte1 = buf.get_u8() as usize;
        pre_len+=(byte1 & 127) << power;
        power +=7;
        if byte1 & 128 == 0 {
            break;
        }
    }
    return Ok(pre_len);
}

