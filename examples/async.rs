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
