use crate::routers;
use serde_json::json;
use std::io::Write;
use std::net::TcpStream;

pub fn ping(mut stream: TcpStream) {
    let status = "success";
    let response_text = routers::response::new()
        .set_data(
            json!({
                "status": status,
                "data":"hello world",
            })
            .to_string(),
        )
        .to_stream();
    println!("{}", response_text);
    stream.write(response_text.as_bytes()).unwrap();
}
