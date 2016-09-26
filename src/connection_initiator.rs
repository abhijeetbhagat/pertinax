use connection::Connection;
use uri::Uri;
pub trait ConnectionInitiator{
    fn connect(&mut self, uri : Uri) ->  Box<Connection>;
}
