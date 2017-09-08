
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
  println!("");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("{}", secret_number);
  println!("");

  let mut guess = String::new();
  io::stdin().read_line(&mut guess)
    .expect ("");
  println!("{}", guess);
}

