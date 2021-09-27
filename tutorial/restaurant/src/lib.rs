// モジュールは、暗黙的に crate配下につくられる。
// こんなかんじ
// crate/
//   outside
//   front_of_hose/
//      hosting/
//          add_to_waitlist

// すんなり理解できたので、割愛した概念。必要なときにもかいみる。
// as エイリアス
// reexport
// named import

fn outside() {}

mod back_of_house {
    // 構造体全体をpubにしてもフィールドはprivateのまま
    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peach"),
            }
        }
    }

    // 一方enumはenum全体にpubを設定すれば列挙したものは公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// mod定義の別ファイル参照はuseで利用かとおもいきや？
// > の後にブロックではなくセミコロンを使うと、Rustにモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令します
mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //crate::front_of_house::hosting::add_to_waitlist();
    // PHPのuseといっしょ
    hosting::add_to_waitlist();

    let mut meal = back_of_house::BreakFast::summer("Syokupan");
    // private fieldは参照できない
    // meal.seasonal_fruit

    meal.toast = String::from("Anpan");
}
