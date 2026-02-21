//! # Yet Another Rust Toolset (yaru) 🛠️
//!
//! A lightweight, zero-dependency collection of utilities for Rust applications.
//!
//! `yaru` provides **relative timestamping**, **thread identification**, and **memory layout 
//! visualization** — all designed for developers and educators who need to understand timing, 
//! concurrency, and memory models without heavy frameworks.
//!
//! ## Modules
//!
//! - **[`time_log`]**: Relative timestamped logging with thread information.
//! - **[`ptr_inspect`]**: Memory layout visualization for various Rust types.
//! - **[`sql_util`]**: Pretty-print any `Serialize` query result as a UTF-8 table.
//!
//! ---
//!
//! ## Quick Start
//!
//! Add `yaru` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! yaru = "0.1"
//! ```
//!
//! Then import everything with:
//!
//! ```rust
//! use yaru::*;
//! ```
//!
//! ---
//!
//! ## ⏱️ Time Logging
//!
//! ### Print vs Format
//!
//! Each logging variant has two versions:
//! - **Print** (`t_print!`, `nt_print!`, `ti_print!`, `nti_print!`): Output directly to stdout
//! - **Format** (`t_format!`, `nt_format!`, `ti_format!`, `nti_format!`): Return formatted `String`
//!
//! | Variant      | Thread ID | Leading Newline | Use Case                  |
//! |--------------|:---------:|:---------------:|---------------------------|
//! | `t*`         | No        | No              | Standard logging          |
//! | `nt*`        | No        | Yes             | Phase separation          |
//! | `ti*`        | Yes       | No              | Multi-threaded debugging  |
//! | `nti*`       | Yes       | Yes             | Thread + phase separation |
//!
//! ### Example
//!
//! ```rust
//! use yaru::*;
//!
//! fn main() {
//!     nti_print!("==> Starting main");
//!     ti_print!("Processing...");
//!     t_print!("Done.");
//!     
//!     // Using format variants
//!     let msg = t_format!("Formatted message");
//!     println!("{}", msg);
//! }
//! ```
//!
//! Output:
//! ```text
//! 
//! [01][00:000] ==> Starting main
//! [01][00:001] Processing...
//! [00:002] Done.
//! [00:003] Formatted message
//! ```
//!
//! ---
//!
//! ## 🔍 Memory Inspection
//!
//! Visualize memory layout of Rust types using macros or functions.
//!
//! ### Print vs Format
//!
//! - **`print_*` macros/functions**: Print to stdout (like `println!`)
//! - **`format_*` macros/functions**: Return a `String` (like `format!`)
//!
//! ### Supported Types
//!
//! | Type          | Print Macro           | Format Macro           |
//! |---------------|-----------------------|------------------------|
//! | Any           | `print_ptr!`          | `format_ptr!`          |
//! | `String`      | `print_string_ptr!`   | `format_string_ptr!`   |
//! | `&str`        | `print_str_ptr!`      | `format_str_ptr!`      |
//! | `&[T]`        | `print_slice_ptr!`    | `format_slice_ptr!`    |
//! | `Box<T>`      | `print_box_ptr!`      | `format_box_ptr!`      |
//! | `Vec<T>`      | `print_vec_ptr!`      | `format_vec_ptr!`      |
//! | `Rc<T>`       | `print_rc_ptr!`       | `format_rc_ptr!`       |
//! | `Arc<T>`      | `print_arc_ptr!`      | `format_arc_ptr!`      |
//! | `&T`          | `print_ref_ptr!`      | `format_ref_ptr!`      |
//! | `Mutex<T>`    | `print_mutex_ptr!`    | `format_mutex_ptr!`    |
//! | `RwLock<T>`   | `print_rwlock_ptr!`   | `format_rwlock_ptr!`   |
//! | `HashMap`     | `print_hashmap_ptr!`  | `format_hashmap_ptr!`  |
//!
//! ### Example
//!
//! ```rust
//! use yaru::*;
//!
//! fn main() {
//!     let s = String::from("hello");
//!     print_ptr!(s);                     // Uses "s" as label
//!     print_ptr!(s, "my_string");        // Uses custom label
//!     
//!     let info = format_ptr!(s);         // Returns String
//!     println!("{}", info);
//! }
//! ```
//!
//! Output:
//! ```text
//! s String::{ addr: 0x7ff..., len: 5, cap: 5 }
//! my_string String::{ addr: 0x7ff..., len: 5, cap: 5 }
//! s String::{ addr: 0x7ff..., len: 5, cap: 5 }
//! ```
//!
//! ---
//!
//! ## ⚠️ Important: Macro Usage
//!
//! **All macros automatically borrow the value with `&`.** 
//!
//! ✅ **Correct usage:**
//! ```rust
//! use yaru::*;
//! 
//! let v = vec![1, 2, 3];
//! print_ptr!(v);           // Correct: macro does &v internally
//! print_vec_ptr!(v);       // Correct: macro does &v internally
//! ```
//!
//! ❌ **Incorrect usage (leads to wrong addresses):**
//! ```rust,ignore
//! let v = vec![1, 2, 3];
//! print_ptr!(&v);          // Wrong: creates &&v (double reference)
//! print_ref_ptr!(&v);      // Wrong: shows address of temporary reference
//! ```
//!
//! **Rule of thumb:** Pass the value directly to macros, without `&`.
//!
//! For functions, you must provide the reference yourself:
//! ```rust
//! use yaru::*;
//! 
//! let v = vec![1, 2, 3];
//! print_vec_ptr(&v, "v");  // Function requires &v
//! ```
//!
//! ---
//!
//! ## License
//!
//! Licensed under either of:
//! - Apache License, Version 2.0
//! - MIT license
//!
//! at your option.

pub mod time_log;
pub mod ptr_inspect;
pub mod sql_util;

/// Re-export of time_log macros and functions for convenience.
pub use time_log::*;

/// Re-export of ptr_inspect macros, functions, and traits for convenience.
pub use ptr_inspect::*;

/// Re-export of sql utilities.
pub use sql_util::*;

/// Re-export of serde for convenience.
pub use serde;
pub use serde_json;
