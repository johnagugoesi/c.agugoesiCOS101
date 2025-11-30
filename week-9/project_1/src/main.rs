use std::io::Write;

fn main() {
   let mut file = std::fs::File::create("nigerianbrews.txt").expect("create failed");
   let lager = "LAGER        : 33 Export| Desperados   | Goldberg   | Gulder  | Heineken | Star \n";
   let stout = "STOUT        : Legend   | Turbo King.  | Williams   |         |          |      \n";
   let n_alc = "NON-ALCOHOLIC: Maltina  | Amstel Malta | Malta Gold | Fayrouz |          |        ";

   file.write_all(lager.as_bytes()).expect("write failed");
   file.write_all(stout.as_bytes()).expect("write failed");
   file.write_all(n_alc.as_bytes()).expect("write failed");
 
print!("Nigerian Breweriers File created successfully!!")

}
