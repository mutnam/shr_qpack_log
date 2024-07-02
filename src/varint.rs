use bytes::{Buf, BufMut};

use crate::error::DecodeError;

pub fn encode<B: BufMut>(b: &mut B,u:u64){
    if u<64{//2的6次方
        b.put_u8(u as u8);
    }else if u< 16384{//2的14次方
        b.put_u16(0b01 << 14 | u as u16);
    }else if u < 1073741824 {//2的30次方
        b.put_u32(0b10 << 30 | u as u32);
    }else if u < 4611686018427387904 {//2的62次方
        b.put_u64(0b10 << 62 | u as u64);
    }else{
        unreachable!("varint编码不在范围")
    }
}
pub fn decode<B: Buf>(buf: &mut B) -> Result<u64,DecodeError>{
    let byte1 = buf.get_u8();
   // println!("varintlen{},第一个字节{},二进制:{:08b}",msg,byte1,byte1);
    let index2 = byte1 >> 6;
   // println!("varintlen{},第一个字节首部前两位{:02b}",msg,index2);
    let byte1_new = byte1 & 0b0011_1111;
   // println!("varintlen{},第一个字节末端后6位{:08b}",msg,byte1_new);
    match index2 {
        0b00 => {
           // println!("varintlen{},长度为一节00:{:08b},值:{}",msg,byte1_new,byte1_new);
            return Ok(byte1_new as u64);
        }
        0b01 => {
            let v = [byte1_new, buf.get_u8()];
           // println!("varintlen{},长度为二节01,值:{:?}",msg,v);
            return Ok(u16::from_be_bytes(v) as u64);
        }
        0b10 => {
            let v = [byte1_new, buf.get_u8(), buf.get_u8(), buf.get_u8()];
           // println!("varintlen{},长度为四节10,值:{:?}",msg,v);
            return Ok(
                u32::from_be_bytes(v) as u64,
            );
        }
        0b11 => {
            let mut buf_a = [byte1_new, 0, 0, 0, 0, 0, 0, 0];
            buf.copy_to_slice(&mut buf_a[1..8]);
           // println!("varintlen{},长度为八节10,值:{:?}",msg,buf_a);
            return Ok(u64::from_be_bytes(buf_a));
        }
        _ => {
            return Err(DecodeError::new("可变整数类型", "可变整数只支持前缀为0b00,0b01,0b10,0b11"));
        }
    }
}