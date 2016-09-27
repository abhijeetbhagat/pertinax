use channel_factory::ChannelFactory;
use uri::Uri;
use endpoint::EndPoint;
use communication_object::CommunicationObject;
use binding::Binding;

pub struct ChannelCreator<T>{
    channel : Box<T>,
    binding : Option<Box<Binding>>
}

impl<T> ChannelCreator<T>{
    pub fn new(channel : T, binding : Option<Box<Binding>>) -> Self{
        ChannelCreator{
            channel : Box::new(channel),
            binding : binding
        }
    }
}

impl<T> ChannelFactory<T> for ChannelCreator<T>{
    fn create_channel(&mut self, to : EndPoint)->Box<T>{
        unimplemented!();
    }
    fn create_channel_with_uri(&mut self, to : EndPoint, uri : Uri)->Box<T>{
        unimplemented!();
    }

}
impl<T> CommunicationObject for ChannelCreator<T>{
    fn open(&mut self){}

    fn close(&mut self){}
}
