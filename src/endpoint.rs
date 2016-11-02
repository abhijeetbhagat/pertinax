use uri::Uri;
use binding::Binding;

pub struct EndPoint<'a>{
    address : Uri<'a>,
    binding : &'a Binding,
}

impl<'a> EndPoint<'a>{
    fn new(uri: Uri<'a>, binding : &'a Binding)->Self {
        EndPoint{
            address : uri,
            binding : binding
        }
    }
}
