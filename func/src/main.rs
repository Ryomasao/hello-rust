use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<F, K, V> {
    calculation: F,
    values: HashMap<K, V>,
}

impl<F, K, V> Cacher<F, K, V>
where
    F: Fn(K) -> V,
    K: Eq + Hash + Clone,
    V: Clone + Copy,
{
    fn new(calculation: F) -> Cacher<F, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    // VではなくVの参照をかえすようにしたパターン、これはコンパイルが通らない
    // おなじような質問。ただ回答の内容が理解できぬ。。。
    // https://stackoverflow.com/questions/56181266/mutable-borrow-from-hashmap-and-lifetime-elision
    //fn failed_value(&mut self, key: K) -> &V {
    //    match self.values.get(&key) {
    //        Some(v) => v,
    //        None => {
    //            let s = String::from("aaa");
    //            let v = (self.calculation)(key.clone());
    //            // ここに表示されるエラーMSGがなぞ
    //            // &Vではなく &Stringとして考えると、エラーMSGの内容とは異なるけどなんだか危険な匂いがすることはわかる
    //            // というのも、そのStringの所有者はここでいうクロージャになると思うんだけど、クロージャ終わった瞬間解放されちゃう気がする。
    //            // HashMapのkeyに参照持たせるのってどうなんだろう。
    //            self.values.insert(key, v.clone());
    //            &v
    //        }
    //    }
    //}

    fn value(&mut self, key: K) -> V {
        match self.values.get(&key) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(key.clone());
                self.values.insert(key, v.clone());
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("today do {} pushups!", {
            expensive_result.value(intensity)
        });

        println!("next do {} situps!", { expensive_result.value(intensity) });
    } else {
        if random_number == 3 {
            println!("take a bread today!");
        } else {
            println!("today run for {} minutes!!", {
                expensive_result.value(intensity)
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cacher;
    #[test]
    fn call_with_diff_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
