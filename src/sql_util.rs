//! # SQL Result Pretty-Printing
//!
//! This module provides three macros for displaying database query results
//! in the terminal:
//!
//! | Macro | Output |
//! |-------|--------|
//! | [`print_sql_table!`] | Pretty UTF-8 table only |
//! | [`print_sql_json_table!`] | `Debug` dump **+** table |
//! | [`print_sql_json!`] | `Debug` dump only |
//!
//! All three macros **automatically capture the variable name** you pass in
//! (via [`stringify!`]), so there is no need for a separate `name` parameter.
//!
//! They work with any `&[T]` (where `T: Serialize`) and are designed for
//! seamless use with **sqlx**, **SeaORM**, **Diesel**, or any other Rust
//! database library — as long as the result types derive [`serde::Serialize`].
//!
//! ## Features
//!
//! | Input shape | What you get |
//! |-------------|--------------|
//! | `&[MyStruct]` | One column per field, one row per item |
//! | `&[(Parent, Vec<Child>)]` | Parent fields + a summarised column for children (1:N) |
//! | `&[(A, Option<B>)]` | Flattened columns; `None` → `NULL` (LEFT JOIN style) |
//! | Struct with `Option` fields | `None` → `NULL`, `Some(v)` → `v` |
//! | `bool` fields | `true` → T, `false` → F |
//! | Nested arrays of objects | Extracts `name` or `title` field if available |
//!
//! ## Quick Example
//!
//! ```rust
//! use serde::Serialize;
//!
//! #[derive(Debug, Serialize)]
//! struct User { id: i32, name: String, active: bool }
//!
//! let users = vec![
//!     User { id: 1, name: "Alice".into(), active: true },
//!     User { id: 2, name: "Bob".into(),   active: false },
//! ];
//!
//! // Just the table
//! yaru::print_sql_table!(users);
//!
//! // Debug dump + table
//! yaru::print_sql_json_table!(users);
//!
//! // Debug dump only
//! yaru::print_sql_json!(users);
//! ```
//!
//! ## Database-Agnostic
//!
//! The only requirement is `T: Serialize`. This means you can use it with:
//!
//! - **sqlx** structs (`#[derive(sqlx::FromRow, Serialize)]`)
//! - **SeaORM** models (`#[derive(DeriveEntityModel, Serialize)]`)
//! - **Diesel** queryable structs
//! - Any custom struct that derives `Serialize`

use comfy_table::Table;
use serde::Serialize;
use serde_json::Value;

/// Prints a slice of [`Serialize`]-able items as a pretty UTF-8 table to stdout.
///
/// The variable name is **automatically captured** — no need to pass it manually.
/// Internally the macro uses [`stringify!`] to reflect the expression you pass in.
///
/// # Usage
///
/// ```rust
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User { id: i32, name: String, active: bool }
///
/// let users = vec![
///     User { id: 1, name: "Alice".into(), active: true },
///     User { id: 2, name: "Bob".into(),   active: false },
/// ];
///
/// yaru::print_sql_table!(users);
/// ```
///
/// Output:
/// ```text
/// => TABLE: users
/// ╔════╦═══════╦════════╗
/// ║ id ║ name  ║ active ║
/// ╠════╬═══════╬════════╣
/// ║ 1  ║ Alice ║ T      ║
/// ╠════╬═══════╬════════╣
/// ║ 2  ║ Bob   ║ F      ║
/// ╚════╩═══════╩════════╝
/// ```
///
/// # Supported input shapes
///
/// | Input shape | What you get |
/// |-------------|--------------|
/// | `&[MyStruct]` | One column per field, one row per item |
/// | `&[(Parent, Vec<Child>)]` | Parent fields + a summarised column for children (1:N) |
/// | `&[(A, Option<B>)]` | Flattened columns; `None` → `NULL` (LEFT JOIN style) |
/// | Struct with `Option` fields | `None` → `NULL`, `Some(v)` → `v` |
/// | `bool` fields | `true` → T, `false` → F |
/// | Nested arrays of objects | Extracts `name` or `title` field if available |
///
/// # Empty input
///
/// If the slice is empty, prints `(Aucune donnée)` and returns immediately.
#[macro_export]
macro_rules! print_sql_table {
    ($items:expr) => {
        print!("\n=> TABLE: {} ({})", stringify!($items), $items.len());
        $crate::sql_util::internal_print_sql_table(&$items);
    };
}

/// Prints a slice of [`Serialize`] + [`Debug`] items as **JSON first, then as a table**.
///
/// This is the most complete debug view: you see the raw `Debug` representation
/// (handy for nested `Option`s, enums, etc.) **and** a clean tabular rendering
/// right below it. The variable name is **automatically captured** via [`stringify!`].
///
/// # Usage
///
/// ```rust
/// use serde::Serialize;
///
/// #[derive(Debug, Serialize)]
/// struct Task { id: i32, title: String, done: bool }
///
/// let tasks = vec![
///     Task { id: 1, title: "Write tests".into(), done: true },
///     Task { id: 2, title: "Review code".into(), done: false },
/// ];
///
/// yaru::print_sql_json_table!(tasks);
/// ```
///
/// Output:
/// ```text
/// => JSON : tasks (2) :
/// [
///     Task { id: 1, title: "Write tests", done: true },
///     Task { id: 2, title: "Review code", done: false },
/// ]
///
/// => TABLE: tasks (2) :
/// ╔════╦═══════════════╦══════╗
/// ║ id ║ title         ║ done ║
/// ╠════╬═══════════════╬══════╣
/// ║ 1  ║ Write tests   ║ T    ║
/// ╠════╬═══════════════╬══════╣
/// ║ 2  ║ Review code   ║ F    ║
/// ╚════╩═══════════════╩══════╝
/// ```
///
/// # When to use
///
/// Use this macro when you want to **inspect the raw data structure** (via `Debug`)
/// alongside the pretty table — for example when debugging SeaORM query results
/// that contain `Option`, tuples, or nested relations.
#[macro_export]
macro_rules! print_sql_json_table {
    ($items:expr) => {
        println!(
            "\n=> JSON: {} ({})\n{:#?}",
            stringify!($items),
            $items.len(),
            $items
        );
        print!("\n=> TABLE: {} ({})", stringify!($items), $items.len());
        $crate::sql_util::internal_print_sql_table(&$items);
    };
}

/// Prints a slice of [`Serialize`] + [`Debug`] items as **JSON only** (no table).
///
/// Uses Rust's `{:#?}` pretty-print format to display the full `Debug`
/// representation. The variable name and item count are printed as a header.
/// The variable name is **automatically captured** via [`stringify!`].
///
/// # Usage
///
/// ```rust
/// use serde::Serialize;
///
/// #[derive(Debug, Serialize)]
/// struct Task { id: i32, title: String, done: bool }
///
/// let tasks = vec![
///     Task { id: 1, title: "Write tests".into(), done: true },
///     Task { id: 2, title: "Review code".into(), done: false },
/// ];
///
/// yaru::print_sql_json!(tasks);
/// ```
///
/// Output:
/// ```text
/// => JSON : tasks (2) :
/// [
///     Task { id: 1, title: "Write tests", done: true },
///     Task { id: 2, title: "Review code", done: false },
/// ]
/// ```
///
/// # When to use
///
/// Use this macro when you only need the raw `Debug` dump — for example to
/// quickly compare two result sets, or when the data contains deeply nested
/// structures that are easier to read in `Debug` form than in a table.
#[macro_export]
macro_rules! print_sql_json {
    ($items:expr) => {
        println!(
            "\n=> JSON: {} ({})\n{:#?}",
            stringify!($items),
            $items.len(),
            $items
        );
    };
}

#[doc(hidden)]
pub fn internal_print_sql_table<T: Serialize>(items: &[T]) {
    if items.is_empty() {
        println!("(Aucune donnée)");
        return;
    }

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);

    fn format_value(v: &Value) -> String {
        match v {
            Value::String(s) => s.clone(),
            Value::Bool(true) => "T".to_string(),
            Value::Bool(false) => "F".to_string(),
            Value::Null => "NULL".to_string(),
            Value::Array(arr) => {
                let elems: Vec<String> = arr
                    .iter()
                    .map(|item| {
                        if let Value::Object(m) = item {
                            if let Some(Value::String(n)) = m.get("name").or(m.get("title")) {
                                return n.clone();
                            }
                        }
                        if let Value::String(s) = item {
                            return s.clone();
                        }
                        item.to_string()
                    })
                    .collect();
                format!("[{}]", elems.join(", "))
            }
            Value::Object(_) => v.to_string(),
            _ => v.to_string(),
        }
    }

    fn flatten_item(val: &Value) -> Vec<(String, String)> {
        let mut cols = Vec::new();
        match val {
            Value::Object(map) => {
                for (k, v) in map {
                    cols.push((k.clone(), format_value(v)));
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    match v {
                        Value::Object(map) => {
                            for (k, inner_v) in map {
                                let key_name = if i > 0 {
                                    format!("{}_{}", k, i)
                                } else {
                                    k.clone()
                                };
                                cols.push((key_name, format_value(inner_v)));
                            }
                        }
                        Value::Array(_) => {
                            cols.push((format!("related_{}", i), format_value(v)));
                        }
                        Value::Null => {} // Ignoring null values from Option::None in outer joins
                        _ => {
                            cols.push((format!("col_{}", i), format_value(v)));
                        }
                    }
                }
            }
            _ => {
                cols.push(("value".to_string(), format_value(val)));
            }
        }
        cols
    }

    let mut headers: Vec<String> = Vec::new();
    let mut rows_data = Vec::new();

    for item in items {
        let val_json = serde_json::to_value(item).unwrap_or_default();
        let cols = flatten_item(&val_json);

        for (k, _) in &cols {
            if !headers.contains(k) {
                headers.push(k.clone());
            }
        }
        rows_data.push(cols);
    }

    if headers.is_empty() {
        println!("┌┐\n└┘");
        return;
    }

    table.set_header(&headers);

    for cols in rows_data {
        let mut row_output = vec!["".to_string(); headers.len()];
        for (k, str_val) in cols {
            if let Some(pos) = headers.iter().position(|h| h == &k) {
                row_output[pos] = str_val;
            }
        }
        table.add_row(row_output);
    }

    println!("\n{table}");
}
