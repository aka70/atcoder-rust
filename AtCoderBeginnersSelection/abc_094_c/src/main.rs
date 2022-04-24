use proconio::input;
fn main() {
    input! {
        n:i32,
        plans:[(i32,i32,i32);n]
    }

    let mut deer = (0, 0, 0);
    let mut ans = "Yes";

    for (t, x, y) in plans {
        let d = (x - deer.1).abs() + (y - deer.2).abs();
        if d <= (t - deer.0) && ((t - deer.0) - d) % 2 == 0 {
            deer.0 = t;
            deer.1 = x;
            deer.2 = y;
        } else {
            ans = "No";
            break;
        }
    }

    println!("{}", ans)
}
