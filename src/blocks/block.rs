use crate::chain;
use crate::clock;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
  pub num: usize,
  pub time: String,
  pub text: String,
  pub hash: String,
  pub prev_hash: String,
  pub staker: String,
}

impl Block {
  pub fn new(previous: Option<Block>) 
    -> Block {
      Block {
        num: chain::get_block_count(),
        time: clock::get_time_now(),
        text: String::new(),
        hash: String::new(),
        prev_hash: match previous {
          Some(block) => {
            let hash: &str = &*block.hash;
            String::from(hash)
          },
          _ => String::new(),
        },
        staker: String::new(),
      }
    }
    
  pub fn set_text(&mut self, text: Option<String>) {
    self.text = match text {
      Some(entry) => entry,
      _ => String::new(),
    };
  }
  
  pub fn set_staker(&mut self, staker: Option<String>) {
    self.staker = match staker {
      Some(writer) => writer,
      _ => String::from("pilcrowd"),
    };
  }

  pub fn calc_hash(&mut self) {
    let mut hashed = String::new();
    let mut hasher = Sha256::default();
    let mut hashable =  self.num.to_string();
    hashable.push_str(&self.time);
    hashable.push_str(&self.text);
    hashable.push_str(&self.prev_hash);
    hasher.input(hashable);
    let result = hasher.result();
    for v in result.iter() {
      hashed.push_str(&v.to_string());
    }
    self.hash = hashed;
  }
}