fn pair<T, S>(t: T, s: S) -> (T, S) { (t, s) }

fn main() {
    // T = i32, S = f64で呼び出す
    let i = pair(1, 1.0);

    // 型を明示する方法もある
    let i = pair::<isize, f64>(1, 1.0);

    // T = &str, S = Stringで呼び出す
    let s = pair("str", "string".to_string());
}
