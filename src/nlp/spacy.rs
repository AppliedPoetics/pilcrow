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

pub fn init()
  -> cpython::PyObject {
    let spacy = SPACY
      .lock()
      .unwrap();
    let gil = cpython::Python::acquire_gil();
    let python = gil.python();
    println!("Loading spacy...");
    // This resolves as an object; need to store it to call it --> nlp("")
    let modeled = spacy.call(python, "load", ("en_core_web_lg",), None)
      .unwrap();
    modeled
  }

pub fn get_instance(model: cpython::PyObject) 
  {//-> cpython::PyObject {
    let gil = cpython::Python::acquire_gil();
    let python = gil.python();
    println!("Reacquiring spacy...");
    let instance = SPACY
      .lock()
      .unwrap();
    println!("Calling instance...");
    let result = model.call(python, ("What",), None).unwrap();
    let result2 = model.call(python, ("nope",), None).unwrap();
    println!("Should be a result here...");
    println!("{:?}",result);
    println!("And other here...");
    println!("{:?}",result.call_method(python,"similarity",(result2,),None));
  }