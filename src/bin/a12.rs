
enum Color {
    Black,
    Brown
}

impl Color {
    fn print(&self) {
      match self {
          Color::Black => println!("Black"),
          Color::Brown => println!("Brown"),
      }
    }
}

struct Dimension {
  width: f64,
  height: f64,
  depth: f64,
}

impl Dimension {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
  width: f64,
  color: Color,
  dimensions: Dimension,
}

impl ShippingBox {
    fn new(width: f64, color: Color, dimensions: Dimension) -> Self {
        Self {
          width,
          color,
          dimensions
        }
    }

    fn print(&self) {
      self.color.print();
      self.dimensions.print();
      println!("{:?}", self.width);
    }
}

fn main() {
  let dimension = Dimension {
    width: 20.0,
    height: 10.0,
    depth: 2.0,
  };

  let small_box = ShippingBox::new(20.0, Color::Brown, dimension);

  small_box.print();
}