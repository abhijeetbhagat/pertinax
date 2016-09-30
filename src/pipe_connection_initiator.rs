use uri::Uri;
use connection::Connection;
use connection_initiator::ConnectionInitiator;
use uuid::Uuid;
use unix_socket::UnixStream;
use pipe_connection::PipeConnection;

struct PipeConnectionInitiator{
    buffer_size : i32
}

impl PipeConnectionInitiator{
    fn new(buffer_size : i32)->PipeConnectionInitiator{
        PipeConnectionInitiator {
            buffer_size : buffer_size
        }  
    } 

    fn get_pipe_name(&self)->String{
        Uuid::new_v4().simple().to_string()
    }
}

impl ConnectionInitiator for PipeConnectionInitiator{
    fn connect(&mut self, uri : Uri) -> Box<Connection>{
        let pipe_name = self.get_pipe_name();
        let stream = UnixStream::connect(pipe_name).unwrap();
        box PipeConnection::new(stream, self.buffer_size)
    }
}

