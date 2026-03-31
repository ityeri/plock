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

    clock = clock.initialized();

    loop {
        println!("This message displays 2 times for a second!");
        println!("Time delta is: {}", clock.last_dt);
        println!();
        clock = clock.tick(2.0);
    }
}
```

* async
```rust
use plock::PlockClock;

#[tokio::main]
async fn main() {
    let mut clock = PlockClock::default();

    clock = clock.initialized();

    loop {
        println!("This message displays 2 times for a second!");
        println!("Time delta is: {}", clock.last_dt);
        println!();
        clock = clock.atick(2.0).await;
    }
}
```
