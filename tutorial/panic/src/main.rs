use std::fs::File;
use std::io::Read;
use std::io::{self, ErrorKind};

fn main() {
    // 基本
    //let f = File::open("hello.txt");
    //let f = match f {
    //    Ok(file) => file,
    //    // この構文はあとでくわしく
    //    // ざっくり書くと、マッチ条件をさらに細かくかける
    //    // refはif condition内で所有権をムーブしないようにするための構文だって
    //    Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //        Ok(fc) => fc,
    //        Err(e) => {
    //            panic!("ファイルを作成しようとして失敗。 file:{:?}", e);
    //        }
    //    },
    //    Err(error) => {
    //        panic!("ファイルのオープンに失敗しました。 file:{:?}", error);
    //    }
    //};

    // matchを割愛してエラーをthrowする
    //let f = File::open("hello.txt").unwrap();
    // matchを割愛してメッセージ付きでエラーをthrowする
    //let f = File::open("hello.txt").expect("ファイルのオープンに失敗");

    // エラーを移譲する
    // 移譲のショートハンドは実践で
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        // read時に読み取り位置を動かすから
        // ミュータブルにする必要があるのかな
        let mut f = match f {
            Ok(file) => file,
            // あー、このreturn は関数の返り値を指してる。fにセットする値じゃないよ
            // matchアームの返却型は全部いっしょの必要があるので、OkでFile型を返却してるから、Err型は返せない
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        // 一方、こっちのmatchは、matchアームそのものを返却してると考えればよさげ。
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    let s = match read_username_from_file() {
        Ok(s) => s,
        Err(e) => {
            panic!("panic! {:?}", e);
        }
    };

    println!("string: {}", s);
}
