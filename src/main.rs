fn main() {
  playground();
}

fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);
}
