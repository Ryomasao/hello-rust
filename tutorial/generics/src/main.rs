pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

fn main() {
    let article = NewArticle {
        headline: String::from("headline"),
        author: String::from("name"),
        content: String::from("contents"),
    };

    let tweet = Tweet {
        username: String::from("name"),
        content: String::from("contents"),
    };

    println!("article:{}", article.summarize());
    println!("tweet:{}", tweet.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let largest = largest(&number_list);
    println!("max:{}", largest);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// スライスの理解がやっぱりあまい
// スライス= vector/配列への参照
//fn largest(list: &Vec<i32>) -> i32 {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number
        }
    }

    return largest;
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
