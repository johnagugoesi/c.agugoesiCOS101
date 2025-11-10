use std::io;

fn add(a: i32, b:i32) {
    let sum = a+ b;
     println!("sum of A and B = {}",sum);
}

fn main(){
    let mut in1 = String::new();
    println!("enter input for parameter A: ");
    io::stdin().read_line(&mut in1).expect("failed to read input!");
    let a:i32 = in1.trim().parse().expect("invalid input!");

    let mut in2 = String::new();
    println!("enter input for parameter B: ");
    io::stdin().read_line(&mut in2).expect("failed to read input!");
    let b:i32 = in2.trim().parse().expect("invalid input");

    add(a, b);
}


