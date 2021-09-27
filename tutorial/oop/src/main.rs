extern crate oop;

use oop::Post;

fn main() {
	let mut post = Post::new();

	let t = String::from("hey");
	post.add_text(&t);
	println!("con:{}", post.content());
}
