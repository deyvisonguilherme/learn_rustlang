fn main() {
  operators();	
}

fn operators() {

	// Operations arithmetic
	let mut a = 2 + 3 * 4;
	println!("a = {}", a);
	a = a + 1;
	a -= 2;
	println!("remainder of {} / {}  = {}", a, 3, (a % 3));
	let a_cubed = i32::pow(a, 3);
	println!("{} cubed is {}", a, a_cubed);

	let b = 2.5;
	let b_cubed = f64::powi(b, 3);
	let b_to_pi = f64::powf(b, std::f64::consts::PI);
	println!("{} cubed = {}, {}^PI = {}",b, b_cubed, b, b_to_pi);

	// bitwise
	let c = 1 | 2;
	println!("1|2 = {}", c);


	let two_to_10 = 1 << 10;
	println!("2^10 = {}", two_to_10);

	// Logical
	let pi_less_4 = std::f64::consts::PI < 4.0;
	println!("pi_less_4 = {}", pi_less_4);
	
	let x  = 5;
	let x_is_5 = x == 5; // true
	println!("x = {} | x_is_5 = {}", x, x_is_5);
}