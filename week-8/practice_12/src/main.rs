fn main() {
   let mut colors = ["red","green","yellow","white"];
    println!("\n original array = {:?}",colors);

    let sliced_colors = &mut colors[1..3];

    println!("first slice = {:?}",sliced_colors);

    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);
}
