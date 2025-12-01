struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = Employee {
        ceo:String::from("Agugoesi John"),
        company:String::from("littleBIG Things"),
        age:17
    };

    let emp2 = Employee {
        ceo:String::from("Sundai Pinchai"),
        company:String::from("Google Inc"),
        age:51
    };

    let emp3 = Employee {
        ceo:String::from("Satya Nadella"),
        company:String::from("Microsoft Corporations"),
        age:56
    };
    display(emp1);
    display(emp2);
    display(emp3);
}
 
 fn display( emp:Employee){
    println!("ceo is {} and his company is {} also his age is age is {} \n",emp.ceo,emp.company,emp.age);
 }

