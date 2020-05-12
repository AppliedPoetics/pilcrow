use crate::block;
use crate::chain;
use crate::clients;
use crate::clock;
use crate::convert;
use crate::packets;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

pub fn validate() 
  -> Result<(), ()> {
  let chain = chain::get_whole_chain();
  let mut check_block = chain[0].clone();
  for b in 0..chain.len() {
    check_block.calc_hash();
    if chain[b].prev_hash != check_block.hash {
      return Err(());
    }
    check_block = chain[b].clone();
  }
  Ok(())
}

pub fn resolve() {
  // Get list of nodes
  let clients = clients::get_client_list();
  // Establish this node's chain and count
  let mut chain = chain::get_whole_chain();
  let mut chain_length = chain::get_block_count();
  // Compare with other nodes
  for client in clients {
    match TcpStream::connect(client + ":1848") {
      Ok(mut stream) => {
        // Packet with placeholder values
        let packet = packets::Incoming {
          purpose: "resolve".to_string(),
          time: clock::get_time_now(),
          sender: "pilcrowd".to_string()
        };
        let data = convert::data_to_json(&packet);
        stream.write(
          &data
            .clone()
            .into_bytes()
        );
      },
      _ => panic!("Everybody panic!"),
    }
  }
}