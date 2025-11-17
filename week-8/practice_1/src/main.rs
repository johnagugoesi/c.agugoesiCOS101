fn main() { 
  let v: Vec<i64> = Vec::new();

  println!("\nThe length of Vec::new is: {}", v.len());

  let v = vec!["Grace","Effi","Basil","Kareem","Susan"];

  println!("\nThe length of the vec macro is : {}",v.len());
}
