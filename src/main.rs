use clap::Parser;
// use std::io::{stdout, Write};
// use curl::easy::Easy;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
  url: Option<String>,

  /// Name of the person to greet
  #[arg(short, long)]
  name: String,

  /// Number of times to greet
  #[arg(short, long, default_value_t = 1)]
  count: u8,
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
  for _ in 0..arguments.count {
    println!("Hello {}!", arguments.name);
  }
}

#[allow(dead_code)]
fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);
}
