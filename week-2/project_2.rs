fn main(){
	println!("1. Toshiba: 2 quantities X 450,000 
              2. Mac: 1 quantity X 1,500,000 
              3. Hp: 3 quantities X 750,000 
              4. Dell: 3 quantities X 2,850,000 
              5. Acer: 1 quantity X 250,000 
		");

	let toshiba = {2*450_000};
	println!("Total toshiba laptop price: {}", toshiba);
println!("----------");
	let mac = {1*1500_000};
	println!("Total mac laptop price: {}", mac);
println!("----------");
    let hp = {3*750_000};
	println!("Total hp laptop price: {}", hp);
println!("----------");
	let dell = {3*2_850_000};
	println!("Total dell laptop price: {}", dell);
println!("----------");
	let acer = {1*250_000};
	println!("Total acer laptop price: {}", acer);

	let a = toshiba;
	let b = mac;
	let c = hp;
	let d = dell;
	let e = acer;

println!(".......................");
	let total_amount = {a+b+c+d+e};
	println!("Sum of the sales record={}", total_amount);

println!("************************");
	let frequency = {2+1+3+3+1};
	let avg = {total_amount/frequency};
	println!("Average of the sales record={}", avg);
println!("************************");
}