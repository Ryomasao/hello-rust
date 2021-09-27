pub struct Post {
	state: Option<Box<State>>,
	content: String,
}

impl Post {
	pub fn new() -> Post {
		Post {
			state: Some(Box::new(Draft {})),
			content: String::new(),
		}
	}
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}
	pub fn content(&self) -> &str {
		&self.content
	}
}

trait State {}

struct Draft {}

impl State for Draft {}
