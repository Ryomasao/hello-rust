use std::num::ParseIntError;

pub fn run() {
	let numbers = vec!["42", "10"];
	let empty = vec![];
	let strings = vec!["tohu"];

	println!("numbers:{}", double_first(numbers));
	println!("empty:{}", double_first(empty));
	println!("strings:{}", double_first(strings));
}

fn multiply(v1: &str, v2: &str) -> Result<i32, ParseIntError> {
	let v1 = v1.parse::<i32>()?;
	let v2 = v2.parse::<i32>()?;
	Ok(v1 * v2)
}

// Some<T>→Some<U>とかに変形するときに利用するのかな？
fn process(food: Option<&str>) -> Option<i32> {
	food.map(|_| 1)
}

// flatの意味があんまりわかってない
fn process2(food: Option<&str>) -> Option<i32> {
	food.and_then(|_| Some(1))
}

fn print1(result: Result<i32, ParseIntError>) {
	// resultにもand_then/mapがある
	result.and_then(|num| println!("{}", num));
}

fn print2(result: Result<i32, ParseIntError>) -> Result<i32, ParseIntError> {
	// resultにもand_then/mapがある
	result.map(|num| num)
}

fn double_first(vec: Vec<&str>) -> i32 {
	// vecは基本Option返すんだね
	let first = vec.first().unwrap();
	// parseとかエラーが起こり得るのものはResult型で返すんだ
	2 * first.parse::<i32>().unwrap()
}
