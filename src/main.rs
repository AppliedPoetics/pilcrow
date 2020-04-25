#[path = "blocks/block.rs"] mod block;

#[path = "networking/clients.rs"] mod clients;
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
          clients::new_client(
            stream
              .peer_addr()
              .unwrap()
          );
          let mut data = [0 as u8; 50];
          while match stream.read(&mut data) {
            Ok(size) => {
              block::make_block(None, None, None);
              true
            },
            Err(_) => {
              println!("ERRRRR");
              false
            }
          }{}
        });
      }
    },
    _ => println!("Error starting listener"),
  }
}
