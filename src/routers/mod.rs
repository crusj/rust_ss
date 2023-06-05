use router::Router;
use std::fmt;
use std::net::TcpStream;
pub mod handler;
pub mod response;
pub mod router;

use std::io::{Read, Write};
pub struct Routers<'a, T>
where
    T: Fn(&mut TcpStream),
{
    pub routers: Vec<Router<'a, T>>,
}

impl<'a, T> Routers<'a, T>
where
    T: Fn(&mut TcpStream),
{
    pub fn register(&mut self, method: &'a str, uri: &'a str, handler: T) {
        self.routers.push(Router::new(method, uri, handler));
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut data: Vec<u8> = Vec::new();
        let mut buf = [0; 10];
        loop {
            let size = match stream.read(&mut buf) {
                Ok(size) => size,
                Err(err) => panic!("{}", err),
            };
            println!("{}", size);
            if size > 0 {
                data.extend(buf[..size].into_iter());
            }

            if size < 10 {
                break;
            }
        }

        println!("{}", data.len());
        let data = String::from_utf8(data).unwrap();
        let c: Vec<&str> = data.split("\r\n").collect();
        let d: Vec<&str> = c[0].split(" ").collect();
        let incoming_uri = d.get(1).unwrap();
        let method = d.get(0).unwrap();

        println!("{} {}", incoming_uri, method);
        match self.check_router(method, incoming_uri) {
            Some(handler) => {
                handler(&mut stream);
            }
            None => {
                stream
                    .write_all("HTTP/1.1 404 not found\r\n\r\n".as_bytes())
                    .unwrap();
            }
        }
    }

    fn check_router(&self, method: &str, uri: &str) -> Option<T> {
        for router in &self.routers {
            if router.uri.eq(uri) && router.method.eq_ignore_ascii_case(method) {
                return Some(router.handler);
            }
        }

        None
    }
}

impl<T> fmt::Display for Routers<'_, T>
where
    T: Fn(&mut TcpStream),
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for router in &self.routers {
            write!(f, "{} {}\n", router.method, router.uri)?;
        }
        Ok(())
    }
}
