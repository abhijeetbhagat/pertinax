use uri::Uri;  
use endpoint::EndPoint;  
use communication_object::CommunicationObject;

pub trait ChannelFactory<T> : CommunicationObject{
    fn create_channel(&mut self, to : EndPoint)->Box<T>;
    fn create_channel_with_uri(&mut self, to : EndPoint, uri : Uri)->Box<T>;
}
