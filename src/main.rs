#[path = "networking/tcp.rs"] mod tcp;

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
  let listener = tcp::bind();
  match listener {
    listener => {
      for stream in listener.incoming() {
        thread::spawn(|| {
          let mut stream = stream.unwrap();
            stream
              .write(b"ALLO ALLO!")
              .unwrap();
        });
      }
    },
    _ => println!("Error starting listener"),
  }
}
