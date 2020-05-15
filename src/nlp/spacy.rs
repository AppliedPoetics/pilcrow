use cpython::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Need to find a way to return the modified instance
lazy_static! {
  static ref SPACY: Mutex<cpython::PyModule> = {
    let gil = cpython::Python::acquire_gil();
    let python = gil.python();
    let spacy = python.import("spacy")
      .unwrap();
    println!("Calling spacy");
    Mutex::new(spacy)
  };
}

pub fn init() {
  let spacy = SPACY
    .lock()
    .unwrap();
  let gil = cpython::Python::acquire_gil();
  let python = gil.python();
  println!("Loading spacy...");
  let modeled = spacy.call(python, "load", ("en_core_web_lg",), None)
    .unwrap();
  println!("Now we have: {:?}",modeled);
  println!("Calling instance...");
}

pub fn get_instance() 
  {//-> cpython::PyObject {
    let gil = cpython::Python::acquire_gil();
    let python = gil.python();
    println!("Reacquiring spacy...");
    let instance = SPACY
      .lock()
      .unwrap();
    println!("Calling instance...");
    let result = instance.call(python,"nlp", ("What",), None);
    println!("Should be a result here...");
    println!("{:?}",result);
  }