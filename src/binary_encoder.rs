use std::borrow::Cow;
use std::convert::Into;
use num::{Num, Integer, PrimInt, FromPrimitive};
use std::cmp::PartialOrd;
use std::mem;

pub fn one_byte_encoder(value : u8) -> u8{
    value
}

pub fn two_bytes_encoder(value : u16) -> u16{
    let a = value & 0x7F; //value A 
    let b = value & 0x3F80; //value B 
    0 | b << 1 | 0x80 | a
    //print!(");
    //a
}

pub fn three_byte_encoder(value : u32) -> u32 {
    let a:u32 = value & 0x7F;
    let b:u32 = value & 0x3F80;
    let c:u32 = value & 0x1FC000;

    (0 | c<<2) | (0x8000 | b<<1) | (0x80 | a)
}

pub fn four_byte_encoder(value : u32) -> u32 {
    let a:u32 = value & 0x7F;
    let b:u32 = value & 0x3F80;
    let c:u32 = value & 0x1FC000;
    let d:u32 = value & 0xFE00000;
    (0 |d<<3) | (0x800000 | c<<2) | (0x8000 | b<<1) | (0x80 | a) 
}

pub fn five_byte_encoder(value : u64) -> u64 {
    let a:u64 = value & 0x7F;
    let b:u64 = value & 0x3F80;
    let c:u64 = value & 0x1FC000;
    let d:u64 = value & 0xFE00000;
    let e:u64 = value & 0x7F0000000;
    (0|e<<4)| (0x80000000 |d<<3) | (0x800000 | c<<2) | (0x8000 | b<<1) | (0x80 | a) 

}


pub fn binary_encoder<T:Ord + FromPrimitive>(value : T) -> T {

    if(value as u8 >= FromPrimitive::from_u8(0x00).unwrap() && value as u8 <= FromPrimitive::from_u8(0x7F).unwrap()){
        //let val:Option<u8> = Some(value);
        return one_byte_encoder(value as u8) as T;
    }
    else if(value as u16 >= FromPrimitive::from_u16(0x0080).unwrap() && value as u16 <= FromPrimitive::from_u16(0x3FFF).unwrap()){
        return two_bytes_encoder(value as u16) as T;
    }
    else if(value as u32 >= FromPrimitive::from_u32(0x004000).unwrap() && value as u32 <= FromPrimitive::from_u32(0x1FFFFF).unwrap()){
        return three_byte_encoder(value as u32) as T;
    }
    else if(value as u32 >= FromPrimitive::from_u32(0x00200000).unwrap() && value as u32 <= FromPrimitive::from_u32(0x0FFFFFFF).unwrap()){
        return four_byte_encoder(value as u32) as T;
    }
    else {
        return five_byte_encoder(value as u64) as T;
    }

}

pub fn string_encoder<'a>(input: &'a str)->Vec<u8>{
   let mut v = Vec::new();
    v.push(input.len() as u8);
   // for c in input.chars(){
    //    v.push(c as u8);
   // }
    v    
}
