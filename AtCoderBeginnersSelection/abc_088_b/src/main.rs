use proconio;

fn main() {
    proconio::input! {
        n: i32,
        mut a: [i32; n]
    }

    a.sort();
    a.reverse();

    let result = a.iter().enumerate().map(|(i, &x)| if i % 2 == 0 { x } else {x * -1}).sum::<i32>();

    println!("{}", result);
}
