
fn main() {
  let v1 = vec![1,2,3];
  let v2 = vec![1,2,3];
  println!("{}", foo(&v1, &v2));
  let _v1 = v1;
  let _v2 = v2;


  let mut _x = 5;
  {
    let _y = &_x;
    //*y = *y + 1;
  }
  println!{"{}", _x};

  let mut x = 5;
  {
    let y = &mut x;
    *y = *y + 1;
  }
  println!{"{}", x};

  let mut v = vec![1,2,3];
  for i in &v {
    println!("{}", i);
    //v.push(34);
    }
  v.push(34);

  let _y: &i32;
  {
    let _x = 5;
  //y = &x;
  //println!("{}", y);
  }
  //println!("{}", y);

  //let y: &i32;
    let x = 5;
  let y: &i32;
    y = &x;
  println!("{}", y);

}

fn foo(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
  42
}

