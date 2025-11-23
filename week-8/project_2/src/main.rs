use std::io;

fn main() {
    let mut num_input = String::new();

    println!("Enter number of developers interviewed:");
    io::stdin().read_line(&mut num_input).unwrap();
    let count: usize = num_input.trim().parse().unwrap();

    let mut names: Vec<String> = Vec::new();
    let mut years: Vec<u32> = Vec::new();

    for i in 0..count {
        let mut name = String::new();
        let mut exp = String::new();

        println!("Enter developer {} name:", i + 1);
        io::stdin().read_line(&mut name).unwrap();

        println!("Enter years of experience for {}:", name.trim());
        io::stdin().read_line(&mut exp).unwrap();

        let exp_val: u32 = exp.trim().parse().unwrap();

        names.push(name.trim().to_string());
        years.push(exp_val);
    }

    // Find highest experience
    let mut highest_index = 0;
    for i in 1..count {
        if years[i] > years[highest_index] {
            highest_index = i;
        }
    }

    println!("\nDeveloper with the highest experience:");
    println!("Name: {}", names[highest_index]);
    println!("Years of Experience: {}", years[highest_index]);
}
