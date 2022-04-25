use proconio;

fn main() {
    proconio::input! {
      n: i32,
    }

    let result = solve(n);
    println!("{}", result);
}

fn solve(num: i32) -> String {
    if num == 1 {
        return "1".to_string();
    }
    let side = solve(num - 1);
    return format!("{} {} {}", side, num, side);
}
