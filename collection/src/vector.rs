pub mod example {
	pub fn handle() {
		// 基本
		let mut v: Vec<i32> = Vec::new();
		v.push(1);
		v.push(2);
		v.push(3);

		// !vecマクロを利用できる
		let v = vec![1, 2, 3];
		//println!("vector:{}", v);

		// 参照
		let value1 = v[0];
		let value2 = v[1];
		// getはOptionを返すのか
		let value1 = v.get(0);

		let a = match value1 {
			None => false,
			// このiにピンときてない
			Some(i) => true,
		};

		// 走査
		let v = vec![1, 2, 3];
		for i in &v {
			println!("{}", i);
		}

		// 参照外し
		// さらっときたけど、よく間違えそう。
		let mut v = vec![1, 2, 3];
		for i in &mut v {
			*i += 50;
		}

		//println!("value1:{} value2:{}", value1, value2);
		println!("a:{}", a);

		// 構造体はどうなる？
		struct User {
			name: String,
		}

		let v = vec![
			User {
				name: String::from("tarou"),
			},
			User {
				name: String::from("jirou"),
			},
		];

		// 案の定、これだめだ。ベクター内の要素の所有権を奪うことはできないっぽい？
		// let u1 = v[0];
		// 借用すべき
		let u1 = &v[0];
		// こっちのほうがシンプルかな
		let u1 = v.get(0);

		// ベクタは同一の型しか入れないけど、Enumを使うことで複数の型を格納できる
		enum SpreadSheetCell {
			Int(i32),
			Float(f64),
			Text(String),
		}

		let row = vec![
			SpreadSheetCell::Int(32),
			SpreadSheetCell::Text(String::from("foo")),
		];
	}
}
