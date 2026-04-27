use std::io;
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    let mut i = 1;
    //scores.insert(String::from("Bob"), 37);
    //scores.insert(String::from("James"), weight);

   while i <= 3 {
        let mut exercise = String::new();
        let mut weight_input = String::new();
        
        println!("Enter an exercise!");
        io::stdin()
            .read_line(&mut exercise)
            .expect("failed to read line");
        
        println!("Enter a weight!");
        io::stdin()
            .read_line(&mut weight_input)
            .expect("failed to read line");

       let weight: i32 = weight_input.trim().parse().expect("Please enter a valid number");
       
       scores.insert(String::from(exercise), weight);
       i += 1;
     }
    dbg!(scores);
   }
