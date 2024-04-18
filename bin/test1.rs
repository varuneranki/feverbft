fn main() {
    let (seconds, nanoseconds) = timey::get_current_time_ns();

    println!("Current system time (seconds): {}", seconds);
    println!("Current system time (nanoseconds): {}", nanoseconds);
}