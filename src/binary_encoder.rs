use std::borrow::Cow;
use std::convert::Into;
pub fn one_byte_encoder(value : u8) -> u8{
    value
}

pub fn two_bytes_encoder(value : u16) -> u16{
    let a = value & 0x7F; //value A 
    let b = value & 0x3F80; //value B 
    0 | b << 1 | 0x80 | a
}

pub fn string_encoder<'a>(input: &'a str)->Vec<u8>{
    let mut v = Vec::new();
    v.push(input.len() as u8);
    for c in input.chars(){
        v.push(c as u8);
    }
    v
}
