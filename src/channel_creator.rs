use channel_factory::ChannelFactory;
use uri::Uri;
use endpoint::EndPoint;
use communication_object::CommunicationObject;
use binding::Binding;

pub struct ChannelCreator<T:Clone>{
    channel : Box<T>,
    binding : Option<Box<Binding>>,
    uri : String
}

impl<T:Clone> ChannelCreator<T>{
    pub fn new(channel : T, binding : Option<Box<Binding>>, uri : String) -> Self{
        ChannelCreator{
            channel : box channel,
            binding : binding,
            uri : uri,
        }
    }
}

impl<T:Clone> ChannelFactory<T> for ChannelCreator<T>{
    fn create_channel(&mut self, to : EndPoint)->Box<T>{
        self.channel.clone()
    }
    fn create_channel_with_uri(&mut self, to : EndPoint, uri : Uri)->Box<T>{
        self.channel.clone()
    }

}
impl<T:Clone> CommunicationObject for ChannelCreator<T>{
    fn open(&mut self){
        println!("channel opened");
    }

    fn close(&mut self){
        println!("channel closed");
    }
}
