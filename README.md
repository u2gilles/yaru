# Yet Another Rust Toolset (yaru) 🛠️

[![Crates.io](https://img.shields.io/crates/v/yaru.svg)](https://crates.io/crates/yaru)
[![Documentation](https://docs.rs/yaru/badge.svg)](https://docs.rs/yaru)
[![License](https://img.shields.io/crates/l/yaru.svg)](https://github.com/u2gilles/yaru#license)

A lightweight collection of utilities for Rust applications, providing:

- **⏱️ Relative Timestamping**: Track elapsed time since application start
- **🧵 Thread Identification**: Debug multi-threaded and async code with thread IDs
- **🔍 Memory Inspection**: Visualize memory layout of Rust types (addresses, lengths, capacities, reference counts, lock states)
- **🗄️ SQL Result Formatting**: Pretty-print any `Serialize` query result as a UTF-8 table

`yaru` is designed for **developers and educators** who need to understand timing, concurrency, memory models, and database results without heavy frameworks.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yaru = "0.2"
```

---

## ⏱️ Time Logging

### Print vs Format

Each logging variant has two versions:
- **Print** (`t_print!`, `nt_print!`, `ti_print!`, `nti_print!`): Output directly to stdout
- **Format** (`t_format!`, `nt_format!`, `ti_format!`, `nti_format!`): Return formatted `String`

| Variant      | Thread ID | Leading Newline | Use Case                           |
|--------------|:---------:|:---------------:|------------------------------------|
| `t*`         | No        | No              | Standard timestamped logging       |
| `nt*`        | No        | Yes             | Visual phase separation            |
| `ti*`        | Yes       | No              | Multi-threaded/async debugging     |
| `nti*`       | Yes       | Yes             | Thread ID + phase separation       |

### Example

```rust
use yaru::*;

fn main() {
    nti_print!("==> Starting main");
    
    std::thread::spawn(|| {
        ti_print!("Worker thread running...");
    }).join().unwrap();
    
    t_print!("All done.");
    
    // Using format variants
    let msg = t_format!("Formatted at runtime");
    println!("{}", msg);
}
```

**Output:**
```text

[01][00:000] ==> Starting main
[02][00:001] Worker thread running...
[00:002] All done.
[00:003] Formatted at runtime
```

### Functions and Macros Available

Each variant has both function and macro versions, in both print and format styles:

**Print to stdout:**
- `t_print(msg)` / `t_print!(...)` 
- `nt_print(msg)` / `nt_print!(...)`
- `ti_print(msg)` / `ti_print!(...)`
- `nti_print(msg)` / `nti_print!(...)`

**Return String:**
- `t_format(msg)` / `t_format!(...)` 
- `nt_format(msg)` / `nt_format!(...)`
- `ti_format(msg)` / `ti_format!(...)`
- `nti_format(msg)` / `nti_format!(...)`

Use `init_time_log()` to manually reset the timer origin.

---

## 🔍 Memory Inspection

Visualize memory layout of Rust types using **macros** or **functions**.

### Print vs Format

| Operation | Output to stdout | Returns `String` |
|-----------|------------------|------------------|
| Macro     | `print_*_ptr!`   | `format_*_ptr!`  |
| Function  | `print_*_ptr()`  | `format_*_ptr()` |

### Supported Types

| Type          | Print Macro           | Format Macro           | Print Function           | Format Function           |
|---------------|-----------------------|------------------------|--------------------------|---------------------------|
| **Universal** | `print_ptr!`          | `format_ptr!`          | —                        | —                         |
| `String`      | `print_string_ptr!`   | `format_string_ptr!`   | `print_string_ptr()`     | `format_string_ptr()`     |
| `&str`        | `print_str_ptr!`      | `format_str_ptr!`      | `print_str_ptr()`        | `format_str_ptr()`        |
| `&[T]`        | `print_slice_ptr!`    | `format_slice_ptr!`    | `print_slice_ptr()`      | `format_slice_ptr()`      |
| `Box<T>`      | `print_box_ptr!`      | `format_box_ptr!`      | `print_box_ptr()`        | `format_box_ptr()`        |
| `Vec<T>`      | `print_vec_ptr!`      | `format_vec_ptr!`      | `print_vec_ptr()`        | `format_vec_ptr()`        |
| `Rc<T>`       | `print_rc_ptr!`       | `format_rc_ptr!`       | `print_rc_ptr()`         | `format_rc_ptr()`         |
| `Arc<T>`      | `print_arc_ptr!`      | `format_arc_ptr!`      | `print_arc_ptr()`        | `format_arc_ptr()`        |
| `&T`          | `print_ref_ptr!`      | `format_ref_ptr!`      | `print_ref_ptr()`        | `format_ref_ptr()`        |
| `Mutex<T>`    | `print_mutex_ptr!`    | `format_mutex_ptr!`    | `print_mutex_ptr()`      | `format_mutex_ptr()`      |
| `RwLock<T>`   | `print_rwlock_ptr!`   | `format_rwlock_ptr!`   | `print_rwlock_ptr()`     | `format_rwlock_ptr()`     |
| `HashMap`     | `print_hashmap_ptr!`  | `format_hashmap_ptr!`  | `print_hashmap_ptr()`    | `format_hashmap_ptr()`    |

### ⚡ Super-Powered Macros (Time + Inspect)

You can combine **Time Logging** and **Memory Inspection** in a single macro call! 

Simply prefix any inspection macro with `t_`, `nt_`, `ti_`, or `nti_`.

**Syntax:** `[prefix]_[macro]!`

| Prefix | Effect | Example |
|--------|--------|---------|
| `t_` | Timestamped inspection | `t_print_vec_ptr!(v)` |
| `nt_` | Newline + Timestamped inspection | `nt_print_vec_ptr!(v)` |
| `ti_` | Thread ID + Timestamped inspection | `ti_print_vec_ptr!(v)` |
| `nti_` | Newline + Thread ID + Timestamped inspection | `nti_print_vec_ptr!(v)` |

**This works for ALL inspection macros:**
- `t_print_ptr!`
- `nti_print_string_ptr!`
- `ti_print_rc_ptr!`
- ...and so on.

### Basic Example

```rust
use yaru::*;

fn main() {
    let s = String::from("hello");
    print_ptr!(s);                    // Uses "s" as label
    print_ptr!(s, "my_string");       // Uses custom label
    
    let info = format_ptr!(s);        // Returns String
    println!("{}", info);
}
```

**Output:**
```text
s String::{ addr: 0x7ff..., len: 5, cap: 5 }
my_string String::{ addr: 0x7ff..., len: 5, cap: 5 }
s String::{ addr: 0x7ff..., len: 5, cap: 5 }
```

### Smart Pointer Example

```rust
use yaru::*;
use std::sync::Arc;

fn main() {
    let arc = Arc::new(42);
    let arc2 = Arc::clone(&arc);
    
    print_arc_ptr!(arc);
    print_arc_ptr!(arc2);
}
```

**Output:**
```text
arc Arc::{ addr: 0x5fa..., strong_count: 2, weak_count: 0 }
arc2 Arc::{ addr: 0x5fa..., strong_count: 2, weak_count: 0 }
```

### Lock State Example

```rust
use yaru::*;
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(100);
    print_mutex_ptr!(m);           // state: unlocked
    
    let _guard = m.lock().unwrap();
    print_mutex_ptr!(m);           // state: locked/poisoned
}
```

---

## 🗄️ SQL Result Formatting

Pretty-print any `&[T]` (where `T: Serialize`) as a UTF-8 table. Works with **sqlx**, **SeaORM**, **Diesel**, or any struct that derives `Serialize`.

Three macros are available — all **automatically capture the variable name** via `stringify!`, so there is no manual `name` parameter:

| Macro | Output |
|-------|--------|
| `print_sql_table!` | Pretty UTF-8 table only |
| `print_sql_json_table!` | `Debug` dump **+** table |
| `print_sql_json!` | `Debug` dump only |

### `print_sql_table!` — Table Only

```rust
use serde::Serialize;

#[derive(Serialize)]
struct User { id: i32, name: String, email: String }

let users = vec![
    User { id: 1, name: "Alice".into(), email: "alice@example.com".into() },
    User { id: 2, name: "Bob".into(),   email: "bob@example.com".into() },
];
yaru::print_sql_table!(users);
```

**Output:**
```text
=> TABLE: users (2)
┌────┬───────┬───────────────────┐
│ id ┆ name  ┆ email             │
╞════╪═══════╪═══════════════════╡
│ 1  ┆ Alice ┆ alice@example.com │
├╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2  ┆ Bob   ┆ bob@example.com   │
└────┴───────┴───────────────────┘
```

### `print_sql_json_table!` — Debug + Table

The most complete debug view: raw `Debug` representation **and** a clean table.

```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Task { id: i32, title: String, done: bool }

let tasks = vec![
    Task { id: 1, title: "Write tests".into(), done: true },
    Task { id: 2, title: "Review code".into(), done: false },
];
yaru::print_sql_json_table!(tasks);
```

**Output:**
```text
=> JSON: tasks (2)
[
    Task { id: 1, title: "Write tests", done: true },
    Task { id: 2, title: "Review code", done: false },
]

=> TABLE: tasks (2)
┌────┬─────────────┬──────┐
│ id ┆ title       ┆ done │
╞════╪═════════════╪══════╡
│ 1  ┆ Write tests ┆ T    │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤
│ 2  ┆ Review code ┆ F    │
└────┴─────────────┴──────┘
```

### `print_sql_json!` — Debug Only

```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Task { id: i32, title: String, done: bool }

let tasks = vec![
    Task { id: 1, title: "Write tests".into(), done: true },
    Task { id: 2, title: "Review code".into(), done: false },
];
yaru::print_sql_json!(tasks);
```

**Output:**
```text
=> JSON: tasks (2)
[
    Task { id: 1, title: "Write tests", done: true },
    Task { id: 2, title: "Review code", done: false },
]
```

### NULL Handling — `Option` Fields

Ideal for LEFT JOIN results where some columns may be `NULL`:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct UserProfile {
    user_id: i32,
    user_name: String,
    bio: Option<String>,
    avatar_url: Option<String>,
}

let results = vec![
    UserProfile { user_id: 1, user_name: "Alice".into(),
                  bio: Some("Hello!".into()), avatar_url: None },
    UserProfile { user_id: 2, user_name: "Bob".into(),
                  bio: None, avatar_url: None },
];
yaru::print_sql_table!(results);
```

**Output:**
```text
=> TABLE: results (2)
┌─────────┬───────────┬────────┬────────────┐
│ user_id ┆ user_name ┆ bio    ┆ avatar_url │
╞═════════╪═══════════╪════════╪════════════╡
│ 1       ┆ Alice     ┆ Hello! ┆ NULL       │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2       ┆ Bob       ┆ NULL   ┆ NULL       │
└─────────┴───────────┴────────┴────────────┘
```

### 1:N Relations — Tuples with Nested `Vec`

Works with SeaORM's `find_with_related()` results (`Vec<(Parent, Vec<Child>)>`):

```rust,ignore
// SeaORM example
let result: Vec<(User, Vec<Task>)> = user::Entity::find()
    .find_with_related(task::Entity)
    .all(&db).await?;

yaru::print_sql_json_table!(result);
// Parent fields are flattened + children summarised in extra columns
```

### Smart Value Formatting

| Rust type | Rendered as |
|-----------|-------------|
| `String` / `&str` | Plain text |
| `bool` | `T` / `F` |
| `Option::None` | `NULL` |
| `Option::Some(v)` | `v` |
| `Vec<T>` (of objects) | `[name1, name2, ...]` (extracts `name` or `title` field) |
| Other | JSON representation |

---

## ⚠️ Important: Macro Usage Best Practices

### Macros Automatically Borrow - Don't Use `&`!

**All pointer inspection macros internally add `&` to the value. Pass values directly, without `&`.**

This is one of the most important rules when using this library's macros:

```rust
use yaru::*;

let v = vec![1, 2, 3];

// ✅ CORRECT: Macro handles borrowing
print_ptr!(v);           // Shows address of v
print_vec_ptr!(v);       // Shows address of v's heap buffer
print_vec_ptr!(v, "my_vec");

// ❌ WRONG: Double reference, shows TEMPORARY reference address
print_ptr!(&v);          // Creates &&v internally → shows temporary!
print_ref_ptr!(&v);      // Shows address of TEMPORARY &v, not v!
```

### Why This Matters

When you write `print_ptr!(&v)`, the macro adds another `&`, creating `&&v`. This double reference points to a **temporary location on the stack**, not the actual data you're trying to inspect. The output will be misleading and won't help you understand your program's memory layout.

**The Simple Rule:** Pass the value directly to macros, without `&`. The macro handles borrowing for you.

### Detailed Examples by Type

#### ✅ Vectors - Correct Usage
```rust
let v = vec![1, 2, 3];
print_vec_ptr!(v);        // ✅ Shows v's heap buffer address
print_ptr!(v);            // ✅ Shows v's heap buffer address
```

#### ❌ Vectors - Incorrect Usage  
```rust
let v = vec![1, 2, 3];
print_vec_ptr!(&v);       // ❌ Shows temporary reference address
print_ptr!(&v);           // ❌ Shows temporary reference address
```

#### ✅ Box - Correct Usage
```rust
let b = Box::new(42);
print_box_ptr!(b);        // ✅ Shows heap allocation address
print_ptr!(b);            // ✅ Shows heap allocation address
```

#### ❌ Box - Incorrect Usage
```rust
let b = Box::new(42);
print_box_ptr!(&b);       // ❌ Shows temporary reference address
```

#### ✅ References - Correct Usage
```rust
let x = 42;
print_ptr!(x);            // ✅ Shows address of x on stack
// If you need to inspect a reference itself:
let r = &x;
print_ref_ptr!(r);        // ✅ Shows where x lives (r is already a reference)
```

#### ❌ References - Incorrect Usage
```rust
let x = 42;
print_ptr!(&x);           // ❌ Shows temporary reference, not x's address!
```

### Functions Require Explicit Reference

When using **functions** directly (not macros), you must provide the reference:

```rust
use yaru::*;

let v = vec![1, 2, 3];

// Function signature: print_vec_ptr(&Vec<T>, prefix: &str)
print_vec_ptr(&v, "v");         // ✅ Explicit & required

// Macro signature: print_vec_ptr!(value) or print_vec_ptr!(value, prefix)
print_vec_ptr!(v);              // ✅ No & needed, macro handles it
print_vec_ptr!(v, "v");         // ✅ No & needed, macro handles it
```

### Summary Table

| You want to...                   | Use this                         | Note                    |
|----------------------------------|----------------------------------|-------------------------|
| Print with auto-label            | `print_vec_ptr!(v)`              | No `&` needed           |
| Print with custom label (macro)  | `print_vec_ptr!(v, "label")`     | No `&` needed           |
| Print with custom label (fn)     | `print_vec_ptr(&v, "label")`     | `&` required for fn     |
| Get formatted String (macro)     | `format_vec_ptr!(v)`             | No `&` needed           |
| Get formatted String (fn)        | `format_vec_ptr(&v, "label")`    | `&` required for fn     |

### Quick Reference

**Macros (most common):**
- ✅ `print_ptr!(value)`
- ✅ `print_vec_ptr!(value)`  
- ✅ `print_box_ptr!(value)`
- ✅ `print_rc_ptr!(value)`
- ❌ `print_ptr!(&value)` ← **DON'T DO THIS**

**Functions (when needed):**
- ✅ `print_vec_ptr(&value, "label")`
- ✅ `format_vec_ptr(&value, "label")`

---

## 🎯 Design Philosophy

1. **Lightweight Dependencies**: Only [`serde`](https://crates.io/crates/serde), [`serde_json`](https://crates.io/crates/serde_json), and [`comfy-table`](https://crates.io/crates/comfy-table).
2. **Educational Focus**: Clear, verbose output for learning.
3. **Non-Invasive**: All functions/macros only borrow data.
4. **Database-Agnostic**: Works with any ORM/query library — sqlx, SeaORM, Diesel, etc.
5. **Consistent API**: `print_*` for output, `format_*` for strings.

---

## 📚 Full Documentation

See the [API documentation on docs.rs](https://docs.rs/yaru) for complete details.

---

## ⚖️ License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
