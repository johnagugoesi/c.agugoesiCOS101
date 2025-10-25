use std::io;

fn main() {
    println!("enter lower bound");
    let mut in1 = String::new();
    io::stdin().read_line(&mut in1).expect("not valid");
    let lowerb:i32 = in1.trim().parse().expect("not valid");

    println!("enter upper bound");
    let mut in2 = String::new();
    io::stdin().read_line(&mut in2).expect("not valid");
    let upperb:i32 = in2.trim().parse().expect("not valid");

    for x in lowerb..upperb { println!("count level is {}",x );
    }
}
