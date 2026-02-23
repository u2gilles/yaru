//! # Yet Another Rust Util (yaru) 🛠️
//!
//! A lightweight collection of utilities for Rust applications.
//!
//! `yaru` provides **relative timestamping**, **thread identification**, **memory layout
//! visualization**, and **SQL result pretty-printing** — all designed for developers and
//! educators who need to understand timing, concurrency, memory models, and database
//! results without heavy frameworks.
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
//! yaru = "0.2"
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
//! ## 🗄️ SQL Result Pretty-Printing
//!
//! Three macros for displaying database query results. The variable name is
//! **automatically captured** via [`stringify!`] — no manual `name` parameter needed.
//!
//! | Macro | Output |
//! |-------|--------|
//! | [`print_sql_table!`] | Pretty UTF-8 table only |
//! | [`print_sql_json_table!`] | `Debug` dump **+** table |
//! | [`print_sql_json!`] | `Debug` dump only |
//!
//! ### Example
//!
//! ```rust
//! use serde::Serialize;
//!
//! #[derive(Debug, Serialize)]
//! struct Task { id: i32, title: String, done: bool }
//!
//! let tasks = vec![
//!     Task { id: 1, title: "Write tests".into(), done: true },
//!     Task { id: 2, title: "Review code".into(), done: false },
//! ];
//!
//! // Table only
//! yaru::print_sql_table!(tasks);
//!
//! // Debug dump + table
//! yaru::print_sql_json_table!(tasks);
//!
//! // Debug dump only
//! yaru::print_sql_json!(tasks);
//! ```
//!
//! Works with **sqlx**, **SeaORM**, **Diesel**, or any `T: Serialize`.
//!
//! ---
//!
//! ## ⚠️ Important: Macro Usage
//!
//! **All pointer inspection macros automatically borrow the value with `&`.**
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
