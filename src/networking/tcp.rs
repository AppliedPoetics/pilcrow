use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Command;
use std::thread;

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

pub fn bind() {
    if let Some(addr) = get_addr() {
      let sock = addr;
      let listener = TcpListener::bind(
        (addr,8080)
      ).unwrap();
      match listener {
        listener => {
          for stream in listener.incoming() {
            thread::spawn(|| {
              let mut stream = stream.unwrap();
              stream.write(b"!").unwrap();
            });
          }
        },
        _ => println!("Error starting listener"),
      }
    }
  }