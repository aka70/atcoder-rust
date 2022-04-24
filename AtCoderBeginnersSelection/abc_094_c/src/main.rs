use proconio;
use regex::Regex;

fn main() {
    proconio::input! {
        s: String,
    }

    let ans = if Regex::new(r"^(dream|dreamer|erase|eraser)*$")
        .unwrap()
        .is_match(&s)
    {
        "YES"
    } else {
        "NO"
    };

    println!("{}", ans);
}
