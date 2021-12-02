
fn cartesian_coordinate() -> (i32, i32) {
  return (10, 20)
}

fn main() {
    let (x, y) = cartesian_coordinate();

    if y > 5 {
      println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
      println!("=5")
    }
}