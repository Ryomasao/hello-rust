#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

//fn shoes_im_my_size_u32_iter<'a>(shoes: &'a Vec<u32>, shoe_size: u32) -> Vec<&'a u32> {
//    shoes
//        .iter()
//        .filter(|s| s == &&shoe_size)
//        .collect::<Vec<&u32>>()
//}
//
//// moveするver
//fn shoes_im_my_size_u32_into_iter(shoes: Vec<u32>, shoe_size: u32) -> Vec<u32> {
//    shoes.into_iter().filter(|s| s == &shoe_size).collect()
//}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // https://doc.rust-lang.org/std/iter/#the-three-forms-of-iteration
    let iter = shoes.into_iter();
    iter.filter(|s| s.size == shoe_size).collect()
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: "sneaker".to_string(),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    // なにをもってeqなんだろう
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: "sneaker".to_string(),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    )
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let s = Counter::new();
    println!("{:?}", s);
    let s = s.skip(1);
    println!("{:?}", s);
    let s = s.zip(Counter::new());
    println!("{:?}", s);
    let s = s.map(|(a, b)| a * b);
    println!("{:?}", s);
    let s = s.filter(|x| x % 3 == 0);
    println!("{:?}", s);
    let sum: u32 = s.sum();
    // なんで18になるのかがわからん。たぶんzipの理解ができてない。
    println!("{:?}", sum);
}
