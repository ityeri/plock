use plock::PlockClock;

#[tokio::main]
async fn main() {
    let mut clock = PlockClock::default();

    clock = clock.initialized();

    loop {
        println!("This message displays 2 times for a second!");
        clock = clock.atick(2f64).await;
        println!("Time delta is: {}", clock.last_dt);
    }
}
