#[path = "blocks/block.rs"] mod block;
#[path = "blocks/chain.rs"] mod chain;
#[path = "comm/convert.rs"] mod convert;
#[path = "comm/messages.rs"] mod messages;
#[path = "comm/packets.rs"] mod packets;
#[path = "comm/router.rs"] mod router;
#[path = "fs/fs.rs"] mod file;
#[path = "ops/clock.rs"] mod clock;
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
          
          let mut stream = stream.unwrap();
          
          clients::new_client(
            stream
              .peer_addr()
              .unwrap()
          );
          
          let mut data = [0 as u8; 50];
          
          let mut next_block = block::Block::new(None);
          
          while match stream.read(&mut data) {
            Ok(size) => {
            
              let msg = messages::interpret(data);
              
              let mut block = block::Block::new(Some(next_block));
              block.calc_hash();
              
              chain::add_block(block);
              
              next_block = chain::get_latest_block();
              
              let unwrapped_block = &next_block
                .clone();
              
              let transmit_block = convert::block_to_json(unwrapped_block);
              
              stream.write(
                &transmit_block
                  .clone()
                  .into_bytes()
              );
              
              file::write_chain_to_disk();
              
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
