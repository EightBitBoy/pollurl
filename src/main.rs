// use std::io::{stdout, Write};
// use curl::easy::Easy;

fn main() {
  // playground();

  // let mut curl = Easy::new();

  // curl.url("https://www.google.de/").unwrap();
  // curl.perform().unwrap();

  // let mut response_code: u32 = 0;
  // response_code = curl.response_code().unwrap();

  // println!("{}", response_code);


}

#[allow(dead_code)]
fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);
}
