

struct Customer {
  age: i32,
}

fn make_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
      return Err("Customer is less than 21 years".to_owned())
    }

    Ok(())
}

fn main() {
    let customer = Customer {
      age: 22,
    };

    let result =  make_purchase(&customer);

    match result {
        Ok(_) => println!("User was able to make a purchase"),
        Err(e) => println!("{}", e)
    }
}