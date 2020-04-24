use std::net::{IpAddr, Ipv4Addr};
use std::net::{SocketAddr, TcpListener};
use std::process::Command;

fn get_addr() 
  -> Option<IpAddr> {
    let cmd = match Command::new("hostname")
      .args(&["-I"])
      .output() {
        Ok(ok) => ok,
        _ =>  return None,
      };
    let stdout = match String::from_utf8(cmd.stdout) {
      Ok(ok) => ok,
      _ => return None,
    };
    let ip_list: Vec<&str> = stdout
      .trim()
      .split(" ")
      .collect::<Vec<&str>>();
    let ip = ip_list.first();
    match ip {
      Some(addr) => {
        if let Ok(addr) = addr.parse::<Ipv4Addr>() {
          return Some(IpAddr::V4(addr));
        }
      }
      None => (),
    }
    None
  }

pub fn bind() 
  -> TcpListener {
    let listener;
    if let Some(addr) = get_addr() {
      let sock = addr;
      listener = TcpListener::bind(
        (addr,8080)
      ).unwrap();
      return listener;
    }
    listener = TcpListener::bind(
      ("127.0.0.1",8080)
    ).unwrap();
    listener
  }