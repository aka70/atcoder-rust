use proconio;

fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        x: i32
    }

    let takahashi = solve(a, b, c, x);
    let aoki = solve(d, e, f, x);

    let result = if takahashi > aoki {
        "Takahashi"
    } else if takahashi == aoki {
        "Draw"
    } else {
        "Aoki"
    };

    println!("{}", result);
}

fn solve(a: i32, b: i32, c: i32, x: i32) -> i32 {
    let cycle = x / (a + c);
    let rest = x % (a + c);
    (cycle * a + std::cmp::min(rest, a)) * b
}
