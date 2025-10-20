use std::io;

fn main() {
    
let mut in1 = String::new();
let mut in2 = String::new();
let mut in3 = String::new();

println!("enter first edge of triangle: ");
io::stdin().read_line(&mut in1).expect("not valid");
let a:f32 = in1.trim().parse().expect("not valid");


println!("enter second edge of triangle: ");
io::stdin().read_line(&mut in2).expect("not valid");
let b:f32 = in2.trim().parse().expect("not valid");


println!("enter third edge of triangle: ");
io::stdin().read_line(&mut in3).expect("not valid");
let c:f32 = in3.trim().parse().expect("not valid");

let s:f32 = (a + b + c)/2.0;
let mut area:f32 = s * (s-a) * (s-b) * (s-c);
area = area.sqrt();

println!("area of a triangle {}", area)
}
