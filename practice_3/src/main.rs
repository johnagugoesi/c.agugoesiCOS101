use std::io;

fn main() {
    let mut in1 = String::new();
    let mut in2 = String::new();

println!("enter base: ");
io::stdin().read_line(&mut in1).expect("not valid");
let b:f32 = in1.trim().parse().expect("not valid");


println!("enter height: ");
io::stdin().read_line(&mut in2).expect("not valid");
let h:f32 = in2.trim().parse().expect("not valid");

if b > 0.0 {
    let area:f32 = (b * h)/2.0;
    println!("area of triangle: {}", area);
}
}
