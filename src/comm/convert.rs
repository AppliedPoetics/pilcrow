use crate::block;
use crate::chain;
use crate::packets;
use serde::{Serialize, Deserialize};
use serde_json::*; 

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
        String::from(_v.trim_end())
      },
      Err(why) => panic!("Everybody panic!"),
    };
    String::from(s)
  }
  
pub fn data_to_json(data: &packets::Incoming) 
  -> String {
    let json = serde_json::to_string(data);
    json.unwrap()
  }
  
pub fn block_to_json(block: &block::Block)
  -> String {
    let json = serde_json::to_string(block);
    json.unwrap()
  }
  
pub fn json_to_block(line: &String)
  -> block::Block {
    let json: Value = serde_json::from_str(line)
      .unwrap();
    let number = json["num"]
      .as_u64()
      .unwrap();
    let block = block::Block {
      num: number as usize,
      time: json["time"].to_string(),
      text: json["text"].to_string(),
      hash: json["hash"].to_string(),
      prev_hash: json["prev_hash"].to_string(),
      staker: json["staker"].to_string(),
    };
    block
  }