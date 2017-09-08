
/// ```
/// # #[macro_use] extern crate functional;
/// # use functional::functional::*;
/// # fn main() {
/// # assert_eq!(
///   compose!(increment)(1)
/// , 2
/// # );
/// # assert_eq!(
///   compose!(increment . increment)(1)
/// , 3
/// # );
/// # assert_eq!(
///   compose!(1,)
/// , 1
/// # );
/// # assert_eq!(
///   compose!(1, . increment . increment)
/// , 3
/// # );
/// # }
/// ```
#[macro_export]
macro_rules! compose {
    ( $c:expr, ) => { $c };
    ( $c:expr, . $f:ident $(.$g:ident)* ) => { $f ( compose!($c, $(. $g)* ) ) };
    ( $f:ident $(. $g:ident)* ) => { |arg| $f ( compose!(arg, $(. $g)* ) ) };
}


/// ```
/// # use functional::functional::*;
/// # assert_eq!(
///   increment(1)
/// , 2
/// # );
/// ```
pub fn increment(x: i32) -> i32 {
  x + 1
}

/// ```
/// # use functional::functional::*;
/// # assert_eq!(
///   add(1,&2)
/// , 3
/// # );
/// ```
pub fn add(x: i32, y: &i32) -> i32 {
  x + y
}

/// ```
/// # use functional::functional::*;
/// # assert_eq!(
///   cons(0, &vec![1,2])
/// , [0,1,2]
/// # );
/// ```
pub fn cons<A: Clone>(x: A, xs: &Vec<A>) -> Vec<A> {
  let mut v = vec![x];
  v.extend_from_slice(xs);
  v
}

/// ```
/// # use functional::functional::*;
/// # assert_eq!(
///   fold(add, 0, &vec![1,2,3])
/// , 6
/// # );
/// ```
pub fn fold<A,B>(f: fn(A,&B) -> A, acc: A, ls: &Vec<B>) -> A {
  ls.iter().fold(acc, f)
}


/// ```
/// # use functional::functional::*;
/// # assert_eq!(
///   _compose(increment, increment)(1)
/// , 3
/// # );
/// ```
pub fn _compose<A:'static,B:'static,C:'static>(f: fn(B) -> C, g: fn(A) -> B) -> Box<Fn(A) -> C> {
  Box::new(move |x| f(g(x)))
}

pub fn identity<A>(x: A) -> A { x }
// pub fn compose(ls){
//   fold(_compose, identity, ls)
// }

// /// ```
// /// # use functional::functional::*;
// /// # assert_eq!(
// ///
// /// , 1
// /// # );
// /// ```

