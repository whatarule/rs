struct Celsius(f64);
struct Kelvin(f64);

// `impl 型名 {..}`で型に対する実装を書ける
impl Celsius {
    // `{..}`の中には関数が書ける。
    // 第一引数が`self`、`&mut self` `&self`, `Box<self>`の場合はメソッドとなる
    fn to_kelvin(self) -> Kelvin {
        // selfを通じてフィールドにアクセスできる。
        Kelvin(self.0 + 273.15)
    }

    // 第一引数が`self`系でない場合は関連関数となる
    fn from_kelvin(k: Kelvin) -> Self {
        Celsius(k.0 - 273.15)
    }
}

fn main() {
    let absolute_zero = Kelvin(0.0);
    let triple_point = Celsius(0.0);
    // 関連関数は`型名::関数名(引数)`で呼び出す。
    let celsius = Celsius::from_kelvin(absolute_zero);
    // メソッドは`値.関数名(引数)`で呼び出す。
    let kelvin = triple_point.to_kelvin();
}
