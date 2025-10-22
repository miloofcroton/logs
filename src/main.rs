use core::error;
// use std::{fs, io::Error};
use std::fs;
use std::io::Error;

// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//   if b == 0.0 {
//     Err(Error::other("can't divide by zero"))
//   }
//   else {
//     Ok(a / b)
//   }
// }

// fn validateEmail(email: String) -> Result<(), Error>{
//   if email.contains("@") {
//     Ok(())
//   }
//   else {
//     Err(Error::other("email is missing @ symbol"))
//   }
// }

fn extractErrors(text: &str) -> Vec<&str> {
  let splitText = text.split("\n");

  let mut results = vec![];

  for line in splitText {
    if line.starts_with("ERROR") {
      results.push(line);
    }
  }

  return results;
}

fn main() {
  // match divide(5.0, 0.0) {
  //   Ok(result) => {
  //     println!("{}", result)
  //   }
  //   Err(err) => {
  //     println!("{}", err)
  //   }
  // }

  // match validateEmail(String::from("test@example.com")) {
  //   Ok(..) => println!("email is valid"),
  //   Err(errMsg) => println!("{}", errMsg),
  // }

  match fs::read_to_string("logs.txt") {
    Ok(text) => {
      let errorLogs = extractErrors(&text);
      println!("{:#?}", errorLogs)
    }
    Err(errMsg) => {
      println!("{}", errMsg)
    }
  }
}
