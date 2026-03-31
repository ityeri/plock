use clock::PlockClock;

fn main() {
    let mut clock = PlockClock::default();

    loop {
        println!("This message displays 2 times for a second!");
        clock = clock.tick(2f64);
    }
}
