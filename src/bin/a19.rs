use std::collections::HashMap;


fn main() {
  let mut store: HashMap<&str, i32> = HashMap::new();
  let mut total_quantity = 0;

  store.insert("Chairs", 5);
  store.insert("Beds", 3);
  store.insert("Tables", 2);
  store.insert("Couches", 0);

  for (item, qty) in store.iter() {
      total_quantity += qty;
      match qty {
          0 => println!("Item: {:?}, Out of stock!", item),
          _ => println!("Item: {:?}, Quantity: {:?}", item, qty)
      }
  }

  println!("Total quantity: {:?}", total_quantity)
}