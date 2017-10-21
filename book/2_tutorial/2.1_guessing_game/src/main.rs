
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("{}", secret_number);
  println!("");

  let mut guess = String::new();
  io::stdin().read_line(&mut guess)
    .expect("failed to read line");
  println!("{}", guess);

  let guess: u32 =
    match guess.trim().parse() {
      Ok(num) => num
    , Err(_) => panic!("type a number")
    };

  match guess.cmp(&secret_number) {
    Ordering::Less    => println!("less")
  , Ordering::Greater => println!("greater")
  , Ordering::Equal   => println!("equal")
  }
}

