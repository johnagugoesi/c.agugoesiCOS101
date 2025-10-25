use std::io;

fn main() {
    println!("ANNUAL INCENTIVE DETERMINATOR!");

    let mut in1 = String::new();
    println!("Enter name of employee: ");
    io::stdin().read_line(&mut in1).expect("enter a valid input");

    let mut in2 = String::new();
    println!("Enter age of employee: ");
    io::stdin().read_line(&mut in2).expect("enter a valid input");
    let _age:i32=in2.trim().parse().expect("enter a valid age");

    let mut in3 = String::new();
    println!("Are you Experienced or Not?\n");
    println!("if you are experienced type 'yes' if not type 'no'\n");
    io::stdin().read_line(&mut in3).expect("enter a valid input");
    
    let exp = in3.trim().to_lowercase();

    if exp == "yes" {
        println!("You are experienced!");
    } else if exp == "no"{
        println!("You are not experienced"); 
    } else {
        println!("please enter 'yes' or 'no'");();
    }

    if exp == "yes" && _age >= 40 { 
        println!("Your incentive is N1_560_000"); 
    }
    
    else if exp == "yes" && _age <= 40 && _age >= 30{ 
         println!("Your incentive is N1_480_000");
     } 

    else if exp == "yes" && _age <28 {
        println!("Your incentive is N1_300_000");
    }
                    
    else if exp == "no" {
        println!("Your incentive is N100_000");
    } 
                        else{
                            println!("You are not valid to receive an incentive.");
                        }


}
