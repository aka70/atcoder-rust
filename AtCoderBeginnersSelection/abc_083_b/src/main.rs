fn main() {
    proconio::input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let result = (1..=n).filter(|x| {
        let total = x.to_string().chars()
            .map(|i| i.to_digit(10).unwrap() as i32).sum::<i32>();
        a <= total && total <= b
    }).sum::<i32>();

    println!("{}", result);
}
