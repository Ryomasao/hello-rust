//struct User {
//    username: String,
//    email: String,
//    sing_in_count: u64,
//    active: bool,
//}
//
//struct Color(i32, i32, i32);
//struct Point(i32, i32, i32);

fn main() {
    //let user1 = User {
    //    username: String::from("souser"),
    //    email: String::from("a@a.com"),
    //    sing_in_count: 1,
    //    active: true,
    //};

    //let name = String::from("a@a.com");
    //let user1 = build_user(name);
    //let user2 = User {
    //    active: false,
    //    ..user1
    //};
    //println!("Hello, {}", user1.username);

    let rectangle = Rectangle {
        width: 10,
        height: 10,
    };
    println!("{:?}", rectangle);
    println!("{}", area(&rectangle));
    // 所有権が移動するマン数
    //area2(rectangle);
    println!("{}", rectangle.area());

    let rectangle2 = Rectangle {
        width: 10,
        height: 12,
    };

    let can = rectangle.can_hold(&rectangle2);
    println!("{}", can);

    let rectangle3 = Rectangle::square(32);
    println!("{:?}", rectangle3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, compare: &Rectangle) -> bool {
        self.width >= compare.width && self.height >= compare.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area2(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//fn build_color(base: i32) -> Color {
//    Color(base, base, base)
//}
//
//fn build_user(username: String) -> User {
//    User {
//        username,
//        email: String::from("a@a.com"),
//        sing_in_count: 1,
//        active: true,
//    }
//}
