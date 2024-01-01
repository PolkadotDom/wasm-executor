#![allow(unused_variables)]

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fs::File;
use std::io::prelude::*;

/// Takes in a simple primitive signed 32-bit integer
/// and adds one to it, returning the result.
///
/// It is ready for wasm execution thanks to the `#[no_mangle]` attribute.
/// You may find that you also need `extern "c"`.
#[no_mangle]
pub extern "C" fn add_one(n: i32) -> i32 {
	n+1
}

/// Prints the number 42 to the terminal.
/// It does not do any math, the number is always 42.
///
/// Reminder, you may need to add some annotations to this function's declaration
/// to make it work with wasm.
#[no_mangle]
pub extern "C" fn print_forty_two() {
	println!("{}", 42);
}

/// Calculates the division between two integers number
#[no_mangle]
pub extern "C" fn div(a: i32, b: i32) -> i32 {
	a/b
}

/// Fetches the current system time and determines whether it is after the year 2000
/// Returns true iff the current time is after January 1st 2000
/// you may find https://doc.rust-lang.org/std/time/struct.SystemTime.html useful
#[no_mangle]
pub extern "C" fn wen_millennium() -> bool {
	// Define the start of the millennium as January 1, 2000.
    // This is 946684800 seconds after the UNIX epoch (January 1, 1970)
    let millennium_start = Duration::from_secs(946684800);

    // Fetch the current time
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(now) => now > millennium_start,
        Err(_) => false, // This happens if the current time is before the UNIX epoch
    }
}

/// Calculates the sum of two floating point numbers
#[no_mangle]
pub extern "C" fn sum_floats(a: f32, b: f32) -> f32 {
	a+b
}

/// Takes in a simple primitive signed 32-bit integer
/// and adds one to it, and writes the result to a file.
#[no_mangle]
pub extern "C" fn write_to_file(a: i32) {
	let b = a + 1;

    let mut file = match File::create("output.txt") {
        Ok(f) => f,
        Err(e) => {
			println!("{}", e);
            // Handle error (e.g., log or return early)
            return;
        },
    };

    if let Err(e) = writeln!(file, "{}", b) {
        // Handle error
		println!("{}", e);
    }
}
