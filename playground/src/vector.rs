#[derive(Debug)]
struct LVar {
	name: String,
	offset: usize,
}

pub fn run() {
	let mut vec = Vec::new();

	vec.push(LVar {
		name: "foo".to_string(),
		offset: 0,
	});

	vec.push(LVar {
		name: "bar".to_string(),
		offset: 8,
	});

	println!("vec:{:#?}", vec);

	let s = "foo".to_string();
	let filtered = vec.iter().find(|lvar| lvar.name == s);
	println!("vec:{:#?}", filtered);

	let name = "taro";

	let s = format!(
		"
  im {}\n
  age {}\n
	",
		name, 22
	);
	println!("s:{}", s);
}
