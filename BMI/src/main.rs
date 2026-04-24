use std::io;

fn main() {

    let mut weight = String::new();
    let mut height = String::new();
    
    println!("Please enter your weight: \n ");
    io::stdin()
        .read_line(&mut weight)
        .expect("failed to read line");
    println!("Please enter your height in inches: \n ");
    io::stdin()
        .read_line(&mut height)
        .expect("failed to read line");
    let int_weight: f32 = weight
        .trim()
        .parse()
        .expect("That is not a valid number");
    let mut int_height: f32 = height
        .trim()
        .parse()
        .expect("That is not a valid number");

    let int_height_sqr: f32 = int_height.powi(2); 

    let bmi = 703.0 * (int_weight / int_height_sqr);
    println!("{bmi}");
}
