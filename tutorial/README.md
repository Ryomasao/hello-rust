# Hello Rust

チュートリアルから派生した疑問を解消するのが難しい。
ある程度学習したら目的ベースでつまんでくほうがよさげ。

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

iterator の iter と into_iter

```rs
fn shoes_im_my_size_u32<'a>(shoes: &'a Vec<u32>, shoe_size: u32) -> Vec<&'a u32> {
    // こんな書き方であってるのかしら
    // iter()の場合所有権を奪わない
    // これを使う場面はもとのベクタを生かしたまま、その要素を参照するベクタを新しくつくるときかしら
    // into_iterでもsは参照を返す。shoesが参照だからかしら。うーむ。
    shoes.iter().filter(|s| s == &&shoe_size).collection()
}
```

### String について

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

### Q and A

#### Q1

```rs
    let foo = "foo";
    let bar = foo;
    // これは問題なかった
    // barに所有権がいくからfooは死んでるかと思った
    let baz = bar;
```

#### A1

文字列`foo`は文字列スライスで、ヒープではなく静的領域に格納される。
なので、プリミティブな値と同じ挙動って考えて大丈夫そう。

```rs
    let foo = "foo";
    // 文字列fooへのポインタを保持する変数
    let bar = foo;
    // 文字列fooへのポインタを保持する変数
    let baz = bar;
```

&str は Copy トレイトを実装しているからっていう説明もできる。
正確には、&str ではなく、&が Copy トレイトを実装している。

https://doc.rust-lang.org/std/primitive.reference.html#trait-implementations-1

そして我々は、`str`の実装はわからなくって、`&str`としてのみ触れることができる。

https://stackoverflow.com/questions/59994765/how-can-str-be-copied-if-it-doesnt-implement-the-copy-trait

#### Q2

`&String::from("foo") = "foo"` なのかがよくわからない。
関数の引数が&str の場合、どっちもいける。

#### A2

違う。

> 関数の引数が&str の場合、どっちもいける。

これは、Defer の機能。あとでみる。

https://qiita.com/nirasan/items/e9c621240a7aae914cb8#deref-%E3%81%AB%E3%82%88%E3%82%8B%E8%87%AA%E5%8B%95%E5%9E%8B%E5%A4%89%E6%8F%9B

`String::from("foo")`  
→ ヒープ領域に格納されている文字列 foo へのアドレスを持つ

`"foo"`  
→ 静的領域に格納されている文字列 foo へのアドレスを持つ

#### Q3

参照パターンでも、所有権が移動するのはなんで。

```rs
	let mut foo = 1;
    let bar = &mut foo;
    let baz = bar;
    // barはmoveされちゃっててNG
	println!("bar:{}", bar)
	println!("baz:{}", baz);
```

### A3

`&`は Copy トレイトを実装してるけど、`&T`は Copy トレイトを実装してないんだって。

https://doc.rust-lang.org/std/primitive.reference.html#trait-implementations-1

> &mut T references get all of the above except Copy and Clone (to prevent creating multiple simultaneous mutable borrows), plus the following, regardless of the type of its referent:

move しないと、`mut`の参照が複数できちゃうから納得。

### Q4

`s`の実体はポインタなんだけど、前者が OK で後者が NG な理由が理解しにくい。

```rs
fn func_ok()->String {
	let s = String::from("hello");
	s
}

fn func_ng<'a>()->&'a String {
	let s = String::from("hello");
	&s
}
```

### A5

`s`の実体はポインタって思うからだめなのかな。  
`s`の実体は String 型。ポインタじゃない。String 型の内部実装として、文字列へのアドレスはもってるけど、ポインタそのものじゃない的な？  
もしくは、`&s`は所有権を move しないから、後者で`&s`返しても s の所有者がいなくなっちゃうって考える？
