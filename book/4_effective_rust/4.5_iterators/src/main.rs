fn main() {
  //  _for_loop();
  let nums = vec![1, 2, 3];
  _vec_loop(&nums);

  let _vec_100 =(1..101).collect::<Vec<i32>>();
  let _vec_100 =(1..101).collect::<Vec<_>>();

  let _gt_42 = (0..100).find(|&x| x > 42);
  match _gt_42 {
    Some(_) => println!("found")
  , None => println!("not found")
  }

  let _sum = (1..4).fold(0, |sum, x| sum + x);
  //  (1..100).map(|x| x + 1);
  //  (1..100).map(|x| println!("{}", x));
  for i in (1..).take(5).filter(|&x| x % 2 == 0) {
    println!("{}", i);
  }

  let nums = (1..).take(10)
    .filter(|x| x % 2 == 0)
    .filter(|x| x % 3 == 0)
    .collect::<Vec<_>>();
  _vec_loop(&nums);
}

fn _vec_loop(nums: &Vec<i32>) {
  for &num in nums.iter() {
    println!("{}", num);
  }
}

fn _for_loop() {
  //  for x in 0..10 {
  //    println!("{}", x);
  //  }

  let mut range = 0..10;
  loop {
    match range.next() {
      Some(x) => {
          println!("{}", x);
      }
    , None => { break }
    }
  }
}

