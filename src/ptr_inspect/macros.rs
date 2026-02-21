// ============================================================================
// Universal Inspection Macros (print_ptr! and format_ptr!)
// ============================================================================

/// Universal macro for inspecting any supported type and printing to stdout.
///
/// This macro borrows the value (using `&`) so it never takes ownership.
/// It can be used in two forms:
///
/// # Forms
///
/// - `print_ptr!(value)` - Uses the variable name as the label
/// - `print_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let x = 42;
/// print_ptr!(x);
///
/// let s = String::from("hello");
/// print_ptr!(s, "greeting");
/// ```
#[macro_export]
macro_rules! print_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::InspectPtr::inspect(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::InspectPtr::inspect(&$val, $prefix)
    };
}

/// Universal macro for inspecting any supported type and returning a formatted String.
///
/// This macro borrows the value (using `&`) so it never takes ownership.
/// It can be used in two forms:
///
/// # Forms
///
/// - `format_ptr!(value)` - Uses the variable name as the label
/// - `format_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let x = 42;
/// let info = format_ptr!(x);
/// println!("{}", info);
///
/// let s = String::from("hello");
/// let info = format_ptr!(s, "greeting");
/// assert!(info.contains("String::"));
/// ```
#[macro_export]
macro_rules! format_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::FormatPtr::format_inspect(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::FormatPtr::format_inspect(&$val, $prefix)
    };
}

// ============================================================================
// String Macros
// ============================================================================

/// Macro for inspecting a `String` and printing to stdout.
///
/// # Forms
///
/// - `print_string_ptr!(value)` - Uses the variable name as the label
/// - `print_string_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let s = String::from("hello");
/// print_string_ptr!(s);
/// print_string_ptr!(s, "my_string");
/// ```
#[macro_export]
macro_rules! print_string_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_string_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_string_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting a `String` and returning a formatted String.
///
/// # Forms
///
/// - `format_string_ptr!(value)` - Uses the variable name as the label
/// - `format_string_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let s = String::from("hello");
/// let info = format_string_ptr!(s);
/// assert!(info.contains("String::"));
/// ```
#[macro_export]
macro_rules! format_string_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_string_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_string_ptr(&$val, $prefix)
    };
}

// ============================================================================
// &str Macros
// ============================================================================

/// Macro for inspecting a string slice `&str` and printing to stdout.
///
/// # Forms
///
/// - `print_str_ptr!(value)` - Uses the variable name as the label
/// - `print_str_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let text = "hello";
/// print_str_ptr!(text);
/// print_str_ptr!(text, "my_text");
/// ```
#[macro_export]
macro_rules! print_str_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_str_ptr($val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_str_ptr($val, $prefix)
    };
}

/// Macro for inspecting a string slice `&str` and returning a formatted String.
///
/// # Forms
///
/// - `format_str_ptr!(value)` - Uses the variable name as the label
/// - `format_str_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let text = "hello";
/// let info = format_str_ptr!(text);
/// assert!(info.contains("&str::"));
/// ```
#[macro_export]
macro_rules! format_str_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_str_ptr($val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_str_ptr($val, $prefix)
    };
}

// ============================================================================
// Slice Macros
// ============================================================================

/// Macro for inspecting a slice `&[T]` and printing to stdout.
///
/// # Forms
///
/// - `print_slice_ptr!(value)` - Uses the variable name as the label
/// - `print_slice_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let arr = [1, 2, 3];
/// let slice = &arr[..];
/// print_slice_ptr!(slice);
/// print_slice_ptr!(slice, "my_slice");
/// ```
#[macro_export]
macro_rules! print_slice_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_slice_ptr($val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_slice_ptr($val, $prefix)
    };
}

/// Macro for inspecting a slice `&[T]` and returning a formatted String.
///
/// # Forms
///
/// - `format_slice_ptr!(value)` - Uses the variable name as the label
/// - `format_slice_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let arr = [1, 2, 3];
/// let slice = &arr[..];
/// let info = format_slice_ptr!(slice);
/// assert!(info.contains("&[T]::"));
/// ```
#[macro_export]
macro_rules! format_slice_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_slice_ptr($val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_slice_ptr($val, $prefix)
    };
}

// ============================================================================
// Box Macros
// ============================================================================

/// Macro for inspecting a `Box<T>` and printing to stdout.
///
/// # Forms
///
/// - `print_box_ptr!(value)` - Uses the variable name as the label
/// - `print_box_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let b = Box::new(42);
/// print_box_ptr!(b);
/// print_box_ptr!(b, "my_box");
/// ```
#[macro_export]
macro_rules! print_box_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_box_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_box_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting a `Box<T>` and returning a formatted String.
///
/// # Forms
///
/// - `format_box_ptr!(value)` - Uses the variable name as the label
/// - `format_box_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let b = Box::new(42);
/// let info = format_box_ptr!(b);
/// assert!(info.contains("Box::"));
/// ```
#[macro_export]
macro_rules! format_box_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_box_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_box_ptr(&$val, $prefix)
    };
}

// ============================================================================
// Vec Macros
// ============================================================================

/// Macro for inspecting a `Vec<T>` and printing to stdout.
///
/// # Forms
///
/// - `print_vec_ptr!(value)` - Uses the variable name as the label
/// - `print_vec_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let v = vec![1, 2, 3];
/// print_vec_ptr!(v);
/// print_vec_ptr!(v, "my_vec");
/// ```
#[macro_export]
macro_rules! print_vec_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_vec_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_vec_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting a `Vec<T>` and returning a formatted String.
///
/// # Forms
///
/// - `format_vec_ptr!(value)` - Uses the variable name as the label
/// - `format_vec_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let v = vec![1, 2, 3];
/// let info = format_vec_ptr!(v);
/// assert!(info.contains("Vec::"));
/// ```
#[macro_export]
macro_rules! format_vec_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_vec_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_vec_ptr(&$val, $prefix)
    };
}

// ============================================================================
// Rc Macros
// ============================================================================

/// Macro for inspecting a `Rc<T>` and printing to stdout.
///
/// # Forms
///
/// - `print_rc_ptr!(value)` - Uses the variable name as the label
/// - `print_rc_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::rc::Rc;
///
/// let rc = Rc::new(42);
/// print_rc_ptr!(rc);
/// print_rc_ptr!(rc, "my_rc");
/// ```
#[macro_export]
macro_rules! print_rc_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_rc_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_rc_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting a `Rc<T>` and returning a formatted String.
///
/// # Forms
///
/// - `format_rc_ptr!(value)` - Uses the variable name as the label
/// - `format_rc_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::rc::Rc;
///
/// let rc = Rc::new(42);
/// let info = format_rc_ptr!(rc);
/// assert!(info.contains("Rc::"));
/// ```
#[macro_export]
macro_rules! format_rc_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_rc_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_rc_ptr(&$val, $prefix)
    };
}

// ============================================================================
// Arc Macros
// ============================================================================

/// Macro for inspecting an `Arc<T>` and printing to stdout.
///
/// # Forms
///
/// - `print_arc_ptr!(value)` - Uses the variable name as the label
/// - `print_arc_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::sync::Arc;
///
/// let arc = Arc::new(100);
/// print_arc_ptr!(arc);
/// print_arc_ptr!(arc, "my_arc");
/// ```
#[macro_export]
macro_rules! print_arc_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_arc_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_arc_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting an `Arc<T>` and returning a formatted String.
///
/// # Forms
///
/// - `format_arc_ptr!(value)` - Uses the variable name as the label
/// - `format_arc_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::sync::Arc;
///
/// let arc = Arc::new(100);
/// let info = format_arc_ptr!(arc);
/// assert!(info.contains("Arc::"));
/// ```
#[macro_export]
macro_rules! format_arc_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_arc_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_arc_ptr(&$val, $prefix)
    };
}

// ============================================================================
// Ref Macros
// ============================================================================

/// Macro for inspecting any reference `&T` and printing to stdout.
///
/// # Forms
///
/// - `print_ref_ptr!(value)` - Uses the variable name as the label
/// - `print_ref_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let x = 42;
/// print_ref_ptr!(x);
/// print_ref_ptr!(x, "my_ref");
/// ```
#[macro_export]
macro_rules! print_ref_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_ref_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_ref_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting any reference `&T` and returning a formatted String.
///
/// # Forms
///
/// - `format_ref_ptr!(value)` - Uses the variable name as the label
/// - `format_ref_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
///
/// let x = 42;
/// let info = format_ref_ptr!(x);
/// assert!(info.contains("Ref::"));
/// ```
#[macro_export]
macro_rules! format_ref_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_ref_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_ref_ptr(&$val, $prefix)
    };
}

// ============================================================================
// Mutex Macros
// ============================================================================

/// Macro for inspecting a `Mutex<T>` and printing to stdout.
///
/// # Forms
///
/// - `print_mutex_ptr!(value)` - Uses the variable name as the label
/// - `print_mutex_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::sync::Mutex;
///
/// let m = Mutex::new(42);
/// print_mutex_ptr!(m);
/// print_mutex_ptr!(m, "my_mutex");
/// ```
#[macro_export]
macro_rules! print_mutex_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_mutex_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_mutex_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting a `Mutex<T>` and returning a formatted String.
///
/// # Forms
///
/// - `format_mutex_ptr!(value)` - Uses the variable name as the label
/// - `format_mutex_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::sync::Mutex;
///
/// let m = Mutex::new(42);
/// let info = format_mutex_ptr!(m);
/// assert!(info.contains("Mutex::"));
/// ```
#[macro_export]
macro_rules! format_mutex_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_mutex_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_mutex_ptr(&$val, $prefix)
    };
}

// ============================================================================
// RwLock Macros
// ============================================================================

/// Macro for inspecting a `RwLock<T>` and printing to stdout.
///
/// # Forms
///
/// - `print_rwlock_ptr!(value)` - Uses the variable name as the label
/// - `print_rwlock_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::sync::RwLock;
///
/// let rw = RwLock::new(true);
/// print_rwlock_ptr!(rw);
/// print_rwlock_ptr!(rw, "my_rwlock");
/// ```
#[macro_export]
macro_rules! print_rwlock_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_rwlock_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_rwlock_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting a `RwLock<T>` and returning a formatted String.
///
/// # Forms
///
/// - `format_rwlock_ptr!(value)` - Uses the variable name as the label
/// - `format_rwlock_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::sync::RwLock;
///
/// let rw = RwLock::new(true);
/// let info = format_rwlock_ptr!(rw);
/// assert!(info.contains("RwLock::"));
/// ```
#[macro_export]
macro_rules! format_rwlock_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_rwlock_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_rwlock_ptr(&$val, $prefix)
    };
}

// ============================================================================
// HashMap Macros
// ============================================================================

/// Macro for inspecting a `HashMap<K, V>` and printing to stdout.
///
/// # Forms
///
/// - `print_hashmap_ptr!(value)` - Uses the variable name as the label
/// - `print_hashmap_ptr!(value, "label")` - Uses a custom label
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// print_hashmap_ptr!(map);
/// print_hashmap_ptr!(map, "my_map");
/// ```
#[macro_export]
macro_rules! print_hashmap_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::print_hashmap_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::print_hashmap_ptr(&$val, $prefix)
    };
}

/// Macro for inspecting a `HashMap<K, V>` and returning a formatted String.
///
/// # Forms
///
/// - `format_hashmap_ptr!(value)` - Uses the variable name as the label
/// - `format_hashmap_ptr!(value, "label")` - Uses a custom label
///
/// # Returns
///
/// A `String` containing the formatted memory information.
///
/// # Examples
///
/// ```rust
/// use yaru::*;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// let info = format_hashmap_ptr!(map);
/// assert!(info.contains("HashMap::"));
/// ```
#[macro_export]
macro_rules! format_hashmap_ptr {
    ($val:expr) => {
        $crate::ptr_inspect::format_hashmap_ptr(&$val, stringify!($val))
    };
    ($val:expr, $prefix:expr) => {
        $crate::ptr_inspect::format_hashmap_ptr(&$val, $prefix)
    };
}

// ============================================================================
// Time-Log Variants Implementation
// ============================================================================

// --- Generic Ptr ---

/// Prints the inspection result with a timestamp.
#[macro_export]
macro_rules! t_print_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_ptr!($val, $prefix)); };
}

/// Prints the inspection result with a timestamp and leading newline.
#[macro_export]
macro_rules! nt_print_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_ptr!($val, $prefix)); };
}

/// Prints the inspection result with a timestamp and thread ID.
#[macro_export]
macro_rules! ti_print_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_ptr!($val, $prefix)); };
}

/// Prints the inspection result with a timestamp, thread ID, and leading newline.
#[macro_export]
macro_rules! nti_print_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_ptr!($val, $prefix)); };
}

// --- String ---

#[macro_export]
macro_rules! t_print_string_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_string_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_string_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_string_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_string_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_string_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_string_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_string_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_string_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_string_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_string_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_string_ptr!($val, $prefix)); };
}

// --- &str ---

#[macro_export]
macro_rules! t_print_str_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_str_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_str_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_str_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_str_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_str_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_str_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_str_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_str_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_str_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_str_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_str_ptr!($val, $prefix)); };
}

// --- Slice ---

#[macro_export]
macro_rules! t_print_slice_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_slice_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_slice_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_slice_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_slice_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_slice_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_slice_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_slice_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_slice_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_slice_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_slice_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_slice_ptr!($val, $prefix)); };
}

// --- Box ---

#[macro_export]
macro_rules! t_print_box_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_box_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_box_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_box_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_box_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_box_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_box_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_box_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_box_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_box_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_box_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_box_ptr!($val, $prefix)); };
}

// --- Vec ---

#[macro_export]
macro_rules! t_print_vec_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_vec_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_vec_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_vec_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_vec_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_vec_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_vec_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_vec_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_vec_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_vec_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_vec_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_vec_ptr!($val, $prefix)); };
}

// --- Rc ---

#[macro_export]
macro_rules! t_print_rc_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_rc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_rc_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_rc_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_rc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_rc_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_rc_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_rc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_rc_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_rc_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_rc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_rc_ptr!($val, $prefix)); };
}

// --- Arc ---

#[macro_export]
macro_rules! t_print_arc_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_arc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_arc_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_arc_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_arc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_arc_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_arc_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_arc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_arc_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_arc_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_arc_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_arc_ptr!($val, $prefix)); };
}

// --- Ref ---

#[macro_export]
macro_rules! t_print_ref_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_ref_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_ref_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_ref_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_ref_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_ref_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_ref_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_ref_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_ref_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_ref_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_ref_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_ref_ptr!($val, $prefix)); };
}

// --- Mutex ---

#[macro_export]
macro_rules! t_print_mutex_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_mutex_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_mutex_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_mutex_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_mutex_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_mutex_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_mutex_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_mutex_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_mutex_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_mutex_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_mutex_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_mutex_ptr!($val, $prefix)); };
}

// --- RwLock ---

#[macro_export]
macro_rules! t_print_rwlock_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_rwlock_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_rwlock_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_rwlock_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_rwlock_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_rwlock_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_rwlock_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_rwlock_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_rwlock_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_rwlock_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_rwlock_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_rwlock_ptr!($val, $prefix)); };
}

// --- HashMap ---

#[macro_export]
macro_rules! t_print_hashmap_ptr {
    ($val:expr) => { $crate::time_log::t_print(&$crate::format_hashmap_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::t_print(&$crate::format_hashmap_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nt_print_hashmap_ptr {
    ($val:expr) => { $crate::time_log::nt_print(&$crate::format_hashmap_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nt_print(&$crate::format_hashmap_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! ti_print_hashmap_ptr {
    ($val:expr) => { $crate::time_log::ti_print(&$crate::format_hashmap_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::ti_print(&$crate::format_hashmap_ptr!($val, $prefix)); };
}

#[macro_export]
macro_rules! nti_print_hashmap_ptr {
    ($val:expr) => { $crate::time_log::nti_print(&$crate::format_hashmap_ptr!($val)); };
    ($val:expr, $prefix:expr) => { $crate::time_log::nti_print(&$crate::format_hashmap_ptr!($val, $prefix)); };
}
