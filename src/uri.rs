enum Scheme{
    NamedPipe,
    Tcp,
}

pub struct Uri<'a>{
    scheme : Scheme,
    host : &'a str,
    port : &'a str,
    path : &'a str, 
}

impl<'a> Uri<'a>{
    fn new()->Self{
        unimplemented!();
    }
}
