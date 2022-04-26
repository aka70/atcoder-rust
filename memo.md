参考 : https://paruma184.hatenablog.com/entry/2021/09/22/210945

## 今日プロ雛形

[dependencies]
proconio = "0.4.3"
regex = "1.5.5"

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

## 正規表現

```rust
use regex::Regex;

fn main() {
    let s = 'dreamer'
    let ans = if Regex::new(r"^(dream|dreamer|erase|eraser)*$")
        .unwrap()
        .is_match(&s)
    {
        "YES"
    } else {
        "NO"
    };

    println!("{}", ans);
}
```

## 三角関数
```rust
    proconio::input! {
        x : f64,
        y : f64
    }
    let deg = y.atan2(x);

    let ans_x = deg.cos();
    let ans_y = deg.sin();

    
    println! {"{} {}", ans_x, ans_y};
    ```
