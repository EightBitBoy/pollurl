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

  /// The interval in seconds between each poll request
  #[arg(short, long, default_value_t = 1)]
  interval: u8
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
