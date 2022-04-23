use proconio;

fn main() {
    proconio::input! {
        n: i32,
        mut d: [i32; n]
    }

    d.sort();
    d.dedup();

    println!("{}", d.len());
}
