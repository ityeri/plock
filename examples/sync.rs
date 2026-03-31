use plock::PlockClock;

fn main() {
    let mut clock = PlockClock::default();

    clock = clock.initialized();

    loop {
        println!("This message displays 2 times for a second!");
        clock = clock.tick(2f64);
        println!("Time delta is: {}", clock.last_dt);
    }
}
