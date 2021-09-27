# MEMO

- 文字列スライスは参照。所有じゃない。格納領域は静的領域。
  → 静的領域にあるので解放する必要がない。
- String は所有。格納領域はヒープ。

- Stackは最大8MB

- mutableな参照が有効な間は、所有権者ですら値を参照できない！

```rs
	let mut owner = String::from("hello");
	let r = &mut owner;
	println!("owner:{}", owner);
  // これがあると↑がNG
	println!("r:{}", r);
```