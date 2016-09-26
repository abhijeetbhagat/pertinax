extern crate uuid;
extern crate unix_socket;

pub mod uri;
pub mod channel_factory;
pub mod channel_creator;
pub mod endpoint;
pub mod connection;
pub mod pipe_connection;
pub mod connection_initiator;
pub mod pipe_connection_initiator;

use uri::Uri;



trait ConnectionListener{
    fn accept();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
