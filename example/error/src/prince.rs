pub fn run() {
	let food = Some("food");
	let _snake = Some("snake");
	let none: Option<&str> = None;

	// unwrapはOption<T>からT型を取り出してくれる
	let s = food.unwrap();
	println!("s:{}", s);

	// Noneだった場合はpanic
	//none.unwrap();
	// expectを利用することでunwrap + Noneのときのエラpanic msgをカスタマイズできる
	// let _s = food.expect("エラー");
	// none.expect("エラー");

	let p = Person {
		job: Some(Job {
			phone_number: Some(PhoneNumber {
				area_code: Some(61),
				number: 1,
			}),
		}),
	};
}

fn _return_none_if_none(o: Option<i32>) -> Option<i32> {
	// ?はOptionをアンパックする。OptionがSome<T>の場合はTを、Noneの場合はNoneを返す
	// まったく意味の無いコード
	Some(o?)
}

// tsで考えると、こんなかんじ。undefined/nullを排除してるから、
// Optionalな項目はまさにOptionで扱うって考えるとスッキリした気がする！
// type Person = {job?:Job}
// type Job = {phone_number?:PhoneNumber}
// 割愛

struct Person {
	job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
	phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
	area_code: Option<u8>,
	number: u32,
}

impl Person {
	fn work_phone_area_code(&self) -> Option<u8> {
		// 普通にやるとめんどくさい & これだとpanicになる
		//let job = self.job.unwrap();
		//let phone_number = job.phone_number.unwrap();
		//phone_number.area_code

		// これなら panicじゃなくってNoneを返せる
		//let job = self.job?;
		//let phone_number = job.phone_number?;
		//phone_number.area_code

		// チェインすればシンプル
		self.job?.phone_number?.area_code
	}
}

//fn give_commoner(gift: Option<&str>) {
//	match gift {
//		Some("snake") => println!("wtf!"),
//		Some(_) => println!("ty!"),
//		None => println!("No gift."),
//	}
//}
