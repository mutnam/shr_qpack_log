
use bytes::{Buf, Bytes};
use error::DecodeError;
use table::FIELD_TABLE;



pub mod varint;
pub mod prefixint;
pub mod table;
pub mod octets;
pub mod huffman;
pub mod error;

#[derive(Debug)]
pub struct HeaderFrame{
    frame_type:Option<u64>,
    bytes:Option<Vec<u8>>,
    length:Option<u64>,
    field_section:Option<FieldSection>
}


#[derive(Debug)]
pub struct FieldSection{
    required_insert_count :Option<usize>,
    bytes:Option<Vec<u8>>,
    sign:Option<u8>,
    delta_base:Option<usize>,
    field_lines:Vec<Line>
}
impl FieldSection {
    fn new(b:&mut Bytes)->Result<Self,DecodeError>{
        let required_insert_count =prefixint::decode(b, 8)?;
        let byte1 = b.get_u8();
        let sign = byte1 >> 7;
        let delta_base = prefixint::decode_byte(b, byte1, 7)?;
        let b_c = b.clone().to_vec();
        Ok(Self {
            required_insert_count: Some(required_insert_count),
            bytes: Some(b_c),
            sign: Some(sign),
            delta_base: Some(delta_base),
            field_lines: Vec::new(),
        })
    }
}
#[derive(Debug)]
pub enum Line {
    IndexedFieldLine{prefix:String,t:u8,index:usize,value:String},
    IndexedFieldLinewithPostBaseIndex{prefix:String,index:usize},
    LiteralFieldLinewithNameReference{prefix:String,n:u8,t:u8,name_index:usize,name_value:String,h:u8,value_length:usize,value_string:String},
    LiteralFieldLinewithPostBaseNameReference{prefix:String,n:u8,name_id:usize,h:u8,value_length:usize,value_string:String},
    LiteralFieldLinewithLiteralName{prefix:String,n:u8,name_h:u8,name_lenght:usize,name_string:String,value_h:u8,value_length:usize,value_string:String}
}
pub struct Lines(Vec<Line>);
impl Lines {
    pub fn new()->Self{
        Self(Vec::new())
    }
    pub fn build(&mut self,b:&mut Bytes)->Result<(),DecodeError>{
        loop {
            if b.has_remaining() {
                self.line(b)?; 
            }else{
                break;
            }
        }
        Ok(())
    }
    fn line(&mut self,b:&mut Bytes)->Result<(),DecodeError>{
        let byte = b.get_u8();
        if byte >> 7 == 0b1 {
            self.line_index(b,byte)?;
        } else if byte >> 4 == 0b0001 {
            self.line_base_index(b,byte)?;
        } else if byte >> 6 == 0b01 {
            self.line_name_ref(b,byte)?;
        } else if byte >> 4 == 0b0000 {
            self.line_base_name_ref(b,byte)?;
        } else if byte >> 5 == 0b001 {
            self.line_name(b,byte)?;
        } else {
            return Err(DecodeError::new("line", "不支持类型"));
        }
        Ok(())
    }
    fn line_index(&mut self,b:&mut Bytes, byte0: u8) -> Result<(), DecodeError> {
        let t = (byte0 >> 6) & 0b01;
        let name_index = prefixint::decode_byte(b, byte0, 6)?;
        let table = match t {
            1=>{
                FIELD_TABLE.get(name_index).ok_or(DecodeError::new("静态表", "查找失败"))?
            },
            _=>{
                &(0,"","",0)
            }
        };
        self.0.push(Line::IndexedFieldLine { prefix: "1".to_string(), t, index: name_index, value:format!("{}={}",table.1,table.2)});
        Ok(())
    }
    fn line_base_index(&mut self,b:&mut Bytes, byte0: u8) -> Result<(), DecodeError> {
        let name_index = prefixint::decode_byte(b, byte0, 4)?;
        self.0.push(Line::IndexedFieldLinewithPostBaseIndex { prefix: "0001".to_string(), index: name_index });
        Ok(())
    }
    fn line_name_ref(&mut self,b:&mut Bytes, byte0: u8) -> Result<(), DecodeError> {
        let n = (byte0 & 0b0010_0000) >> 5;
        let t = (byte0 & 0b0001_0000) >> 4;
        let name_index = prefixint::decode_byte(b, byte0, 4)?;
        let table = match t {
            1=>{
                FIELD_TABLE.get(name_index).ok_or(DecodeError::new("静态表", "查找失败"))?
            },
            _=>{
                &(0,"","",0)
            }
        };
        let (h,l,value_string) = str_decode(b)?;
        self.0.push(Line::LiteralFieldLinewithNameReference { prefix: "01".to_string(), n, t, name_index, h, value_length:l, value_string, name_value: format!("{}",table.1) });
        Ok(())
    }
    fn line_base_name_ref(&mut self,b:&mut Bytes, byte0: u8) -> Result<(), DecodeError> {
        let n = (byte0 & 0b0000_1000) >> 3;
        let name_id = prefixint::decode_byte(b, byte0, 3)?;
        let (h,l,value_string) = str_decode(b)?;
        self.0.push(Line::LiteralFieldLinewithPostBaseNameReference { prefix: "0000".to_string(), n, name_id, h, value_length:l, value_string: value_string});
        Ok(())
    }

    fn line_name(&mut self,b:&mut Bytes, byte0: u8) -> Result<(), DecodeError> {
        let n = (byte0 & 0b0001_0000) >> 4;
        let nh = (byte0 & 0b0000_1000) >> 3;
        let name_len = prefixint::decode_byte(b, byte0, 3)?;
        let name_bytes = b.copy_to_bytes(name_len);
        let name_string = str_decode_n(nh,name_bytes)?;
        let (h,l,value_string) = str_decode(b)?;
        self.0.push(Line::LiteralFieldLinewithLiteralName{ prefix: "001".to_string(), name_h: nh, name_lenght: name_len, name_string, value_h: h, value_string, n, value_length:l });
        Ok(())
    }

}
fn str_decode_n(hm:u8,b: Bytes)->Result<String,DecodeError>{
    if hm == 0b1 {
        let mut hm_octets = octets::Octets::with_slice(&b);
        let str_bytes = match huffman::decode(&mut hm_octets) {
            Ok(v) => v,
            Err(_) => {
                return Err(DecodeError::new("str_decode-huffman", "解码失败"));
            }
        };
        if let Ok(str_value) = String::from_utf8(str_bytes) {
            return Ok(str_value);
        } else {
            return Err(DecodeError::new("str_decode-huffman-str", "解码失败"));
        }
    }else{
        if let Ok(str_value) = String::from_utf8(b.to_vec()) {
            return Ok(str_value);
        } else {
            return Err(DecodeError::new("str_decode-str", "解码失败"));
        }
    }
}
fn str_decode<B: Buf>(b: &mut B)->Result<(u8,usize,String),DecodeError>{
    let byte1 = b.get_u8();
    let value_len = prefixint::decode_byte(b, byte1, 7)?;
    if byte1>>7 == 0b1 {
        let  hm_bytes = b.copy_to_bytes(value_len);
        let mut hm_octets = octets::Octets::with_slice(&hm_bytes);
        let str_bytes = match huffman::decode(&mut hm_octets) {
            Ok(v) => v,
            Err(_) => {
                return Err(DecodeError::new("str_decode-huffman", "解码失败"));
            }
        };
        if let Ok(str_value) = String::from_utf8(str_bytes) {
            return Ok((1,value_len,str_value));
        } else {
            return Err(DecodeError::new("str_decode-huffman-str", "解码失败"));
        }
    }else{
        let str_bytes = b.copy_to_bytes(value_len);
        if let Ok(str_value) = String::from_utf8(str_bytes.to_vec()) {
            return Ok((0,value_len,str_value));
        } else {
            return Err(DecodeError::new("str_decode-str", "解码失败"));
        }
    }
}
impl HeaderFrame {
    pub fn new()->Self{
        Self { frame_type: None, bytes: None, length: None, field_section: None }
    }
    pub fn parse(&mut self,b: &mut Bytes)->Result<(),DecodeError>{
        let buf_c = b.clone().to_vec();
        self.bytes = Some(buf_c);
        self.parse_type(b)?;
        let mut field_section = FieldSection::new(b)?;
        let mut lines = Lines::new();
        let _ = lines.build(b)?;
        field_section.field_lines = lines.0;
        self.field_section = Some(field_section);
        
        Ok(())
    }
    pub fn parse_type(&mut self,b: &mut Bytes )->Result<(),DecodeError>{
        let ty= varint::decode(b)?;
        if ty !=0x01 {
            return Err(DecodeError::new("帧类型错误", "不是Headrs帧"));
        }
        self.frame_type = Some(ty);
        let len = varint::decode(b)?;
        self.length = Some(len);
        *b = b.copy_to_bytes(len as usize);
        Ok(())
    }
}
pub struct Decoder(Bytes);

impl Decoder {
    pub fn new(buf:Vec<u8>)->Self{
        let bytes = Bytes::from(buf);
        Self(bytes)
    }
    pub fn parse(&mut self)->Result<HeaderFrame,DecodeError>{
        let mut frame = HeaderFrame::new();
        frame.parse(&mut self.0)?;
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_worksa() {
       let v=[1, 64, 130, 0, 0, 209, 215, 80, 143, 53, 43, 250, 241, 235, 192, 235, 173, 114, 30, 155, 141, 62, 251, 255, 193, 95, 80, 231, 208, 127, 102, 162, 129, 176, 218, 224, 82, 26, 235, 160, 188, 139, 30, 99, 37, 134, 217, 117, 118, 92, 83, 250, 205, 143, 126, 140, 255, 74, 80, 110, 165, 83, 17, 73, 212, 255, 106, 16, 244, 214, 52, 154, 58, 11, 246, 167, 43, 199, 144, 186, 74, 150, 4, 184, 62, 212, 255, 115, 165, 53, 162, 227, 12, 78, 148, 214, 202, 254, 8, 121, 10, 189, 69, 75, 31, 218, 151, 167, 176, 244, 149, 128, 133, 197, 192, 184, 95, 101, 229, 221, 113, 77, 195, 148, 118, 25, 134, 217, 117, 118, 92, 221, 223];
        let x = Decoder::new(v.to_vec()).parse();
        println!("###{:#?}",x);
    }
}
