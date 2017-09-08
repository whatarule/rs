
use std::iter;

fn add(x: i32, y: &i32) -> i32 {
  x + y
}

/// ```
/// use Function::cons;
/// assert_eq!(1
///   , cons(0, vec![1,2])
///   )
/// ```
fn cons<A>(x: A, y: Vec<A>) -> Vec<A> {
  y
}

fn fold<A,B>(f: fn(A,&B) -> A, acc: A, ls: Vec<B>) -> A {
  ls.iter().fold(acc, f)
}

fn main() {
  println!(
//  "{}", add(1,2)
//  "{}", [0,1,2].iter().fold(0, add)

    "{}", fold(add, 0, vec![1,2,3])
    );
}

macro_rules! compose {
    ( $c:expr, ) => { $c };
    ( $c:expr, . $x:ident $(.$y:ident)* ) => { $x ( compose!($c, $(. $y)* ) ) };
    ( $x:ident $(. $y:ident)* ) => { |arg| $x ( compose!(arg, $(. $y)* ) ) };
}

