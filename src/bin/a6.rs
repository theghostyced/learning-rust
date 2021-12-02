fn main() {
  let mut countdown = 5;

  while countdown >= 1 {
    println!("{:?}", countdown);

    countdown = countdown - 1;
  }

  println!("done!")
}