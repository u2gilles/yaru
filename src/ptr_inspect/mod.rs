//! # Pointer Inspection Module 🔍
//!
//! A comprehensive toolkit for visualizing memory layout and pointer information in Rust.
//!
//! This module provides both **macros** and **functions** to inspect various Rust types,
//! displaying their memory addresses, lengths, capacities, reference counts, and lock states.
//!
//! ---
//!
//! ## Overview
//!
//! | Category      | Print (stdout)        | Format (returns String) |
//! |---------------|----------------------|-------------------------|
//! | **Macros**    | `print_*_ptr!`       | `format_*_ptr!`         |
//! | **Functions** | `print_*_ptr()`      | `format_*_ptr()`        |
//!
//! ---
//!
//! ## Supported Types
//!
//! - **Universal**: `print_ptr!` / `format_ptr!` (works with any supported type)
//! - **Strings**: `String`, `&str`
//! - **Collections**: `Vec<T>`, `&[T]`, `HashMap<K, V>`
//! - **Smart Pointers**: `Box<T>`, `Rc<T>`, `Arc<T>`
//! - **Synchronization**: `Mutex<T>`, `RwLock<T>`
//! - **References**: `&T`
//!
//! ---
//!
//! ## Quick Start
//!
//! ```rust
//! use yaru::*;
//!
//! let s = String::from("hello");
//! print_ptr!(s);                    // Uses variable name "s" as label
//! print_ptr!(s, "greeting");        // Uses custom label "greeting"
//!
//! let info = format_ptr!(s);        // Returns String instead of printing
//! println!("{}", info);
//! ```
//!
//! ---
//!
//! ## ⚠️ Important: Macro Usage
//!
//! **All macros automatically borrow the value with `&`.**
//!
//! ### ✅ Correct Usage
//!
//! ```rust
//! use yaru::*;
//!
//! let v = vec![1, 2, 3];
//! print_ptr!(v);           // Macro borrows internally → shows address of v
//! print_vec_ptr!(v);       // Macro borrows internally → shows address of v
//! 
//! let b = Box::new(42);
//! print_box_ptr!(b);       // ✅ Correct - macro handles the &
//! 
//! let rc = std::rc::Rc::new(10);
//! print_rc_ptr!(rc);       // ✅ Correct - macro handles the &
//! ```
//!
//! ### ❌ Incorrect Usage
//!
//! ```rust,ignore
//! let v = vec![1, 2, 3];
//! print_ptr!(&v);          // ❌ Creates &&v → shows TEMPORARY reference address!
//! print_vec_ptr!(&v);      // ❌ Creates &&v → shows WRONG address!
//! print_ref_ptr!(&v);      // ❌ Creates &(&v) → inspects temporary, not v!
//! 
//! let b = Box::new(42);
//! print_box_ptr!(&b);      // ❌ Wrong - shows temporary reference address
//! ```
//!
//! **Why this matters:** When you pass `&v` to a macro, it adds another `&`, creating `&&v`.
//! This double reference points to a **temporary location**, not your actual data!
//!
//! **The Simple Rule:** Pass the value directly to macros, without `&`. The macro takes care of borrowing.
//!
//!
//! ---
//!
//! ## Functions vs Macros
//!
//! ### Macros (Recommended for most cases)
//!
//! - Automatically borrow the value
//! - Optional prefix (uses variable name if omitted)
//!
//! ```rust
//! use yaru::*;
//!
//! let v = vec![1, 2, 3];
//! print_vec_ptr!(v);              // Label: "v"
//! print_vec_ptr!(v, "my_vec");    // Label: "my_vec"
//! ```
//!
//! ### Functions (For dynamic prefixes or when you already have a reference)
//!
//! - Require explicit reference `&`
//! - Require explicit prefix string
//!
//! ```rust
//! use yaru::*;
//!
//! let v = vec![1, 2, 3];
//! print_vec_ptr(&v, "v");         // Must pass &v and prefix
//!
//! let prefix = format!("vec_{}", 42);
//! print_vec_ptr(&v, &prefix);     // Dynamic prefix
//! ```
//!
//! ---
//!
//! ## Output Examples
//!
//! ### String
//! ```text
//! s String::{ addr: 0x7ff3a8c04a80, len: 5, cap: 5 }
//! ```
//!
//! ### Vec
//! ```text
//! v Vec::{ addr: 0x7ff3a8c04a90, len: 3, cap: 3 }
//! ```
//!
//! ### Rc / Arc
//! ```text
//! rc Rc::{ addr: 0x7ff3a8c04aa0, strong_count: 2, weak_count: 0 }
//! arc Arc::{ addr: 0x7ff3a8c04ab0, strong_count: 1, weak_count: 0 }
//! ```
//!
//! ### Mutex / RwLock
//! ```text
//! m Mutex::{ addr: 0x7ff3a8c04ac0, state: unlocked }
//! rw RwLock::{ addr: 0x7ff3a8c04ad0, state: readable }
//! ```
//!
//! ---
//!
//! ## Traits
//!
//! - [`InspectPtr`]: Trait for types that can be printed (used by `print_ptr!`)
//! - [`FormatPtr`]: Trait for types that can be formatted (used by `format_ptr!`)

pub mod macros;
pub mod inspector;

pub use inspector::*;
