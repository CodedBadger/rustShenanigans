use std::fs;
use std::io;
use std::collections::HashMap;


fn main(){
    let mut user_input = String::new();
    let mut entries: HashMap<i32, String> = HashMap::new();

    println!("Do you want to add anything to your list yes or no?");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    while user_input.trim() == "yes" || user_input.trim() != "done" {
    println!("please type your next entry, If Done type done!");
    user_input.clear();
        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    if user_input.trim() == "done" {
        println!("Done adding!");
        break;
        } else {
            write_to_file(&user_input.trim())
        }
    }
}


fn write_to_file() -> std::io::Result<()> {
    fs::write("output.txt", "user_input\n")?;
    Ok(())
}

fn read_file() -> std::io::Result<()> {
    let file_output = fs::read_to_string("output.txt")?;
    println!("{file_output}");
    Ok(())
}
