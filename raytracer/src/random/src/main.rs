extern crate rand;

use rand::Rng;

fn main(){
  let mut rng =rand::thread_rng();
  // Each thread has an automatically-initialised random number generator:

  // Integers are uniformly distributed over the type's whole range:

  let n1:u8 = rng.gen();
  let n2:u16 = rng.gen();
  println!("n1 is :{}",n1);
  println!("n2 is :{}",n2);
  println!("here is a random u32 number:{}",rng.gen::<u32>());
  println!("here is a random i32 number:{}",rng.gen::<i32>());

  // Floating point numbers are uniformly distributed in the half-open range [0, 1)
  println!("Random float: {}", rng.gen::<f64>());


  //Generates a random value within half-open [0, 10) range (not including 10) with Rng::gen_range.
  println!("Integer: {}", rng.gen_range(0, 10));
  println!("Float: {}", rng.gen_range(0.0, 10.0));



}

