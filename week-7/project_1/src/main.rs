use std::io;

fn main() {
    println!("Shape Area/Volume Calculator");
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice (1-5):");

    let choice = read_input().trim().parse::<i32>().unwrap();

    match choice {
        1 => trapezium(),
        2 => rhombus(),
        3 => parallelogram(),
        4 => cube(),
        5 => cylinder(),
        _ => println!("Invalid choice!"),
    }
}

fn trapezium() {
    println!("Enter height:");
    let height = read_input().trim().parse::<f64>().unwrap();

    println!("Enter base 1:");
    let base1 = read_input().trim().parse::<f64>().unwrap();

    println!("Enter base 2:");
    let base2 = read_input().trim().parse::<f64>().unwrap();

    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}

fn rhombus() {
    println!("Enter diagonal 1:");
    let d1 = read_input().trim().parse::<f64>().unwrap();

    println!("Enter diagonal 2:");
    let d2 = read_input().trim().parse::<f64>().unwrap();

    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

fn parallelogram() {
    println!("Enter base:");
    let base = read_input().trim().parse::<f64>().unwrap();

    println!("Enter altitude:");
    let altitude = read_input().trim().parse::<f64>().unwrap();

    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

fn cube() {
    println!("Enter length of the side:");
    let side = read_input().trim().parse::<f64>().unwrap();

    let area = 6.0 * side * side;
    println!("Area of Cube = {}", area);
}

fn cylinder() {
    println!("Enter radius:");
    let radius = read_input().trim().parse::<f64>().unwrap();

    println!("Enter height:");
    let height = read_input().trim().parse::<f64>().unwrap();

    // Using 3.14159 instead of PI
    let volume = 3.14159 * radius * radius * height;

    println!("Volume of Cylinder = {}", volume);
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
