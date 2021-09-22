pub mod example {
	pub fn handle() {
		// スライス復習
		// &String::from("foo")[..] = "foo"(スライス)
		// &String::from("foo") = "foo" なのかがよくわからない。関数の引数が&strの場合、どっちもいける。
		let a = "aaa bbb";
		println!("sliced:{}", super::slice(a));
		let a = &String::from("aaa bbb")[..];
		println!("sliced:{}", super::slice(a));
		let b = String::from("aaa bbb");
		// これがいけてしまう理由がよくわからなかった
		// 参照外し型強制キーワードをあとでみる
		super::slice(&b);

		// String基本
		let mut s = String::new();
		let b = String::from("bar");
		s.push('a');
		s.push_str(&b);

		println!("im {}", s);

		// &str→String
		let s1 = "foo".to_string();
		let s2 = "bar".to_string();

		// s1の所有権はs3に移り、s2はそのまま使える
		let s3 = s1 + &s2;

		let s1 = String::from("foo");
		let s2 = String::from("bar");
		let s3 = String::from("baz");

		// こっちがおすすめ
		// 引数の所有権も奪わない
		let s = format!("{}-{}-{}", s1, s2, s3);

		let moji_slice = "五里霧中";
		let moji_string = String::from("五里霧中");
		
		// これはできない
		//let m1 = moji_slice[0];
		//let m1 = moji_string[0];
		//let m1 = &moji_string[0];
		// 文字の境界値を明示的に取得する必要がある
		// UTF8で日本語は1文字3bytesだっけ？
		let m1 = &moji_string[0..3];
		let m1 = &moji_slice[0..3];

		println!("m1:{}", m1);

		// charsで走査できる
		for c in moji_slice.chars()
		{
			println!("{}", c);
		}

	}
}

fn slice(s: &str) -> &str {
	let bytes = s.as_bytes();

	// ここの理解ができてない
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}
