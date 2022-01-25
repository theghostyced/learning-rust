use std::io;

fn main() {
    println!("Ready for the game??");

    println!("Guess a number for me...");

    let mut guessed_num = String::new();


    // Implementation 1

    // io::stdin()
    //     .read_line(&mut guessed_num)
    //     .expect("Failed to process input");

    // println!("You guessed: {}", guessed_num);



    // Implementation 2

    match io::stdin().read_line(&mut guessed_num) {
        Ok(n) => {
            println!("You guessed: {}", guessed_num);

            println!("Number in bytes: {}", n);
        }

        Err(e) => println!("An error occured while performing the operation: {}", e),
    }


}
