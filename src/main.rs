use routers::Routers;
use std::net::TcpListener;

use std::sync::Arc;
use std::thread;

pub mod routers;

fn main() {
    let mut rs = Routers {
        routers: Vec::new(),
    };

    rs.register("post", "/ping", routers::handler::ping);

    let share_routers = Arc::new(rs);
    // listen
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080...");
    println!("{}", share_routers);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let sr = Arc::clone(&share_routers);
        thread::spawn(move || {
            sr.handle_connection(stream);
        });
    }
}
