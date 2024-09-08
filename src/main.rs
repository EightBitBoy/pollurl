use std::time;
use std::thread;

use clap::Parser;


/// waiturl: Wait for a specific HTTP response by continuously polling a URL
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
  /// The URL to poll
  url: String,

  /// The HTTP response status code to wait for, defaults to "200 OK"
  #[arg(short, long, default_value_t = 200)]
  response_code: u8,

  /// Timeout in seconds until the program terminates, runs infinitely otherwise
  #[arg(short, long, default_value_t = 0)]
  timeout: u8,

  /// The interval in seconds between each poll request. This does not include the time for the actual request
  #[arg(short, long, default_value_t = 1)]
  interval: u64
}

fn main() {
  let arguments = Arguments::parse();
  let interval_duration_seconds = time::Duration::from_secs(arguments.interval);

  loop {
    poll();
    thread::sleep(interval_duration_seconds);
  };

  // playground();

  // let mut curl = Easy::new();

  // curl.url("https://www.google.de/").unwrap();
  // curl.perform().unwrap();

  // let mut response_code: u32 = 0;
  // response_code = curl.response_code().unwrap();

  // println!("{}", response_code);

  // for _ in 0..arguments.count {
  // }
}

fn poll() {
  println!("Polling!");
}

#[allow(dead_code)]
fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);
}
