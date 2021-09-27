//#[derive(Debug)]
//enum IpAddrKind {
//    V4,
//    V6,
//}

//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

fn main() {
    //let four = IpAddrKind::V4;
    //let six = IpAddrKind::V6;
    //println!("{:?}", four);

    // たしかにこれはやりそう
    //let ip1 = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1"),
    //};

    #[derive(Debug)]
    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    // これができるとのこと。利点がまだわからない。
    let ip1 = IpAddrKind::V4(String::from("127.0.0.2"));
    //V4("127.0.0.2")
    println!("{:?}", ip1);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // enumで型を定義することによって、この関数は柔軟にパラメータを受け取ることができる
    fn receive_message(message: Message) {}

    impl Message {
        fn call(&self) {
            // ここをどう扱うのかな。型がわからん。
            //println!("{}", &self);
            //println!("foo");
        }
    }

    let message1 = Message::Write(String::from("hey"));
    message1.call();

    /**
     * enumsのmatch
     */
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // Messageのcallで悩んでたやつかな。matchは型の絞り込みができるってtsっぽく考える。
            Coin::Quarter(state) => {
                println!("state quarter from {:?}!", state);
                25
            }
        }
    }

    let quarter = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(quarter));

    // 基本 Enumはeuqal式つかえないんだ
    //if quarter == Coin::Quarter {
    //
    //}

    // if let を使う
    let quarter = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = quarter {
        println!("state quarter from {:?}!", state);
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    // Optionの列挙をdisplayする方法はまだわからない
    //println!("plus one {}", plus_one(five));

    // SomeとNoneがいきなり出てきた。なかみはこんなかんじ。
    // match xと Some<T>がどんなときにマッチするかがしっくりくればさくさくつかえるようになるかも。
    //pub enum Option<T> {
    //    /// No value
    //    #[lang = "None"]
    //    #[stable(feature = "rust1", since = "1.0.0")]
    //    None,
    //    /// Some value `T`
    //    #[lang = "Some"]
    //    #[stable(feature = "rust1", since = "1.0.0")]
    //    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    //}
}
