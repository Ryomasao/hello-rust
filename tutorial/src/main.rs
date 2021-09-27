fn main() {
    func();
    condition();
    func_loop();
    func_owner();
}

// 関数の戻り値がある場合は、型指定を行う
fn func() -> i32 {
    // ;がなければ、式として扱われる。
    // ブロックの最後が式だと、それがreturnされる
    // xの値は2になる
    let x = {
        let x = { 1 + 1 };
        x
    };
    println!("{}", x);
    // これもxをreturnしてる
    x
}

fn condition() {
    let val = 0;
    // conditionはbooleanじゃなきゃだめ
    if val > 0 {
        println!("more than zero.");
    } else {
        println!("0 or less than zero.");
    }

    // 三項演算子っぽく値もセットできる
    // 型は一致させなきゃだめ
    let val2 = if val > 0 { 1 } else { 2 };
    println!("{}", val2);
}

fn func_loop() {
    let array = ['a', 'b'];
    for item in array.iter() {
        println!("{}", item);
    }

    for item in (1..4).rev() {
        println!("{}", item);
    }
}

fn func_owner() {
    let s1 = String::from("hello world");
    let mut s2 = func_move(s1);
    func_view(&mut s2);
    let index = first_word(&s2);
    //{
    //    let r1 = &mut s1;
    //    r1.push_str("add by r1");
    //}

    //let r2 = &mut s1;
    //r2.push_str("add by r2");

    println!("s={}", s2);
    println!("index={}", index);
}

fn func_move(s: String) -> String {
    // 関数に渡した値にも所有権は移る。
    // 参照と借用から
    // https://doc.rust-jp.rs/book-ja/ch04-02-references-and-borrowing.html
    s
}

fn func_view(s: &mut String) -> &mut String {
    s.push_str("a");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
