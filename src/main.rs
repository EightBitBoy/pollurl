use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
  // playground();

  let mut curl = Easy::new();

  curl.url("https://www.sfffdsdafr3wfd.org/").unwrap();
  curl.perform().unwrap();

  let response_code = -1;
  response_code = curl.response_code().unwrap();

  println!("{}", response_code);
}

fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);
}
