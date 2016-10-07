use message::*;
use std::any::{Any};

struct Proxy;


impl Proxy{
  fn new()->Self{
    Proxy
  }

  fn send(method_name : &str, args : &[Box<Any>])->Message{
      Message{}
  }
}
