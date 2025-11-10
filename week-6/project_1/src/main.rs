use std::io;

fn main() {
    println!("*__PLEASE MAKE YOUR ORDER!!!__*");
    println!("P = POUNDO YAM & EDINKAIKO SOUP___________N3200");
    println!("F = FRIED RICE & CHICKEN__________________N3000");
    println!("A = AMALA & EWEDU SOUP____________________N2500");
    println!("E = EBA & EGUSI SOUP______________________N2000");
    println!("W = WHITE RICE & STEW_____________________N2500");

println!("what would you like to purchase? ");
let mut in1 = String::new();
    io::stdin().read_line(&mut in1).expect("input valid statement");
    let order = in1.trim().to_uppercase();

let mut cost = 0.0;

if order ==  "P"{
     cost = 3200.0;
} else if order == "F"{
     cost = 3000.0;
} else if order == "A"{
     cost = 2500.0;
} else if order == "E"{
     cost = 2000.0;
} else if order == "W"{
     cost = 2500.0;
} else {
    println!("invalid input!")
}

println!("how many servings?");
let mut in2 = String::new();
    io::stdin().read_line(&mut in2).expect("input valid statement");
    let quantity:f32 = in2.trim().parse().expect("not an integer");

let tcost = quantity * cost;
println!("Total cost of purchase for {} amount of {}= {}",quantity, order, tcost);

//discount
if tcost > 10000.0 {
    let x = 0.05 * tcost;
     let discount = tcost - x ;
     println!("You've been given a discount of {} ",discount);
println!("Actual amount= {}", discount);
}else{
println!("No discount creditted");
println!("Actual amount= {}", tcost);

}


}
