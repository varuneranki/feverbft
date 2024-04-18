// Define a module for time-related functions
mod timey {
    use std::time::{Instant, Duration};

    // Get the current system time as a tuple of (seconds, nanoseconds)
    pub fn get_current_time_ns() -> (u64, u32) {
        let now = Instant::now();
        let elapsed_duration = now.elapsed();

        (elapsed_duration.as_secs(), elapsed_duration.subsec_nanos())
    }
}





































//get time from chrony using network  todo!();
//perform time divisions 10^3 if needed
//precise time sample for rust https://stackoverflow.com/a/55619763

//use std::time::SystemTime;

//use std::time::Instant;
//https://users.rust-lang.org/t/high-accuracy-timer/29019/11

/*use std::time::{Instant, Duration};

fn main() {
    // Get the current time as an Instant
    let now = Instant::now();

    // Convert the Instant to a duration since the epoch (time since system startup)
    let elapsed_duration = now.elapsed();

    // Extract seconds and nanoseconds from the duration
    let seconds = elapsed_duration.as_secs();
    let nanoseconds = elapsed_duration.subsec_nanos();

    println!("Current system time (seconds): {}", seconds);
    println!("Current system time (nanoseconds): {}", nanoseconds);
}*/

