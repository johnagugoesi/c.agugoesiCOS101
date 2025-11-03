fn main() {
    let name = "Agugoesi John";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni, addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Tech.";
    println!("Department: {}, \nSchool: {}", department,school);
}
