use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    let mut hello = "Hello Class\n";
    let mut append = "This is the appendage to the document.";
    
    file.write_all(hello.as_bytes()).expect("write failed");
    file.write_all(append.as_bytes()).expect("write failed");
    println!("file append success");

}
