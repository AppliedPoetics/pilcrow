use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::Mutex;

struct Client {
  ip: SocketAddr,
  stake: u32
}

lazy_static! {
  static ref CLIENTS: Mutex<HashMap<usize, Client>> = {
    let map = HashMap::new();
    Mutex::new(map)
  };
}

fn add_client(addr: SocketAddr) {
  let mut map = CLIENTS
    .lock()
    .unwrap();
  let client = Client {
    ip: addr,
    stake: 0
  };
  if map.is_empty() {
    map.insert(1,client);
  } else {
    let next_id = map.len() + 1;
    map.insert(next_id,client);
  }
}

pub fn new_client(addr: SocketAddr) {
  println!("Connection from: {}",addr);
  add_client(addr);
}