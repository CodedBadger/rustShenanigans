use std::io;

fn main() {
    let mut user_input = String::new();

    println!("Do you want to add anything to your list yes or no?");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    while user_input.trim() == "yes" {
        println!("Do you you want to add anything else?");
        
        user_input.clear();

        io::stdin()
          .read_line(&mut user_input)
          .expect("Failed to read line");
    }
}
