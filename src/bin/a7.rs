
enum Color {
  Red,
  Black,
  Green,
  Purple
}

fn print_color(color: Color) {
  match color {
    Color::Black => println!("black"),
    Color::Green => println!("green"),
    Color::Purple => println!("purple"),
    Color::Red => println!("red"),
  }
}

fn main() {
  print_color(Color::Red);
  print_color(Color::Green);
  print_color(Color::Purple);
  print_color(Color::Black);
}