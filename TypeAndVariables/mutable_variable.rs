use std::mem;

fn main() {
  let mut b:i8 = 0;
  println!("b = {}", b);
  b = 42;
  println!("b = {}", b);

  let c = 123456789;
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

  let z:isize = 128;
  let size_of_z = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit OS", 
    z, size_of_z, size_of_z * 8);

    let d = "z";
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e:f64 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // true and false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
