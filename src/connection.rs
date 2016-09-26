pub trait Connection{
    fn write(&mut self, buffer : &[u8] , offset : i32 , size : i32 , immediate : bool);
    fn read(&mut self,  buffer : &mut [u8], offset : i32, size : i32);
    fn close(&mut self);
}
