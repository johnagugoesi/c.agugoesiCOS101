fn main() {
    let mut city : Vec<String> = Vec::new();
    println!("The city vector has element {}",city.len());

    let mut in1 = String::new();
    println!("how many cities do you want to enter");
    std::io::stdin().read_line(&mut in1).expect("Failed to read input");
    let city_num:i32 = in1.trim().parse().expect("invalid input");

    for count in 0..city_num{
        let mut in2 = String::new();
        println!("Enter city {}",count+1);
        std::io::stdin().read_line(&mut in2).expect("failed to read input");
        let new_city:String = in2.trim().parse().expect("invalid input");
        city.push(new_city);
    }
    println!("your preferred cities are: \n");
    let mut count=1;

    for i in city {
        println!("{} {}",count,i);
        count+=1;
    }
}
