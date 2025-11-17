fn main() {
    let v = vec!['C','O','M','P','U','T','E','R']

    let mut in1 = String::new();

    println!("Enter am index value between (0 - 7)");
    std::io::stdin().read_line(&mut in1).expect("failed to read input");

    let index:usize = in1.trim().parse().expect("invalid input");

    let ch:char = v[index];
    print!("{} is the character for index [{}]\n",ch,index);
}
