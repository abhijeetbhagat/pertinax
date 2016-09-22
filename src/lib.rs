#![feature(conservative_impl_trait)]
extern crate unix_socket;

use unix_socket::UnixStream;
use std::io::Read;
use std::io::Write;

trait Connection{
    fn write(&mut self, buffer : &[u8] , offset : i32 , size : i32 , immediate : bool);
    fn read(&mut self,  buffer : &mut [u8], offset : i32, size : i32);
    fn close(&mut self);
}

struct Uri{

}

trait ConnectionInitiator{
    fn connect(&mut self, uri : Uri) ->  Box<Connection>;
}

trait ConnectionListener{
    fn accept();
}

struct PipeConnectionInitiator{
    buffer_size : i32
}

impl PipeConnectionInitiator{
    fn new(buffer_size : i32)->PipeConnectionInitiator{
        PipeConnectionInitiator {
            buffer_size : buffer_size
        }  
    } 
}

impl ConnectionInitiator for PipeConnectionInitiator{
    fn connect(&mut self, uri : Uri) -> Box<Connection>{
        let stream = UnixStream::connect("").unwrap();
        Box::new(PipeConnection::new(stream, self.buffer_size))
    }
}

struct PipeConnection{
    stream : UnixStream,
    buffer_size : i32
}

impl PipeConnection{
    fn new(stream : UnixStream, buffer_size : i32)->Self{
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
