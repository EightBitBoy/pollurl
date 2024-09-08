use std::time;
use std::thread;
use std::process;

use clap::Parser;

//TODO Create an enum of exit codes
//https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html


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
  timeout: u64,

  /// The interval in seconds between each poll request. This does not include the time for the actual request
  #[arg(short, long, default_value_t = 1)]
  interval: u64

  //TODO -q arg to suppress output during polling
}

fn main() {
  let arguments = Arguments::parse();
  let interval_duration_seconds = time::Duration::from_secs(arguments.interval);
  let start_time = time::Instant::now();

  loop {
    poll(&arguments.url, arguments.response_code);

    if arguments.timeout > 0 && start_time.elapsed().as_secs() >= arguments.timeout {
      //TODO Use some red color!
      println!("Timeout after {} seconds", arguments.timeout);
      process::exit(1);
    }

    thread::sleep(interval_duration_seconds);
  };

  // playground();

  // let mut curl = Easy::new();
  // curl.url("https://www.google.de/").unwrap();
  // curl.perform().unwrap();
  // response_code = curl.response_code().unwrap();
}

fn poll(url: &String, response_code: u8) {
  println!("{}: 404, expected {}", url, response_code);
}

#[allow(dead_code)]
fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);

  for _ in 0..2 {
  }
}
