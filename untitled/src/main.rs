/*use std::time::{Instant};

fn main() {
    // Get the current time as an Instant
    let now = Instant::now();

    println!("time: {:?}",now);
    // Convert the Instant to a duration since the epoch (time since system startup)
    let elapsed_duration = now.elapsed();

    // Extract seconds and nanoseconds from the duration
    let seconds = elapsed_duration.as_secs();
    let nanoseconds = elapsed_duration.subsec_nanos();

    println!("Current system time (seconds): {}", seconds);
    println!("Current system time (nanoseconds): {}", nanoseconds);
}*/


/*use std::time::{Instant, Duration};
use chrono::{DateTime, Local};

fn main() {
    // Get the current time as an Instant
    let now = Instant::now();

    // Convert the Instant to a duration since the epoch (time since system startup)
    /*et elapsed_duration = now.elapsed();

    // Extract seconds and nanoseconds from the duration
    let seconds = elapsed_duration.as_secs();
    let nanoseconds = elapsed_duration.subsec_nanos();*/

    // Get the current system time with date and time information using chrono
    let local_time = Local::now();
    println!("Current Date: {}", local_time);

    // Extract date components (year, month, day)
    /*let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();

    // Calculate hours and minutes using elapsed duration and modulo operations
    let hours = (elapsed_duration.as_secs() / 3600) % 24;
    let minutes = (elapsed_duration.as_secs() / 60) % 60;

    // Print the formatted time information
    println!("Current Date: {}-{:02}-{:02}", year, month, day);
    println!("Current Time (hours:minutes:seconds.nanos): {:02}:{:02}:{:02}.{:09}", hours, minutes, seconds, nanoseconds);*/
}*/


use std::time::{Instant, Duration};
use chrono::{DateTime, Local};
use std::thread::sleep;

fn main() {
    let mut previous_time = Instant::now();

    loop {
        // Get the current time
        let now = Instant::now();

        // Check if at least one second has passed since the previous print
        let elapsed = now.duration_since(previous_time);
        if elapsed >= Duration::from_secs(1) {
            // Get the current date and time using chrono
            let local_time = Local::now();

            // Print the formatted time information
            println!("Current Date: {}", local_time);

            // Update the previous time for the next iteration
            previous_time = now;
        }

        // Sleep for a short duration to avoid busy-waiting
        sleep(Duration::from_millis(10)); // Adjust this value as needed
    }
}


//Click network.