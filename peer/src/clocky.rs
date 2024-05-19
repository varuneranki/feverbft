use std::net::UdpSocket;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sntpc;
use chrono::{NaiveDateTime, Timelike};

// Global variable to hold the logical clock value
static mut LOGICAL_CLOCK: u64 = 0;

pub fn synchronize_logical_clock() {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("Unable to create UDP socket");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("Unable to set UDP socket read timeout");
        
    match sntpc::simple_get_time("time.google.com:123", socket.try_clone().unwrap()) {
                    Ok(time) => {
                        unsafe {
                            // Set logical clock to NTP time
                            LOGICAL_CLOCK = time.sec() as u64 + 1; // Add 1 second offset
                            println!("Logical Clock synchronized with NTP: {}", LOGICAL_CLOCK);
                        }
                    }
                    Err(err) => println!("Failed to synchronize with NTP: {:?}", err),
                }
}
    /*loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed = input.trim();

        match trimmed {
            "klock" => {
                // Synchronize logical clock with NTP server time
                match sntpc::simple_get_time("time.google.com:123", socket.try_clone().unwrap()) {
                    Ok(time) => {
                        unsafe {
                            // Set logical clock to NTP time
                            LOGICAL_CLOCK = time.sec() as u64 + 1; // Add 1 second offset
                            println!("Logical Clock synchronized with NTP: {}", LOGICAL_CLOCK);
                        }
                    }
                    Err(err) => println!("Failed to synchronize with NTP: {:?}", err),
                }
            }
            _ => {
                // Convert logical clock to human-readable time and print
                unsafe {
                    let epoch = UNIX_EPOCH + Duration::from_secs(LOGICAL_CLOCK);
                    let human_readable_time = NaiveDateTime::from_timestamp(epoch.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, 0);
                    println!("Current Logical Clock: {}", human_readable_time.time());
                }
            }
        }
    }
}*/

pub fn current_logical_clock_time() -> u64 {
    unsafe {
        LOGICAL_CLOCK
    }
}


