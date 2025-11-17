fn main() {
    let mut mountain_heigths = ("everest", 8848, "fishtail", 6993);

    println!("original tuple = {:?}", mountain_heigths);

    mountain_heigths.2 = "Lhotse";
    mountain_heigths.3 = 8516;

    println!("changed tuple = {:?}", mountain_heigths);
}
