
fn main() {

  let mut _y = 5;
  let _x = _y = 6;

  //_x
  //_diverges();
  //let _x: i32 = _diverges();
  //let _x: String = _diverges();

  let f: fn(i32) -> i32;
  f = plus_one;
  println!("{}", f(5));

}

fn plus_one(i: i32) -> i32 {
  i + 1
}

fn _diverges() -> ! {
  panic!("diverges");
}

