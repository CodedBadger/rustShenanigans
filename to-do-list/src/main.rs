use std::fs;
fn main(){
    write_to_file();
    read_file();
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
