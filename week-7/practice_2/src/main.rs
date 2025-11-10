use std::io;

fn checker() {
    let mut in1 = String::new();
    println!("enter character:");
    io::stdin().read_line(&mut in1).expect("not valid");
    let ch:char = in1.trim().parse().expect("invalid input");

if ch >= '0' && ch <= '9'{
    println!("character '{}' is a digit",ch);
}else {
     println!("character '{}' is not a digit",ch);
}
}


fn main() {
    println!("Welcome this program checks whether the character is a digit or not!!");
    checker();
}
