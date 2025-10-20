use std::io;

fn main() {
    println!("enter a number: ");
    let mut in1 = String::new();
    io::stdin().read_line(&mut in1).expect("not valid");
    let mut num:i32 = in1.trim().parse().expect("not valid");

    while num < 10 {
        println!("inside loop number value is {}",num );
        num+=1
    }
    println!("outside loop number value is {}",num );
}