#[path = "blocks/block.rs"] mod block;
#[path = "blocks/chain.rs"] mod chain;
#[path = "networking/clients.rs"] mod clients;
#[path = "networking/tcp.rs"] mod tcp;

use std::io::Read;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
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
          let mut latest_block = Some(block::make_block(None, None, None));
          while match stream.read(&mut data) {
            Ok(size) => {
              let block = block::make_block(latest_block, None, None);
              chain::add_block(block);
              latest_block = Some(chain::get_latest_block());
              println!("{:?}",&latest_block.clone().unwrap());
              true
            },
            Err(_) => {
              stream.shutdown(Shutdown::Both).unwrap();
              false
            }
          }{}
        });
      }
    },
    _ => println!("Error starting listener"),
  }
}
