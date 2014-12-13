fn main() {
  for x in range(0i, 10i) {
    printnum("x", x);
  }
}

fn printnum(name: &str, val: int) {
  println!("{} = {}", name, val);
}
