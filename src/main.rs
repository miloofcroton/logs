// use std::{fs, io::Error};
use std::fs;
use std::io::Error;

fn divide(a: f64, b: f64) -> Result<f64, Error> {
  if b == 0.0 {
    Err(Error::other("can't divide by zero"))
  }
  else {
    Ok(a / b)
  }
}

fn validateEmail(email: String) -> Result<(), Error>{
  if email.contains("@") {
    Ok(())
  }
  else {
    Err(Error::other("email is missing @ symbol"))
  }
}

fn main() {
  // let text = fs::read_to_string("logs.txt");

  // println!("{:#?}", text);

  match divide(5.0, 0.0) {
    Ok(result) => {
      println!("{}", result)
    }
    Err(err) => {
      println!("{}", err)
    }
  }

  match validateEmail(String::from("test@example.com")) {
    Ok(..) => println!("email is valid"),
    Err(errMsg) => println!("{}", errMsg),
  }
}
