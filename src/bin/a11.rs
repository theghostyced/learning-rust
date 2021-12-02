
struct Grocery {
    quantity: i32,
    id: i32,
}

fn print_quantity(grocery: &Grocery) {
    println!("{:?}", grocery.quantity);
}

fn print_id(grocery: &Grocery) {
    println!("{:?}", grocery.id);
}

fn main() {
    let grocery: Grocery = Grocery {
        quantity: 3,
        id: 23
    };

    // The & (ampersand) is something used to
    // borrow the reference of the grocery variable
    print_quantity(&grocery);
    print_id(&grocery);
}