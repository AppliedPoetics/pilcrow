use rand::*;
use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::prelude::*;
use std::path::Path;

fn load()
  -> String {
    let mut file = OpenOptions::new()
      .read(true)
      .open("data/corpus.txt")
      .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
      .unwrap();
    contents
  }
  
pub fn select() 
  -> String {
    let re = Regex::new(r"([A-Z][^\.!?]*[\.!?])")
      .unwrap();
    let text = load();
    let matches = re.find_iter(&text)
      .map(|cap| cap.as_str())
      .collect::<Vec<_>>();
    let index = (rand::random::<f32>() * matches.len() as f32)
        .floor() as usize;
    let choice = matches[index];
    String::from(choice)
  }