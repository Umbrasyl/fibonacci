use std::io::stdin;

fn main() {
    println!("Welcome to the Fibonacci program.");
    println!("Enter a number for which Fibonacci number you want to learn.");
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");
    let user_input: i32 = user_input
        .trim()
        .parse()
        .expect("Failed to parse into integer.");
    let mut n_minus_one = 0;
    let mut nth_fibonacci = 1;
    let mut holder: i32;
    let mut counter = 2;
    while counter < user_input {
        holder = nth_fibonacci;
        nth_fibonacci = n_minus_one + nth_fibonacci;
        n_minus_one = holder;
        counter += 1;
    }
    println!("The {user_input}th fibonacci number is: {nth_fibonacci}");
}
