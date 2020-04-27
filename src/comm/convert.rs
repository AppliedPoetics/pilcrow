use crate::block;

use serde::{Serialize, Deserialize};
use serde_json;
  
pub fn data_from_u8(data: &[u8;50])
  -> String {
    let s = match std::str::from_utf8(data) {
      Ok(v) => {
        let mut _v = String::from(v);
        for c in String::from(v).chars() {
          if c.is_control() {
            _v.pop();
          }
        }
        if _v.ends_with('\n') {
          _v.pop();
        }
        if _v.ends_with('\r') {
          _v.pop();
        }
        String::from(_v.trim_end())
      },
      Err(why) => panic!("Everybody panic!"),
    };
    String::from(s)
  }
  
pub fn block_to_json(block: &block::Block)
  -> String {
    let json = serde_json::to_string(block);
    json.unwrap()
  }
  
pub fn json_to_block(block: &String)
  -> String {
    let json = serde_json::from_str(block);
    json.unwrap()
  }