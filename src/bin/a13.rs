
fn main() {
    let numbers = vec![10, 20, 30, 40];

    for number in &numbers {
      match number {
          30 => println!("thirty"),
          _ => println!("{:?}", number)
      }
    }

    println!("length of vector is: {:?}", numbers.len())
}