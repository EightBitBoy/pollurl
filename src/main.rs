use clap::Parser;
// use std::io::{stdout, Write};
// use curl::easy::Easy;

/// waiturl: Wait for a specific HTTP response by continuously polling a URL.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
  // url: Option<String>,
  /// The URL to poll
  url: String,

  /// Timeout in seconds until the program terminates, runs infinitely otherwise
  #[arg(short, long, default_value_t = 0)]
  timeout: u8,

  /// The interval in seconds between each poll request
  #[arg(short, long, default_value_t = 1)]
  interval: u8,

  // TODO
  // Timeout for each request
}

fn main() {
  // playground();

  // let mut curl = Easy::new();

  // curl.url("https://www.google.de/").unwrap();
  // curl.perform().unwrap();

  // let mut response_code: u32 = 0;
  // response_code = curl.response_code().unwrap();

  // println!("{}", response_code);

  let arguments = Arguments::parse();
  println!("Hello {}!", arguments.timeout);
  // for _ in 0..arguments.count {
  // }
}

#[allow(dead_code)]
fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);
}
