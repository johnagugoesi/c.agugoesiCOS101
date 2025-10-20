use std::io;

fn main() {
    let mut inp =  String::new();

    println!("\nEnter your height (in cm):");
    io::stdin().read_line(&mut inp).expect("not valid");
    let height:f32 = inp.trim().parse().expect("not valid");

    if height >= 150.0 && height <= 170.0 {
    println!("you are an average height person");
}
    else if height >= 170.0 && height <= 195.0{
        println!("you are tall");
    }

        else if height < 150.0 && height <= 100.0{
        println!("you are short");
    }
            else {
                println!("abnormal height!");
            }
}
