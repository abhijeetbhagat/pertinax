extern crate unix_socket;
extern crate uuid;

use unix_socket::UnixStream;
use std::io::Read;
use std::io::Write;
use uuid::Uuid;
use connection::Connection;

pub struct PipeConnection{
    stream : UnixStream,
    buffer_size : i32
}

impl PipeConnection{
    pub fn new(stream : UnixStream, buffer_size : i32)->Self{
        PipeConnection{
            stream : stream,
            buffer_size : buffer_size
        }
    }
}

impl Connection for PipeConnection{
    fn write(&mut self, buffer : &[u8] , offset : i32 , size : i32 , immediate : bool){
        self.stream.write_all(buffer);
    }

    fn read(&mut self, buffer : &mut [u8], offset : i32, size : i32){
        let mut tmp_buf = Vec::new();
        self.stream.read_to_end(&mut tmp_buf).unwrap();
        //can we instead factor out the above line in a new method
        //that returns the vec and transfer the ownership
        buffer.copy_from_slice(&tmp_buf);
    }

    fn close(&mut self){
    }
}
