use crate::chain;
use crate::convert;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::path::Path;

pub fn write_chain_to_disk() {
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("data/chain.json")
    .unwrap();
  let block = chain::get_latest_block();
  writeln!(file,"{}",convert::block_to_json(&block));
}

pub fn read_chain_from_disk() {
  let mut file = OpenOptions::new()
    .read(true)
    .open("data/chain.json")
    .unwrap();
  let reader = BufReader::new(file);
  for block in reader.lines() {
    chain::add_block(
      convert::json_to_block(&block.unwrap())
    );
  }
}