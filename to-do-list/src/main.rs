use std::fs;
use std::io;
use std::collections::HashMap;


fn main(){
    let mut user_input = String::new();

    println!("Do you want to add anything to your list yes or no?");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    while user_input.trim() == "yes" {
    println!("please type your next entry, If Done type done!");
    user_input.clear();
        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    if user_input.trim() == "done" {
        println!("Done adding!");
        break;
        }
    }
}


fn write_to_file() -> std::io::Result<()> {
    fs::write("output.txt", "Hello, Rust!\n")?;
    Ok(())
}

fn read_file() -> std::io::Result<()> {
    let file_output = fs::read_to_string("output.txt")?;
    println!("{file_output}");
    Ok(())
}
