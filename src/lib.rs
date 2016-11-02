#![feature(box_syntax, box_patterns)]
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
pub mod message;
pub mod proxy;
pub mod binary_encoder;

use uri::Uri;

trait ConnectionListener{
    fn accept();
}


#[cfg(test)]
mod tests {
    use super::channel_factory::*;
    use super::channel_creator::*;
    use super::endpoint::*;
    use namedpipe_binding::*;
    use message::*;
    use proxy::*;
    use binary_encoder::*;

    trait IService{
        fn foo(&self, a:i32);
        fn bar(&self)->i32;
    }

    #[derive(Clone)]
    struct Client{
        //binding : Binding,
        //proxy : Proxy
    }

    impl IService for Client{
        fn foo(&self, a:i32){
            //proxy.send("foo", [(a : i32)]);
        }

        fn bar(&self)->i32{
            //let result = proxy.send("foo", [a]);
            0
        }
    }
    #[test]
    fn test_usage() {
        //let c = Client;
        //let mut c : ChannelCreator<&IService> = ChannelCreator::new(&c, Some(box NamedPipeBinding), String::from("localhost"));
        //let s : &IService = *c.create_channel(EndPoint);
        //assert!(s.bar() == 0);
    }

    #[test]
    fn test_one_byte_encoder(){
        assert_eq!(one_byte_encoder(17), 0x11);
    }

    #[test]
    fn test_two_byte_encoder(){
        assert_eq!(two_bytes_encoder(145), 0x191);
        assert_eq!(two_bytes_encoder(5521), 0x2B91);
    }

}
