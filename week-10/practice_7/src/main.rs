struct Employee {
    name:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = Employee {
         name:String::from("Agugoesi John"),
        company:String::from("littleBIG Things"),
        age:17
    };
    println!("Name = {} \n",emp1.name);
    println!("Company = {} \n",emp1.company);
    println!("Age = {} \n",emp1.age);
}
