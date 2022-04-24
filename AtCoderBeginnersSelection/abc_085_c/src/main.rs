use proconio;

fn main() {
    proconio::input! {
        n: u32,
        y: u32,
    }

    let mut result:(i32, i32, i32) = (-1, -1 ,-1);

    for i in 0..=n {
        for j in 0..= n-i {
            if 10000 * i + 5000 * j + 1000 * (n - i - j) == y {
                result = (i as i32, j as i32, (n - i - j) as i32);
            }
        }
    };

    println!("{} {} {}", result.0, result.1, result.2);
}
