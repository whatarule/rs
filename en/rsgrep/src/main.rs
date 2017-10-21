// regexを宣言
extern crate regex;
// regexからRegex型をインポート
use regex::Regex;

// stdクレートのfsモジュールにあるFile型をインポート。以後は`File`として参照できる。
use std::fs::File;

// 同じモジュールから複数インポートする際は`{}`でまとめて指定できる。
use std::io::{BufReader, BufRead};

// モジュール自体をインポートすることもできる。
use std::env;

fn usage() {
    println!("rsgrep PATTERN FILENAME")
}

fn main() {
    // 引数からパターンを取り出す
    let pattern = match env::args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };
    // 取り出したパターンから`Regex`をあらためて作る
    // 無効な正規表現だった場合などにはエラーが返る
    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(e) => {
            println!("invalid regexp {}: {}", pattern, e);
            return
        }
    };

    // envモジュールのargs関数でプログラムの引数を取得できる。
    // そのうち2番目を`nth`で取得（0番目はプログラムの名前、1番目はパターンで今は無視）。
    // 引数があるか分からないのでOptionで返される。
    let filename = match env::args().nth(2) {
        // あれば取り出す。
        Some(filename) => filename,
        // なければヘルプを表示して終了
        None => {
            usage();
            return;
        }
    };
    // `File`構造体の`open`関連関数でファイルを開ける。
    // 失敗する可能性があるので結果は`Result`で返される。
    // 下の方でもう一度`filename`を使うためにここでは`&filename`と参照で渡していることに注意。
    let file = match File::open(&filename) {
        // 成功すれば取り出す。
        Ok(file) => file,
        // ファイルが見つからないなどのエラーの場合はそのままプログラム終了
        Err(e) => {
            println!("An error occurred while opening file {}:{}", filename, e);
            return;
        }
    };
    // Fileをそのまま使うと遅いのと`lines`メソッドを使うために`BufReader`に包む。
    // この`new`もただの関連関数。
    let input = BufReader::new(file);
    // `BufReader`が実装するトレイトの`BufRead`にある`lines`メソッドを呼び出す。
    // 返り値はイテレータなので`for`式で繰り返しができる
    for line in input.lines() {
        // 入力がUTF-8ではないなどの理由で行のパースに失敗することがあるので
        // `line`もResultに包まれている。
        let line = match line {
            Ok(line) => line,
            // 失敗したらそのまま終了することにする。
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;

            }
        };
        // パターンにマッチしたらプリントする
        // is_matchはリードオンリーなので参照型を受け取る
        if reg.is_match(&line) {
            // 上で参照型で引数に渡したので、ここでも使える
            println!("{}", line);
        }
    }
}
