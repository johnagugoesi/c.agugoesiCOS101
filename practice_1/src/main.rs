use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    println!("\n Please enter your name:");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("failed to read input");
    println!("Your name is: {}", name);

    println!("\n Enter your age:");
    let mut age = String::new();
        io::stdin()
        .read_line(&mut age)
        .expect("failed to read input");
    println!("You are {} years old.", age);

}
