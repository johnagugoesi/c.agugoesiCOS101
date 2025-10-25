use std::io;

fn main() {
    println!("WE ARE GONNA FIND THE ROOTS OF THE QUADRATIC EQUATION!!");

    let mut in1 = String::new();
    println!("enter the first value 'a' ");
    io::stdin().read_line(&mut in1).expect("can't read input, try again");
    let a:f32= in1.trim().parse().expect("please input an integer");

    let mut in2 = String::new();
    println!("enter second value 'b' in ax^2+bx+c ");
    io::stdin().read_line(&mut in2).expect("can't read input, try again");
    let b:f32=in2.trim().parse().expect("please input an integer");

    let mut in3 = String::new();
    println!("enter the third value 'c':");
    io::stdin().read_line(&mut in3).expect("can't read input, try again");
    let c:f32 = in3.trim().parse().expect("please input an integer");

    let d:f32 = b * b - 4.0 * a * c;
    let x1 = (-b + d.sqrt())/(2.0 * a);
    let x2 = (-b - d.sqrt())/(2.0 * a);
    println!("the roots of the formula {}x^2+{}x+{} are: {} and {}", a, b, c, x1, x2);

    if d < 0.0 {
        println!("The discriminant is negative, therefore there are no real roots!");
    } else if d > 0.0 {
        println!("The disciminant is positive, therefore there are two distinct roots!");
    } else if d == 0.0{
        println!("The disciminant is zero, therefore there is exactly one real root!");
    } else {
        println!("math error!");
    }

}
