use crate::chain;
use crate::clock;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
  num: usize,
  time: String,
  text: String,
  hash: String,
  prev_hash: String,
  staker: String
}

fn calculate_hash(block: &Block) 
  -> String {
    let mut hashed = String::new();
    let mut hasher = Sha256::default();
    let mut hashable =  block.num.to_string();
    hashable.push_str(&block.time);
    hashable.push_str(&block.text);
    hashable.push_str(&block.prev_hash);
    hasher.input(hashable);
    let result = hasher.result();
    for v in result.iter() {
      hashed.push_str(&v.to_string());
    }
    hashed
  }

//pub fn make_block(previous: Option<Block>, text: Option<String>, staker: Option<String>) 

impl Block {
  
  pub fn new() 
    -> Block {
      Block {
        num: chain::get_block_count(),
        time: clock::get_time_now(),
        text: String::new(),
        hash: String::new(),
        prev_hash: String::new(),
        staker: String::new(),
      }
    }
    
  pub fn create(mut self)
    -> Block {
      self.hash = calculate_hash(&self);
      self
    }
}