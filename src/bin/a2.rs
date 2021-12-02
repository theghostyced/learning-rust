
fn sum(a: i32, b: i32) -> i32 {
  a + b
}

fn display_result(num: i32) {
  println!("{:?}", num);
}

fn main() {
  display_result(sum(2, 6));
}
