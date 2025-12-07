struct Laptop {
    brand: String,
    price: u32,
}

fn main() {
    // Create laptop structs for each brand
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // quantity the customer wants
    let qty = 3;

    
    let total =
        (hp.price * qty) +
        (ibm.price * qty) +
        (toshiba.price * qty) +
        (dell.price * qty);

    println!("Total cost for purchasing 3 laptops from each brand is: ₦{}", total);
}
