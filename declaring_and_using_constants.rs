const MEANING_OF_LIFE:u8 = 45; // no fixed address
static mut Z:i32 = 123;

fn main() {
  println!("{:?}", MEANING_OF_LIFE);
  // println!("{}", Z);

  unsafe
  {
  	Z = 777;
  	println!("unsafe z {}", Z);
  }
}