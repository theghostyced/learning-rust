enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String)
}

fn main() {
  let tickets: Vec<Ticket> = vec![
    Ticket::Backstage(12.0, "Susan".to_owned()),
    Ticket::Standard(12.0),
    Ticket::Vip(12.0, "Daniel".to_owned())
  ];

  for ticket in tickets {
      match ticket {
          Ticket::Backstage(price, holder) => println!("Backstage: {:?} {:?}", holder, price),
          Ticket::Standard(price) => println!("Standard: {:?}", price),
          Ticket::Vip(price, holder) => println!("Vip: {:?} {:?}", holder, price)
      }
  }
}