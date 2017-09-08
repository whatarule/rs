
/// ```
/// # #[macro_use] extern crate macros;
/// # fn main() {
/// assert_eq!(
///     foo!(y => 3)
///   , ()
///   )
/// # }
/// ```
#[macro_export]
macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

// /// ```
// /// use ::*;
// /// assert_eq!(
// ///   , 1
// ///   )
// /// ```
