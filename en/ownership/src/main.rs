fn main() {
    let x = 1;
    // 下記の行で所有権がムーブしてしまいそうだが…
    let y = x;
    println!("{:?}", y); // => 1
    // 数値はCopyな値なので一度使ったあともまた使える。
    println!("{:?}", x); // => 1

    // &strもstrへの参照なのでCopyな値
    let a = "abc";
    let b = a;
    println!("{}", a); // => abc
}
