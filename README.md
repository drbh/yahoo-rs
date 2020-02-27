# yahoo-rs

A clean Rust wrapper around Yahoo Finance's API


###  Example
```rust
use yahoo_rs::*;

fn main() {
    let data = YahooFinance::new("TVIX")
        .interval("1m")
        .set_period1(1580406600)
        .set_period2(1581011340)
        .fetch();

    println!("[X] Fetch Complete");
    println!("{:#?}", data);
}
```

### Params

- interval
- period 1
- period 2
- events
- extended market
