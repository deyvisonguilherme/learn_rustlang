fn match_statement(){
	let contry_code = 44;
	let contry = match contry_code
	{
		44 => "UK",
		46 => "Sweden",
		7  => "Russian",
		1...999  => "Unknown",
		_ => "Invalid"
	};
	println!("the contry with code {} is {}", contry_code, contry);
}

fn main(){
	match_statement();
}