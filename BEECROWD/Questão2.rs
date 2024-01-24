use std::io;
fn main()
{
let mut a = String::new();
let mut b = String::new();
io::stdin().read_line(&mut a);
io::stdin().read_line(&mut b);
let a: i32 = a.trim().parse().unwrap();
let b: i32 = b.trim().parse().unwrap();
let resultado = a + b;
println!("X={}",a+b);
}