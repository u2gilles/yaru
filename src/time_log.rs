//! # time_log
//! 
//! `time_log` is a simple, lightweight logging utility for Rust applications.
//! It provides a way to log messages with a timestamp relative to the first time 
//! the logging function was called or the last time it was initialized.
//!
//! ## Print vs Format
//!
//! Each logging function has two variants:
//! - **Print functions** (`t_print`, `nt_print`, `ti_print`, `nti_print`): Output directly to stdout
//! - **Format functions** (`t_format`, `nt_format`, `ti_format`, `nti_format`): Return formatted strings
//!
//! Similarly, each macro has two variants:
//! - **Print macros** (`t_print!`, `nt_print!`, `ti_print!`, `nti_print!`): Output directly to stdout
//! - **Format macros** (`t_format!`, `nt_format!`, `ti_format!`, `nti_format!`): Return formatted strings

use std::sync::Mutex;
use std::time::Instant;

/// Global reference point for the "Time Zero" of the application.
static START_TIME: Mutex<Option<Instant>> = Mutex::new(None);

// ============================================================================
// Helper Function (internal)
// ============================================================================

/// Internal helper to get or initialize the start time and calculate elapsed time.
// fn get_elapsed() -> (std::time::Duration, Option<std::sync::MutexGuard<'static, Option<Instant>>>) {
//     let mut lock = START_TIME.lock().unwrap();
//     let start = match *lock {
//         Some(s) => s,
//         None => {
//             let now = Instant::now();
//             *lock = Some(now);
//             now
//         }
//     };
//     let elapsed = start.elapsed();
//     drop(lock);
    
//     // Return elapsed; caller can access START_TIME if needed
//     let lock = START_TIME.lock().unwrap();
//     let start = lock.unwrap();
//     let elapsed = start.elapsed();
//     (elapsed, Some(lock))
// }

// ============================================================================
// Format Functions (return String)
// ============================================================================

/// Returns a formatted message with the elapsed time since the first call or last `init_time_log()`.
///
/// This function is thread-safe and returns a string in the format `[ss:mmm] message`.
/// - `ss`: Seconds elapsed since the reference point (padded to 2 digits).
/// - `mmm`: Milliseconds elapsed in the current second (padded to 3 digits).
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be formatted.
///
/// # Returns
/// A `String` containing the formatted timestamp and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// let formatted = t_format("Starting the application...");
/// println!("{}", formatted);
/// ```
pub fn t_format(msg: &str) -> String {
    let mut lock = START_TIME.lock().unwrap();
    let start = match *lock {
        Some(s) => s,
        None => {
            let now = Instant::now();
            *lock = Some(now);
            now
        }
    };
    
    let elapsed = start.elapsed();
    format!(
        "[{:02}:{:03}] {}", 
        elapsed.as_secs(), 
        elapsed.subsec_millis(), 
        msg
    )
}

/// Returns a formatted message with a leading newline and elapsed time.
///
/// This is particularly useful when you need formatted output with visual separation.
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be formatted.
///
/// # Returns
/// A `String` containing a newline followed by the formatted timestamp and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// let formatted = nt_format("--- PHASE 1: Initialization ---");
/// println!("{}", formatted);
/// ```
pub fn nt_format(msg: &str) -> String {
    format!("\n{}", t_format(msg))
}

/// Returns a formatted message with the current thread ID and elapsed time.
///
/// This variant is indispensable for debugging multi-threaded or asynchronous 
/// applications where tasks jump between threads.
///
/// The format used is `[id][ss:mmm] message`.
/// - `id`: The numeric ID of the current thread (extracted and formatted).
/// - `ss`: Seconds elapsed since the reference point.
/// - `mmm`: Milliseconds elapsed in the current second.
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be formatted.
///
/// # Returns
/// A `String` containing the formatted thread ID, timestamp, and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// std::thread::spawn(|| {
///     let formatted = ti_format("Message from a background thread!");
///     println!("{}", formatted);
/// }).join().unwrap();
/// ```
pub fn ti_format(msg: &str) -> String {
    let mut lock = START_TIME.lock().unwrap();
    let start = match *lock {
        Some(s) => s,
        None => {
            let now = Instant::now();
            *lock = Some(now);
            now
        }
    };
    
    let elapsed = start.elapsed();
    let id_str = format!("{:?}", std::thread::current().id());
    let id_num = id_str.replace("ThreadId(", "").replace(")", "");
    let id_num: u32 = id_num.parse().unwrap_or(0);

    format!(
        "[{:02}][{:02}:{:03}] {}", 
        id_num,
        elapsed.as_secs(), 
        elapsed.subsec_millis(), 
        msg
    )
}

/// Returns a formatted message with thread ID, elapsed time, and a leading newline.
///
/// This provides the ultimate level of detail (thread ID + timestamp) combined 
/// with visual separation.
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be formatted.
///
/// # Returns
/// A `String` containing a newline followed by the formatted thread ID, timestamp, and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// let formatted = nti_format("=== STARTING ASYNC EXECUTOR ===");
/// println!("{}", formatted);
/// ```
pub fn nti_format(msg: &str) -> String {
    format!("\n{}", ti_format(msg))
}

// ============================================================================
// Print Functions (call format functions, then println!)
// ============================================================================

/// Logs a message to the console with the elapsed time since the first call to `t_print` or the last `init_time_log()`.
///
/// This function is thread-safe and can be used to track the progression of your application 
/// over time. The output is consistently formatted to ensure that timestamps remain 
/// vertically aligned in your terminal, making it easy to scan the logs.
///
/// The time is displayed in the format `[ss:mmm]`.
/// - `ss`: Seconds elapsed since the reference point (padded to 2 digits).
/// - `mmm`: Milliseconds elapsed in the current second (padded to 3 digits).
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be logged.
///
/// # Examples
/// ```
/// use yaru::*;
/// t_print("Starting the application...");
/// t_print("Processing data batch #1...");
/// ```
pub fn t_print(msg: &str) {
    println!("{}", t_format(msg));
}

/// Logs a message with a leading newline for better visual separation between logical phases.
/// 
/// This is particularly useful when your terminal output is crowded and you want 
/// to draw attention to the start of a new process or test case. It internally 
/// calls `t_print` after printing the newline, ensuring consistent timestamping.
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be logged.
///
/// # Examples
/// ```
/// use yaru::*;
/// nt_print("--- PHASE 1: Initialization ---");
/// nt_print("All resources loaded successfully.");
/// ```
pub fn nt_print(msg: &str) {
    println!("{}", nt_format(msg));
}

/// Initializes or resets the reference time point ("Time Zero") to the current instant.
///
/// Calling this function is optional. If not called, the timer will automatically start 
/// on the first call to `t_print` or `nt_print`.
///
/// Use `init_time_log()` if you want the "Time Zero" to be exactly at the 
/// start of your `main` function or any other specific point.
///
/// # Examples
///
/// ```
/// use yaru::*;
/// init_time_log(); // Start the timer NOW
/// t_print("Timer started explicitly.");
/// ```
pub fn init_time_log() {
    let mut lock = START_TIME.lock().unwrap();
    *lock = Some(Instant::now());
}

/// Logs a message to the console with the current thread ID and elapsed time.
///
/// This variant is indispensable for debugging multi-threaded or asynchronous 
/// applications where tasks jump between threads. It allows you to track not just 
/// *when* something happened, but also *where* (on which executor thread).
///
/// The format used is `[id][ss:mmm] message`.
/// - `id`: The numeric ID of the current thread (extracted and formatted).
/// - `ss`: Seconds elapsed since the reference point.
/// - `mmm`: Milliseconds elapsed in the current second.
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be logged.
///
/// # Examples
/// ```
/// use yaru::*;
/// std::thread::spawn(|| {
///     ti_print("Message from a background thread!");
/// }).join().unwrap();
/// ```
pub fn ti_print(msg: &str) {
    println!("{}", ti_format(msg));
}

/// Logs a message with thread info and a leading newline for enhanced clarity.
///
/// This provides the ultimate level of detail (thread ID + timestamp) combined 
/// with visual separation. It's the recommended choice for logging the start 
/// of asynchronous tasks in a complex runtime like Tokio.
///
/// # Arguments
/// * `msg` - A string slice that holds the message to be logged.
///
/// # Examples
/// ```
/// use yaru::*;
/// nti_print("=== STARTING ASYNC EXECUTOR ===");
/// ```
pub fn nti_print(msg: &str) {
    println!("{}", nti_format(msg));
}

// =========================================================================
// MACROS
// =========================================================================

// ============================================================================
// Format Macros (return String)
// ============================================================================

/// Returns a formatted message with a relative timestamp as a `String`.
/// 
/// This macro provides a convenient way to format messages using the same 
/// syntax as `format!`. It calculates the time elapsed since the first call 
/// to any logging function or the last call to `init_time_log()`.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments, following the `format!` syntax.
///
/// # Returns
/// A `String` containing the formatted timestamp and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// let val = 42;
/// let output = t_format!("Processing value: {}", val);
/// println!("{}", output);
/// let simple = t_format!("Simple message");
/// ```
#[macro_export]
macro_rules! t_format {
    ($($arg:tt)*) => {
        $crate::time_log::t_format(&format!($($arg)*))
    };
}

/// Returns a formatted message with a leading newline and relative timestamp as a `String`.
/// 
/// Identical to `t_format!`, but prepends an empty line to the output. This is 
/// especially helpful for visually separating different phases of an application.
/// It uses the same formatting syntax as `format!`.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments.
///
/// # Returns
/// A `String` containing a newline, followed by the formatted timestamp and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// let phase = "Initialization";
/// let output = nt_format!("--- STARTING PHASE: {} ---", phase);
/// println!("{}", output);
/// ```
#[macro_export]
macro_rules! nt_format {
    ($($arg:tt)*) => {
        $crate::time_log::nt_format(&format!($($arg)*))
    };
}

/// Returns a formatted message with the current thread ID and relative timestamp as a `String`.
/// 
/// This macro is the formatted version of `ti_print!`. It includes the numeric ID 
/// of the thread executing the log, making it indispensable for debugging 
/// multi-threaded or asynchronous code where task switching occurs.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments.
///
/// # Returns
/// A `String` containing the formatted thread ID, timestamp, and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// let task_id = 5;
/// let output = ti_format!("Executing task #{}", task_id);
/// println!("{}", output);
/// ```
#[macro_export]
macro_rules! ti_format {
    ($($arg:tt)*) => {
        $crate::time_log::ti_format(&format!($($arg)*))
    };
}

/// Returns a formatted message with thread ID, relative timestamp, and a leading newline as a `String`.
/// 
/// This is the most comprehensive logging macro in the library. It combines 
/// visual separation (newline), thread identification, and relative timestamping 
/// with full formatting support.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments.
///
/// # Returns
/// A `String` containing a newline, followed by the formatted thread ID, timestamp, and message.
///
/// # Examples
/// ```
/// use yaru::*;
/// let name = "Tokio";
/// let output = nti_format!("Welcome to the {} executor", name);
/// println!("{}", output);
/// ```
#[macro_export]
macro_rules! nti_format {
    ($($arg:tt)*) => {
        $crate::time_log::nti_format(&format!($($arg)*))
    };
}

// ============================================================================
// Print Macros (print to stdout)
// ============================================================================

/// Formats and logs a message with a relative timestamp.
/// 
/// This macro provides a convenient way to log formatted messages using the same 
/// syntax as `println!`. It calculates the time elapsed since the first call 
/// to any logging function or the last call to `init_time_log()`.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments, following the `format!` syntax.
///
/// # Examples
/// ```
/// use yaru::*;
/// let val = 42;
/// t_print!("Processing value: {}", val);
/// t_print!("Simple message");
/// ```
#[macro_export]
macro_rules! t_print {
    ($($arg:tt)*) => {
        $crate::time_log::t_print(&format!($($arg)*));
    };
}

/// Formats and logs a message with a leading newline and a relative timestamp.
/// 
/// Identical to `t_print!`, but prepends an empty line to the output. This is 
/// especially helpful for visually separating different phases of an application.
/// It uses the same formatting syntax as `println!`.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments.
///
/// # Examples
/// ```
/// use yaru::*;
/// let phase = "Initialization";
/// nt_print!("--- STARTING PHASE: {} ---", phase);
/// ```
#[macro_export]
macro_rules! nt_print {
    ($($arg:tt)*) => {
        $crate::time_log::nt_print(&format!($($arg)*));
    };
}

/// Formats and logs a message with the current thread ID and relative timestamp.
/// 
/// This macro is the formatted version of `ti_print`. It includes the numeric ID 
/// of the thread executing the log, making it indispensable for debugging 
/// multi-threaded or asynchronous code where task switching occurs.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments.
///
/// # Examples
/// ```
/// use yaru::*;
/// let task_id = 5;
/// ti_print!("Executing task #{}", task_id);
/// ```
#[macro_export]
macro_rules! ti_print {
    ($($arg:tt)*) => {
        $crate::time_log::ti_print(&format!($($arg)*));
    };
}

/// Formats and logs a message with thread ID, relative timestamp, and a leading newline.
/// 
/// This is the most comprehensive logging macro in the library. It combines 
/// visual separation (newline), thread identification, and relative timestamping 
/// with full formatting support.
///
/// # Arguments
/// * `($($arg:tt)*)` - Formatting string and optional arguments.
///
/// # Examples
/// ```
/// use yaru::*;
/// let name = "Tokio";
/// nti_print!("Welcome to the {} executor", name);
/// ```
#[macro_export]
macro_rules! nti_print {
    ($($arg:tt)*) => {
        $crate::time_log::nti_print(&format!($($arg)*));
    };
}
