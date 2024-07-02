use std::{error::Error, fmt::Display};



#[derive(Debug)]
pub struct DecodeError{
    name:String,
    message:String,
}

impl DecodeError {
   pub fn new(name:&str,meaasge:&str)->Self{
        Self { name: name.to_string(), message: meaasge.to_string() }
   } 
}

impl Error for DecodeError {}

impl Display for DecodeError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "==>错误类型:{},错误描述:{}",self.name,self.message)
    }
}