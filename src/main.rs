use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        if let Err(e) = stream {
            println!("error: {}", e);
            return Err(e);
        }
        stream?.write(b"+PONG\r\n")?;
    }
    Ok(())
}
