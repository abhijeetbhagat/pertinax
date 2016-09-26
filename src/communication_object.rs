pub trait CommunicationObject{
    fn open(&mut self);
    fn close(&mut self);
}
