## 今日プロ雛形

[dependencies]
proconio = "0.4.3"

```rust
use proconio;

fn main() {
    proconio::input! {
        n: u32,
    }

    println!("{}", result);
}
```

## 整数の各桁の総和を求める

```rust
use proconio;

fn main() {
    proconio::input! {
        x: u32,
    }

    // x = 1234 とすると、total = 10となる。
    let total = x.to_string().chars()
          .map(|i| i.to_digit(10).unwrap()).sum::<u32>();

    println!("{}", total);
}

```
