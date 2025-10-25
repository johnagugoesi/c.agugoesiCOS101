use std::io;

fn main() {
    let mut in1 = String::new();
    let mut in2 = String::new();

println!("enter name: ");
io::stdin().read_line(&mut in1).expect("not valid");

println!("enter age: ");
io::stdin().read_line(&mut in2).expect("not valid");
let age:i32 = in2.trim().parse().expect("not valid");

if age >= 18 {
    println!("Welcome to the party! {}", in1);
}
    else {
        println!("Oops, you are not of age to enter the party {}", in1);
    }
}
