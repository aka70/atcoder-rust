use proconio;

fn main() {
  proconio::input! {
      s: String,
    }

    println!("0{}", &s[0..s.len() - 1]);
}
