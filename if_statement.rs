fn if_statement()
{
	let temp = 5;

	if temp > 30
	{
		println!("readlly hot outside");
	} else if temp < 10
	{
		println!("really cold!");
	}
	else 
	{
		println!("temperature is ok");
	}

	let day = if temp > 20 {"sunny"} else {"cloudy"};
	println!("today is {}", day);

	println!("is it {}", 
		if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"});
}

fn main() {
	if_statement();
}