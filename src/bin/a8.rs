

enum DrinkFlavor {
  Orange,
  Pineapple
}

struct Drink {
  flavor: DrinkFlavor,
  ounce: f64
}

fn print_drink_info(drink: Drink) {
  match drink.flavor {
    DrinkFlavor::Orange => println!("Orange flavor"),
    DrinkFlavor::Pineapple => println!("Pineapple flavor"),
  }

  println!("Ounce is: {:?}", drink.ounce)
}

fn main() {
  let orange_drink: Drink = Drink {
    flavor: DrinkFlavor::Orange,
    ounce: 3.02
  };

  let pineapple_drink: Drink = Drink {
    flavor: DrinkFlavor::Pineapple,
    ounce: 1.02
  };

  print_drink_info(orange_drink);
  print_drink_info(pineapple_drink);
}