fn main(){
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.00;

		//compound interest formula: A-P
		//where A = p(1 + (r/100))^n
	let a = p * (1.0+(r/100.0)).powf(n);
	println! ("amount = {}", a);

	let ci = a-p;
	println!("compund interest of Ibeju Local Government is {}", ci);
}