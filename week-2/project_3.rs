fn main(){
	let p:f64 = 510_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.00;

		//compound interest formula: A-P
		//where A = p(1 + (r/100))^n
	let a = p * (1.0-(r/100.0)).powf(n);
	println! ("value of the tv after 3yrs = {}", a);
}