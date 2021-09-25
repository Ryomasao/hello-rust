# Hello Rust

## Basic

1. 新しくプロジェクトを作成する

```
> cargo new project_name
```

実行

```
> cd project_name
> cargo run
```

## 作業途中のメモ

### 整数について

> では、どの整数型を使うべきかはどう把握すればいいのでしょうか？もし確信が持てないのならば、 Rust の基準型は一般的にいい選択肢になります。整数型の基準は i32 型です: 64 ビットシステム上でも、 この型が普通最速になります。isize と usize を使う主な状況は、何らかのコレクションにアクセスすることです。

### 参照について

```rs
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s2;
    // s3は&&String
    // 問題なくlen:5が表示される
    println!("len:{}", s3.len());
```

参照はもちろん、参照の参照対してもメソッド呼び出しできる。
Rust が自動で参照を解決してくれる。

https://doc.rust-jp.rs/book-ja/ch05-03-method-syntax.html

> C と C++では、メソッド呼び出しには 2 種類の異なる演算子が使用されます: オブジェクトに対して直接メソッドを呼び出すのなら、.を使用するし、オブジェクトのポインタに対してメソッドを呼び出し、 先にポインタを参照外しする必要があるなら、->を使用するわけです。 言い換えると、object がポインタなら、object->something()は、(\*object).something()と同等なのです。

> Rust には->演算子の代わりとなるようなものはありません; その代わり、Rust には、 自動参照および参照外しという機能があります。Rust においてメソッド呼び出しは、 この動作が行われる数少ない箇所なのです。

```rs
    let foo = "foo";
    let bar = foo;
    // これは問題なかった
    // barに所有権がいくからfooは死んでるかと思った
    let baz = foo;
```

### String について

```rs
		// こっちの形は組み込みString
    let name = String::from("tarou");
		// こっちは &str バイナリへの特定の位置を指すスライス
    let name = "tarou";
    // &String::from("foo")[..] = "foo"
    // &String::from("foo") = "foo" なのかがよくわからない。関数の引数が&strの場合、どっちもいける。
    // 参照外し型強制キーワードをあとでみる
```

これはライフタイム指定子が必要になる。あとで。

```rs
struct User {
    username: &str,
}
let user1 = User {username: "tarou"}
```

### 所有権について

> 関数の引数に参照を取ることを借用と呼びます。

構造体をクローンすればいいのかな？

```rs
    let user1 = build_user(name);
    let user2 = User {
        active: false,
				// user1の所有権がここでmoveしてしまう
        ..user1
    };
		// user1が参照できない
    println!("Hello, {}", user1.username);

```

スカラー型については、move した後も参照できる？

https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html

> その理由は、整数のようなコンパイル時に既知のサイズを持つ型は、スタック上にすっぽり保持されるので、 実際の値をコピーするのも高速だからです。これは、変数 y を生成した後にも x を無効化したくなる理由がないことを意味します。 換言すると、ここでは、shallow copy と deep copy の違いがないことになり、 clone メソッドを呼び出しても、一般的な shallow copy 以上のことをしなくなり、 そのまま放置しておけるということです。

これは問題なかった。なるほど。

```rs
    let x = 5;
    let y = x;

    let z = x;

    println!("x:{} y:{} z:{}", x, y, z);
```

関数に所有権を渡すべきか、渡さないべきかが問題だ。

- println!って所有権奪わないんだ
