fn main() {
    let name1 = "Agugoesi Chimdike";
    println!("My name is {}", name1);

    //find and replace
    let name2 = name1.replace("Chimdike","John");
    println!("You can also call me {}", name2);
    let faculty = "Faculty of Science and Technology";

    //finda and replace
    let _school = faculty.replace("Faculty","School");
    println!("I am a student of the {}", _school);
}
