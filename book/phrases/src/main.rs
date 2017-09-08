extern crate phrases;

use phrases::english::{greetings, farewells};
use phrases::spanish;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Spanish: {}", spanish::hello());
    println!("Goodbye in Spanish: {}", spanish::goodbye());
}
