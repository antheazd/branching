use std::io;
fn main() {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u32 = input.trim().parse().expect("Please input a number");
    
    if number > 1 {
        println!("Number is bigger than 1");
    }
    else if number < 1 {
        println!("Number is smaller than 1");
    }
    else if number == 1 {
        println!("Number is equal to 1");
    }
}
