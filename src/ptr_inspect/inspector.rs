use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::hash::Hash;

// ============================================================================
// Traits for Unified Inspection
// ============================================================================

/// Trait for unified inspection of different types (printing version).
///
/// This trait is implemented for common Rust types and is used by the `print_ptr!` macro.
/// You typically don't need to call this directly; use the macro instead.
pub trait InspectPtr {
    /// Performs the inspection and prints memory information to stdout.
    fn inspect(&self, prefix: &str);
}

/// Trait for unified inspection of different types (formatting version).
///
/// This trait is implemented for common Rust types and is used by the `format_ptr!` macro.
/// You typically don't need to call this directly; use the macro instead.
pub trait FormatPtr {
    /// Performs the inspection and returns memory information as a String.
    fn format_inspect(&self, prefix: &str) -> String;
}

// ============================================================================
// Trait Implementations for InspectPtr
// ============================================================================

impl InspectPtr for String {
    fn inspect(&self, prefix: &str) {
        print_string_ptr(self, prefix)
    }
}

impl InspectPtr for i32 {
    fn inspect(&self, prefix: &str) {
        print_ref_ptr(self, prefix)
    }
}

impl InspectPtr for u8 {
    fn inspect(&self, prefix: &str) {
        print_ref_ptr(self, prefix)
    }
}

impl InspectPtr for u64 {
    fn inspect(&self, prefix: &str) {
        print_ref_ptr(self, prefix)
    }
}

impl InspectPtr for usize {
    fn inspect(&self, prefix: &str) {
        print_ref_ptr(self, prefix)
    }
}

impl<T> InspectPtr for [T] {
    fn inspect(&self, prefix: &str) {
        print_slice_ptr(self, prefix)
    }
}

impl<T> InspectPtr for &[T] {
    fn inspect(&self, prefix: &str) {
        print_slice_ptr(*self, prefix)
    }
}

impl InspectPtr for &str {
    fn inspect(&self, prefix: &str) {
        print_str_ptr(*self, prefix)
    }
}

impl InspectPtr for str {
    fn inspect(&self, prefix: &str) {
        print_str_ptr(self, prefix)
    }
}

impl<T> InspectPtr for Box<T> {
    fn inspect(&self, prefix: &str) {
        print_box_ptr(self, prefix)
    }
}

impl<T: Debug> InspectPtr for Vec<T> {
    fn inspect(&self, prefix: &str) {
        print_vec_ptr(self, prefix)
    }
}

impl<T: Debug> InspectPtr for Rc<T> {
    fn inspect(&self, prefix: &str) {
        print_rc_ptr(self, prefix)
    }
}

impl<T: Debug> InspectPtr for Arc<T> {
    fn inspect(&self, prefix: &str) {
        print_arc_ptr(self, prefix)
    }
}

impl<T> InspectPtr for Mutex<T> {
    fn inspect(&self, prefix: &str) {
        print_mutex_ptr(self, prefix)
    }
}

impl<T> InspectPtr for RwLock<T> {
    fn inspect(&self, prefix: &str) {
        print_rwlock_ptr(self, prefix)
    }
}

impl<K: Hash + Eq, V> InspectPtr for HashMap<K, V> {
    fn inspect(&self, prefix: &str) {
        print_hashmap_ptr(self, prefix)
    }
}

impl<T: Sized> InspectPtr for &T {
    fn inspect(&self, prefix: &str) {
        print_ref_ptr(*self, prefix)
    }
}

// ============================================================================
// Trait Implementations for FormatPtr
// ============================================================================

impl FormatPtr for String {
    fn format_inspect(&self, prefix: &str) -> String {
        format_string_ptr(self, prefix)
    }
}

impl FormatPtr for i32 {
    fn format_inspect(&self, prefix: &str) -> String {
        format_ref_ptr(self, prefix)
    }
}

impl FormatPtr for u8 {
    fn format_inspect(&self, prefix: &str) -> String {
        format_ref_ptr(self, prefix)
    }
}

impl FormatPtr for u64 {
    fn format_inspect(&self, prefix: &str) -> String {
        format_ref_ptr(self, prefix)
    }
}

impl FormatPtr for usize {
    fn format_inspect(&self, prefix: &str) -> String {
        format_ref_ptr(self, prefix)
    }
}

impl<T> FormatPtr for [T] {
    fn format_inspect(&self, prefix: &str) -> String {
        format_slice_ptr(self, prefix)
    }
}

impl<T> FormatPtr for &[T] {
    fn format_inspect(&self, prefix: &str) -> String {
        format_slice_ptr(*self, prefix)
    }
}

impl FormatPtr for &str {
    fn format_inspect(&self, prefix: &str) -> String {
        format_str_ptr(*self, prefix)
    }
}

impl FormatPtr for str {
    fn format_inspect(&self, prefix: &str) -> String {
        format_str_ptr(self, prefix)
    }
}

impl<T> FormatPtr for Box<T> {
    fn format_inspect(&self, prefix: &str) -> String {
        format_box_ptr(self, prefix)
    }
}

impl<T: Debug> FormatPtr for Vec<T> {
    fn format_inspect(&self, prefix: &str) -> String {
        format_vec_ptr(self, prefix)
    }
}

impl<T: Debug> FormatPtr for Rc<T> {
    fn format_inspect(&self, prefix: &str) -> String {
        format_rc_ptr(self, prefix)
    }
}

impl<T: Debug> FormatPtr for Arc<T> {
    fn format_inspect(&self, prefix: &str) -> String {
        format_arc_ptr(self, prefix)
    }
}

impl<T> FormatPtr for Mutex<T> {
    fn format_inspect(&self, prefix: &str) -> String {
        format_mutex_ptr(self, prefix)
    }
}

impl<T> FormatPtr for RwLock<T> {
    fn format_inspect(&self, prefix: &str) -> String {
        format_rwlock_ptr(self, prefix)
    }
}

impl<K: Hash + Eq, V> FormatPtr for HashMap<K, V> {
    fn format_inspect(&self, prefix: &str) -> String {
        format_hashmap_ptr(self, prefix)
    }
}

impl<T: Sized> FormatPtr for &T {
    fn format_inspect(&self, prefix: &str) -> String {
        format_ref_ptr(*self, prefix)
    }
}

// ============================================================================
// Format Functions (return String)
// ============================================================================

/// Returns memory details of a `String` as a formatted string.
///
/// Displays the heap address where characters are stored, the current length,
/// and the allocated capacity.
///
/// # Arguments
///
/// * `s` - Reference to the String to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let s = String::from("hello");
/// let info = format_string_ptr(&s, "s");
/// assert!(info.contains("String::"));
/// assert!(info.contains("len: 5"));
/// ```
pub fn format_string_ptr(s: &String, prefix: &str) -> String {
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}String::{{ addr: {:p}, len: {}, cap: {} }}",
        padding,
        s.as_ptr(),
        s.len(),
        s.capacity()
    )
}

/// Returns memory details of a string slice `&str` as a formatted string.
///
/// String slices are "fat pointers" containing both an address and a length.
///
/// # Arguments
///
/// * `s` - The string slice to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let text = "hello";
/// let info = format_str_ptr(text, "text");
/// assert!(info.contains("&str::"));
/// assert!(info.contains("len: 5"));
/// ```
pub fn format_str_ptr(s: &str, prefix: &str) -> String {
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}&str::{{ addr: {:p}, len: {} }}",
        padding,
        s.as_ptr(),
        s.len()
    )
}

/// Returns memory details of a slice `&[T]` as a formatted string.
///
/// Slices are "fat pointers" containing both a starting address and a length.
///
/// # Arguments
///
/// * `s` - The slice to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let arr = [1, 2, 3];
/// let slice = &arr[1..];
/// let info = format_slice_ptr(slice, "slice");
/// assert!(info.contains("&[T]::"));
/// assert!(info.contains("len: 2"));
/// ```
pub fn format_slice_ptr<T>(s: &[T], prefix: &str) -> String {
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}&[T]::{{ addr: {:p}, len: {} }}",
        padding,
        s.as_ptr(),
        s.len()
    )
}

/// Returns the heap address of a `Box<T>` as a formatted string.
///
/// # Arguments
///
/// * `b` - Reference to the Box to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let b = Box::new(42);
/// let info = format_box_ptr(&b, "b");
/// assert!(info.contains("Box::"));
/// ```
pub fn format_box_ptr<T>(b: &Box<T>, prefix: &str) -> String {
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}Box::{{ addr: {:p} }}",
        padding, *b
    )
}

/// Returns memory allocation details of a `Vec<T>` as a formatted string.
///
/// Displays the heap address of the vector's buffer, current length, and capacity.
///
/// # Arguments
///
/// * `v` - Reference to the vector to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let v = vec![1, 2, 3];
/// let info = format_vec_ptr(&v, "v");
/// assert!(info.contains("Vec::"));
/// assert!(info.contains("len: 3"));
/// ```
pub fn format_vec_ptr<T: Debug>(v: &Vec<T>, prefix: &str) -> String {
    let addr = v.as_ptr() as *const ();
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}Vec::{{ addr: {:p}, len: {}, cap: {} }}",
        padding,
        addr,
        v.len(),
        v.capacity()
    )
}

/// Returns details of a reference-counted pointer `Rc<T>` as a formatted string.
///
/// Shows the address of the shared data along with strong and weak reference counts.
///
/// # Arguments
///
/// * `rc` - Reference to the Rc to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::rc::Rc;
///
/// let rc = Rc::new(42);
/// let _clone = Rc::clone(&rc);
/// let info = format_rc_ptr(&rc, "rc");
/// assert!(info.contains("Rc::"));
/// assert!(info.contains("strong_count: 2"));
/// ```
pub fn format_rc_ptr<T: Debug>(rc: &Rc<T>, prefix: &str) -> String {
    let addr = Rc::as_ptr(rc) as *const ();
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}Rc::{{ addr: {:p}, strong_count: {}, weak_count: {} }}",
        padding,
        addr,
        Rc::strong_count(rc),
        Rc::weak_count(rc)
    )
}

/// Returns details of a thread-safe reference-counted pointer `Arc<T>` as a formatted string.
///
/// Shows the address of the shared data along with atomic strong and weak counts.
///
/// # Arguments
///
/// * `arc` - Reference to the Arc to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::sync::Arc;
///
/// let arc = Arc::new(100);
/// let info = format_arc_ptr(&arc, "arc");
/// assert!(info.contains("Arc::"));
/// assert!(info.contains("strong_count: 1"));
/// ```
pub fn format_arc_ptr<T: Debug>(arc: &Arc<T>, prefix: &str) -> String {
    let addr = Arc::as_ptr(arc) as *const ();
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}Arc::{{ addr: {:p}, strong_count: {}, weak_count: {} }}",
        padding,
        addr,
        Arc::strong_count(arc),
        Arc::weak_count(arc)
    )
}

/// Returns the address of a reference as a formatted string.
///
/// Useful for understanding where the actual data lives in memory.
///
/// # Arguments
///
/// * `r` - Reference to any data
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let x = 42;
/// let info = format_ref_ptr(&x, "x");
/// assert!(info.contains("Ref::"));
/// ```
pub fn format_ref_ptr<T>(r: &T, prefix: &str) -> String {
    let addr = r as *const T as *const ();
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!("{}Ref::{{ addr: {:p} }}", padding, addr)
}

/// Returns the address and lock state of a `Mutex<T>` as a formatted string.
///
/// Attempts to acquire the lock to determine if it's currently held.
///
/// # Arguments
///
/// * `m` - Reference to the Mutex to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::sync::Mutex;
///
/// let m = Mutex::new(42);
/// let info = format_mutex_ptr(&m, "m");
/// assert!(info.contains("Mutex::"));
/// assert!(info.contains("state: unlocked"));
/// ```
pub fn format_mutex_ptr<T>(m: &Mutex<T>, prefix: &str) -> String {
    let state = match m.try_lock() {
        Ok(_) => "unlocked",
        Err(_) => "locked/poisoned",
    };
    let addr = m as *const Mutex<T> as *const ();
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!("{}Mutex::{{ addr: {:p}, state: {} }}", padding, addr, state)
}

/// Returns the address and lock state of a `RwLock<T>` as a formatted string.
///
/// Attempts to acquire a read lock to determine the current state.
///
/// # Arguments
///
/// * `rw` - Reference to the RwLock to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::sync::RwLock;
///
/// let rw = RwLock::new(true);
/// let info = format_rwlock_ptr(&rw, "rw");
/// assert!(info.contains("RwLock::"));
/// assert!(info.contains("state: readable"));
/// ```
pub fn format_rwlock_ptr<T>(rw: &RwLock<T>, prefix: &str) -> String {
    let state = match rw.try_read() {
        Ok(_) => "readable",
        Err(_) => "exclusive-locked",
    };
    let addr = rw as *const RwLock<T> as *const ();
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!("{}RwLock::{{ addr: {:p}, state: {} }}", padding, addr, state)
}

/// Returns memory details of a `HashMap<K, V>` as a formatted string.
///
/// Shows the address, number of entries, and current bucket capacity.
///
/// # Arguments
///
/// * `hm` - Reference to the HashMap to inspect
/// * `prefix` - A prefix for the output line
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// let info = format_hashmap_ptr(&map, "map");
/// assert!(info.contains("HashMap::"));
/// assert!(info.contains("len: 2"));
/// ```
pub fn format_hashmap_ptr<K, V>(hm: &HashMap<K, V>, prefix: &str) -> String {
    let addr = hm as *const HashMap<K, V> as *const ();
    let padding = if prefix.is_empty() { "".to_string() } else { format!("{} ", prefix) };
    format!(
        "{}HashMap::{{ addr: {:p}, len: {}, capacity: {} }}",
        padding,
        addr,
        hm.len(),
        hm.capacity()
    )
}

// ============================================================================
// Print Functions (call format functions, then println!)
// ============================================================================

/// Prints memory details of a `String`.
///
/// Displays the heap address where characters are stored, the current length,
/// and the allocated capacity.
///
/// # Arguments
///
/// * `s` - Reference to the String to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let s = String::from("hello");
/// print_string_ptr(&s, "s");
/// // Output: s String::{ addr: 0x..., len: 5, cap: 5 }
/// ```
pub fn print_string_ptr(s: &String, prefix: &str) {
    println!("{}", format_string_ptr(s, prefix));
}

/// Prints memory details of a string slice `&str`.
///
/// String slices are "fat pointers" containing both an address and a length.
///
/// # Arguments
///
/// * `s` - The string slice to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let text = "hello";
/// print_str_ptr(text, "text");
/// // Output: text &str::{ addr: 0x..., len: 5 }
/// ```
pub fn print_str_ptr(s: &str, prefix: &str) {
    println!("{}", format_str_ptr(s, prefix));
}

/// Prints memory details of a slice `&[T]`.
///
/// Slices are "fat pointers" containing both a starting address and a length.
///
/// # Arguments
///
/// * `s` - The slice to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let arr = [1, 2, 3];
/// let slice = &arr[1..];
/// print_slice_ptr(slice, "slice");
/// // Output: slice &[T]::{{ addr: 0x..., len: 2 }}
/// ```
pub fn print_slice_ptr<T>(s: &[T], prefix: &str) {
    println!("{}", format_slice_ptr(s, prefix));
}

/// Prints the heap address of a `Box<T>`.
///
/// # Arguments
///
/// * `b` - Reference to the Box to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let b = Box::new(42);
/// print_box_ptr(&b, "b");
/// // Output: b Box::{ addr: 0x... }
/// ```
pub fn print_box_ptr<T>(b: &Box<T>, prefix: &str) {
    println!("{}", format_box_ptr(b, prefix));
}

/// Prints memory allocation details of a `Vec<T>`.
///
/// Displays the heap address of the vector's buffer, current length, and capacity.
///
/// # Arguments
///
/// * `v` - Reference to the vector to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let v = vec![1, 2, 3];
/// print_vec_ptr(&v, "v");
/// // Output: v Vec::{ addr: 0x..., len: 3, cap: 3 }
/// ```
pub fn print_vec_ptr<T: Debug>(v: &Vec<T>, prefix: &str) {
    println!("{}", format_vec_ptr(v, prefix));
}

/// Prints details of a reference-counted pointer `Rc<T>`.
///
/// Shows the address of the shared data along with strong and weak reference counts.
///
/// # Arguments
///
/// * `rc` - Reference to the Rc to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::rc::Rc;
///
/// let rc = Rc::new(42);
/// let _clone = Rc::clone(&rc);
/// print_rc_ptr(&rc, "rc");
/// // Output: rc Rc::{ addr: 0x..., strong_count: 2, weak_count: 0 }
/// ```
pub fn print_rc_ptr<T: Debug>(rc: &Rc<T>, prefix: &str) {
    println!("{}", format_rc_ptr(rc, prefix));
}

/// Prints details of a thread-safe reference-counted pointer `Arc<T>`.
///
/// Shows the address of the shared data along with atomic strong and weak counts.
///
/// # Arguments
///
/// * `arc` - Reference to the Arc to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::sync::Arc;
///
/// let arc = Arc::new(100);
/// print_arc_ptr(&arc, "arc");
/// // Output: arc Arc::{ addr: 0x..., strong_count: 1, weak_count: 0 }
/// ```
pub fn print_arc_ptr<T: Debug>(arc: &Arc<T>, prefix: &str) {
    println!("{}", format_arc_ptr(arc, prefix));
}

/// Prints the address of a reference.
///
/// Useful for understanding where the actual data lives in memory.
///
/// # Arguments
///
/// * `r` - Reference to any data
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
///
/// let x = 42;
/// print_ref_ptr(&x, "x");
/// // Output: x Ref::{ addr: 0x... }
/// ```
pub fn print_ref_ptr<T>(r: &T, prefix: &str) {
    println!("{}", format_ref_ptr(r, prefix));
}

/// Prints the address and lock state of a `Mutex<T>`.
///
/// Attempts to acquire the lock to determine if it's currently held.
///
/// # Arguments
///
/// * `m` - Reference to the Mutex to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::sync::Mutex;
///
/// let m = Mutex::new(42);
/// print_mutex_ptr(&m, "m");
/// // Output: m Mutex::{ addr: 0x..., state: unlocked }
/// ```
pub fn print_mutex_ptr<T>(m: &Mutex<T>, prefix: &str) {
    println!("{}", format_mutex_ptr(m, prefix));
}

/// Prints the address and lock state of a `RwLock<T>`.
///
/// Attempts to acquire a read lock to determine the current state.
///
/// # Arguments
///
/// * `rw` - Reference to the RwLock to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::sync::RwLock;
///
/// let rw = RwLock::new(true);
/// print_rwlock_ptr(&rw, "rw");
/// // Output: rw RwLock::{ addr: 0x..., state: readable }
/// ```
pub fn print_rwlock_ptr<T>(rw: &RwLock<T>, prefix: &str) {
    println!("{}", format_rwlock_ptr(rw, prefix));
}

/// Prints memory details of a `HashMap<K, V>`.
///
/// Shows the address, number of entries, and current bucket capacity.
///
/// # Arguments
///
/// * `hm` - Reference to the HashMap to inspect
/// * `prefix` - A prefix for the output line
///
/// # Example
///
/// ```rust
/// use yaru::*;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// print_hashmap_ptr(&map, "map");
/// // Output: map HashMap::{ addr: 0x..., len: 2, capacity: 3 }
/// ```
pub fn print_hashmap_ptr<K, V>(hm: &HashMap<K, V>, prefix: &str) {
    println!("{}", format_hashmap_ptr(hm, prefix));
}
