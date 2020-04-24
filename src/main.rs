#[path = "networking/tcp.rs"] mod tcp;

use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
  let listener = tcp::bind();
  match listener {
    listener => {
      for stream in listener.incoming() {
        thread::spawn(|| {
          let mut buf = String::new();
          let mut stream = stream.unwrap();
          stream.read_to_string(&mut buf);
          
        });
      }
    },
    _ => println!("Error starting listener"),
  }
}
