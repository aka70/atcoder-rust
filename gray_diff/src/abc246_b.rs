use proconio;

fn main() {
    proconio::input! {
        x : f64,
        y : f64
    }
    let deg = y.atan2(x);

    let ans_x = deg.cos();
    let ans_y = deg.sin();

    
    println! {"{} {}", ans_x, ans_y};
}
