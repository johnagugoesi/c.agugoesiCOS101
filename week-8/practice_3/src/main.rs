fn value(ch:Option<&char>) {
    println!("Element of vector {:?}",ch);
}


fn main() {
    let v = vec!['R','U','S','T','A','C','I','A','N'];

    let mut in1 = String::new();
    println!("\n Enter an index value between (0 - 8)");
    std::io::stdin().read_line(&mut in1).expect("invalid input");

    let index:usize = in1.trim().parse().expect("invalid input");

    let ch:Option<&char> = v.get(index);
    value(ch);

}
