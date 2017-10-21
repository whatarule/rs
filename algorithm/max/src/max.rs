
fn foldl<A,B>(f: fn(A,&B) -> A, initial: A, v: &Vec<B>) -> A {
  let mut range = 0..v.len();
  let mut acc = initial;
  loop {
    match range.next() {
      Some(x) => { acc = f(acc, &v[x]) }
    , None => { break }
    }
  }
  acc
}


/// ```
/// use max::max::*;
/// assert_eq!(max3(1,2,3), 3);
/// ```
pub fn max3(x: i32, y: i32, z: i32) -> i32 {
  let x_gt_y: bool = x > y;
  let y_gt_z: bool = y > z;
  let z_gt_x: bool = z > x;
  if x_gt_y && !z_gt_x { x }
  else if y_gt_z { y }
  else { z }
}

/// ```
/// use max::max::*;
/// assert_eq!(min3(1,2,3), 1);
/// ```
pub fn min3(x: i32, y: i32, z: i32) -> i32 {
  let x_gt_y: bool = x > y;
  let y_gt_z: bool = y > z;
  let z_gt_x: bool = z > x;
  if !x_gt_y && z_gt_x { x }
  else if !y_gt_z { y }
  else { z }
}

/// ```
/// use max::max::*;
/// assert_eq!(med3(1,2,3), 2);
/// ```
pub fn med3(x: i32, y: i32, z: i32) -> i32 {
  let x_gt_y: bool = x > y;
  let y_gt_z: bool = y > z;
  let z_gt_x: bool = z > x;
  if x_gt_y && z_gt_x { x }
  else if !x_gt_y && !z_gt_x { x }
  else if y_gt_z && x_gt_y { y }
  else if !y_gt_z && !x_gt_y { y }
  else { z }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn max3_test() {
    assert_eq!(test_123(max3, 3), true);
    assert_eq!(test_223(max3, 3), true);
    assert_eq!(test_233(max3, 3), true);
    assert_eq!(test_333(max3, 3), true);
  }

  #[test]
  fn min3_test() {
    assert_eq!(test_123(min3, 1), true);
    assert_eq!(test_223(min3, 2), true);
    assert_eq!(test_233(min3, 2), true);
    assert_eq!(test_333(min3, 3), true);
  }

  #[test]
  fn med3_test() {
    assert_eq!(test_123(med3, 2), true);
    assert_eq!(test_223(med3, 2), true);
    assert_eq!(test_233(med3, 3), true);
    assert_eq!(test_333(med3, 3), true);
  }

  fn test_123(f: fn(i32,i32,i32) -> i32, result: i32) -> bool {
    true
    && f(1,2,3) == result
    && f(2,3,1) == result
    && f(3,1,2) == result
  }
  fn test_223(f: fn(i32,i32,i32) -> i32, result: i32) -> bool {
    true
    && f(2,2,3) == result
    && f(2,3,2) == result
    && f(3,2,2) == result
  }
  fn test_233(f: fn(i32,i32,i32) -> i32, result: i32) -> bool {
    true
    && f(2,3,3) == result
    && f(3,2,3) == result
    && f(3,3,2) == result
  }
  fn test_333(f: fn(i32,i32,i32) -> i32, result: i32) -> bool {
    true
    && f(3,3,3) == result
  }
}

