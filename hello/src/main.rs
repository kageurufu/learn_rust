use std::io;

fn main() {
  print!("Please enter your name: ");

  let your_name = io::stdin()
      .read_line()
      .ok()
        .expect("Failed to read line");

  println!("Hello, {}", your_name);
}
