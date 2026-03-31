# plock

Im using the pygame library.
In pygame lib, there is a usable little class `pygame.time.Clock` for maintaining a certain fps

`plock::PlockClock` is the same as `pygame.time.Clock`.

# using plock

```toml
plock = { git = "https://github.com/ityeri/plock" }
```

# examples

* sync
```rust
use plock::PlockClock;

fn main() {
    let mut clock = PlockClock::default();

    loop {
        println!("This message displays 2 times for a second!");
        clock = clock.tick(2f64);
    }
}
```

* async
```rust
use plock::PlockClock;

#[tokio::main]
async fn main() {
    let mut clock = PlockClock::default();

    loop {
        println!("This message displays 2 times for a second!");
        clock = clock.atick(2f64).await;
    }
}
```
