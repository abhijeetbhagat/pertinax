use channel_factory::ChannelFactory;
use uri::Uri;
use endpoint::EndPoint;
struct ChannelCreator<T>{
    channel : T
}

impl<T> ChannelFactory<T> for ChannelCreator<T>{
    fn create_channel(&self, to : EndPoint)->Box<T>{
        unimplemented!();
    }
    fn create_channel_with_uri(&self, to : EndPoint, uri : Uri)->Box<T>{
        unimplemented!();
    }

}
