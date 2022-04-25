use proconio;
use std::collections::HashMap;

fn main() {
    proconio::input! {
      n: i32,
      names: [(String, String); n],
    }

    let mut map: HashMap<&String, usize> = HashMap::new();
    for (s, t) in names.iter() {
        *map.entry(s).or_insert(0) += 1;

        *map.entry(t).or_insert(0) += 1;
    }

    for (s, t) in names.iter() {
        if map[s] == 1 || map[t] == 1 {
            continue;
        } else if map[s] == 2 && s == t {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
