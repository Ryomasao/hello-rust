// ハッシュの更新は必要になったら見に来る。

pub mod example {
	use std::collections::HashMap;

	pub fn handle() {
		// 基本
		let mut scores = HashMap::new();

		let blue_name = String::from("Blue");
		scores.insert(blue_name, 10);
		scores.insert(String::from("Yellow"), 50);

		// ベクタから作る
		let teams = vec![String::from("Blue"), String::from("Yellow")];
		let initial_scores = vec![10, 50];

		let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

		let key = String::from("Blue");
		// optionを返す
		// 走査
		for (key, value) in &scores {
			println!("{}:{}", key, value);
		}
	}
}
