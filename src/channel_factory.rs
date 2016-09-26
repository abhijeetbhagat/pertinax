use uri::Uri;  
use endpoint::EndPoint;  

pub trait ChannelFactory<T>{
    fn create_channel(&self, to : EndPoint)->Box<T>;
    fn create_channel_with_uri(&self, to : EndPoint, uri : Uri)->Box<T>;
}
