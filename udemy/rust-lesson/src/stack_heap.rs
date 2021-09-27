pub fn run() {
	let mut owner = String::from("hello");
	let r = &mut owner;
	println!("owner:{}", owner);
	println!("r:{}", r);

}

fn func_ok()->String {
	let s = String::from("hello");
	s
}

fn func_ng<'a>()->&'a String {
	let s = String::from("hello");
	&s
}