fn while_and_loop()
{
	let mut x:i32 = 1;	
	while x < 1000
	{
		x *= 2;
		if x == 64 { continue; }
		println!("x = {}", x);
	}

	let mut y:i32 = 1;
	loop
	{
		y *= 2;
		println!("y = {}", y);
		if y == 1<<10 { break; }
	}
}

fn main() {
	while_and_loop();
}