pub fn one_byte_encoder(value : u8) -> u8{
    value
}

pub fn two_bytes_encoder(value : u16) -> u16{
    let a = value & 0x7F; //value A 0x11
    let b = value & 0x3F80; //value B 0x80
    let mut r = 0 | b << 1 ; //0x80
    //r = r << 1;//0x8000
    r = r | 0x80;
    r = r | a;
    r
}
