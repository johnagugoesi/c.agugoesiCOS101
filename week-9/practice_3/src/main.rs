use std::fs;

fn main() {
    //first you crrate the text file that's gonna be removed!
    let _file = std::fs::File::create("data.txt").expect("create failed");
    fs::remove_file("data.txt").expect("could not remove this file");
    println!("file is removed");
}
