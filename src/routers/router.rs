use std::net::TcpStream;

pub struct Router<'a, T>
where
    T: Fn(&mut TcpStream),
{
    pub method: &'a str,
    pub uri: &'a str,
    pub handler: T,
}

impl<'a, T> Router<'a, T>
where
    T: Fn(&mut TcpStream),
{
    pub fn new(method: &'a str, uri: &'a str, handler: T) -> Router<'a, T> {
        Router {
            method,
            uri,
            handler,
        }
    }
}
