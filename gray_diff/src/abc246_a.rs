use proconio;

fn main() {
    proconio::input! {
        a: i32, b: i32,
        c: i32, d: i32,
        e: i32, f: i32,
    }

    let x = if a == c {
        e
    } else if a == e {
        c
    } else {
        a
    };
    let y = if b == d {
        f
    } else if b == f {
        d
    } else {
        b
    };
    println! {"{} {}", x, y};
}
