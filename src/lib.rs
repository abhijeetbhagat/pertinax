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
pub mod communication_object;
pub mod binding;
pub mod namedpipe_binding;

use uri::Uri;

trait ConnectionListener{
    fn accept();
}


#[cfg(test)]
mod tests {
    use super::channel_factory::*;
    use super::channel_creator::*;

    trait IService{
        fn foo(&self, a:i32);
        fn bar(&self)->i32;
    }

    struct Client;

    impl IService for Client{
        fn foo(&self, a:i32){

        }

        fn bar(&self)->i32{
            0
        }
    }
    #[test]
    fn test_usage() {
        let c : ChannelCreator<Box<IService>> = ChannelCreator::new(Box::new(Client), None);
    }
}
