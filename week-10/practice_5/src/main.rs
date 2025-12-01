fn main() {
    let x = vec![122,122,122];
    borrow_vector(&x);
    println!("printing the value from main() x[0]={}",x[0]);
    println!("***********************************");
}

fn borrow_vector(z:&Vec<i32>){
    println!("***********************");
    println!("print_vector function {:?}",z);
    println!("-----------------------");
}