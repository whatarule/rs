
fn main() {
  let v = vec![1,2,3];
  take(v);
//println!("{}", v[0]);

  let v = 1;
  let _v2 = v;
  println!("{}", v);

  let a = 5;
  let _y = double(a);
  println!("{}", a);

  let a = true;
  let _y = change_truth(a);
  println!("{}", a);

}

fn double(x: i32) -> i32 {
  x * 2
}

fn change_truth(x: bool) -> bool {
  !x
}

fn take(_v: Vec<i32>) {
  ()
}

fn _foo() {
  let v = vec![1,2,3];
  let _v2 = v;
//println!("{}", v[0]);
}


