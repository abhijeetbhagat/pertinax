use channel_factory::ChannelFactory;
use uri::Uri;
use endpoint::EndPoint;
use communication_object::CommunicationObject;

pub struct ChannelCreator<T>{
    channel : T
}

impl<T> ChannelCreator<T>{
    pub fn new(channel : T) -> Self{
        ChannelCreator{
            channel : channel
        }
    }
}

impl<T> ChannelFactory<T> for ChannelCreator<T>{
    fn create_channel(&self, to : EndPoint)->Box<T>{
        unimplemented!();
    }
    fn create_channel_with_uri(&self, to : EndPoint, uri : Uri)->Box<T>{
        unimplemented!();
    }

}
impl<T> CommunicationObject for ChannelCreator<T>{
    fn open(&mut self){}

    fn close(&mut self){}
}
