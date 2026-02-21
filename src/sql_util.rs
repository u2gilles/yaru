//! # SQL Result Pretty-Printing
//!
//! This module provides [`print_sql_result`], a generic function that renders any
//! `&[T]` (where `T: Serialize`) as a pretty UTF-8 table in the terminal.
//!
//! It is designed to work seamlessly with query results from any Rust database
//! library — **sqlx**, **SeaORM**, **Diesel**, or any other — as long as the
//! result types derive [`serde::Serialize`].
//!
//! ## Features
//!
//! | Input shape | What you get |
//! |-------------|--------------|
//! | `&[MyStruct]` | One column per field, one row per item |
//! | `&[(Parent, Vec<Child>)]` | Parent fields + a summarised column for children (1:N) |
//! | `&[(A, Option<B>)]` | Flattened columns; `None` → `NULL` (LEFT JOIN style) |
//! | Struct with `Option` fields | `None` → `NULL`, `Some(v)` → `v` |
//! | `bool` fields | `true` → ✅, `false` → ⬜ |
//! | Nested arrays of objects | Extracts `name` or `title` field if available |
//!
//! ## Quick Example
//!
//! ```rust
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct User { id: i32, name: String, active: bool }
//!
//! let users = vec![
//!     User { id: 1, name: "Alice".into(), active: true },
//!     User { id: 2, name: "Bob".into(),   active: false },
//! ];
//!
//! yaru::print_sql_result(&users);
//! ```
//!
//! Output:
//! ```text
//! ╔════╦═══════╦════════╗
//! ║ id ║ name  ║ active ║
//! ╠════╬═══════╬════════╣
//! ║ 1  ║ Alice ║ ✅     ║
//! ╠════╬═══════╬════════╣
//! ║ 2  ║ Bob   ║ ⬜     ║
//! ╚════╩═══════╩════════╝
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
/// This function serializes each item to JSON via [`serde_json`], dynamically
/// discovers the column names from the struct fields, and renders the result
/// using [`comfy_table`] with the `UTF8_FULL` preset (double-line borders).
///
/// # Arguments
///
/// * `items` — A slice of any type that implements [`Serialize`].
///   Accepts `&Vec<T>` thanks to auto-deref.
///
/// # Supported input shapes
///
/// ## 1. Flat structs — `&[MyStruct]`
///
/// Each field becomes a column header; each item becomes a row.
///
/// ```rust
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User { id: i32, name: String, email: String }
///
/// let users = vec![
///     User { id: 1, name: "Alice".into(), email: "alice@example.com".into() },
///     User { id: 2, name: "Bob".into(),   email: "bob@example.com".into() },
/// ];
/// yaru::print_sql_result(&users);
/// // ╔════╦═══════╦══════════════════╗
/// // ║ id ║ name  ║ email            ║
/// // ╠════╬═══════╬══════════════════╣
/// // ║ 1  ║ Alice ║ alice@example.com║
/// // ║ 2  ║ Bob   ║ bob@example.com  ║
/// // ╚════╩═══════╩══════════════════╝
/// ```
///
/// ## 2. Structs with `Option` fields — NULL handling
///
/// `Option::None` is displayed as `NULL`, which is ideal for LEFT JOIN results.
///
/// ```rust
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct UserProfile {
///     user_id: i32,
///     user_name: String,
///     bio: Option<String>,        // NULL when no profile exists
///     avatar_url: Option<String>, // NULL when no avatar
/// }
///
/// let results = vec![
///     UserProfile {
///         user_id: 1, user_name: "Alice".into(),
///         bio: Some("Hello!".into()), avatar_url: None,
///     },
///     UserProfile {
///         user_id: 2, user_name: "Bob".into(),
///         bio: None, avatar_url: None,
///     },
/// ];
/// yaru::print_sql_result(&results);
/// // ╔═════════╦═══════════╦════════╦════════════╗
/// // ║ user_id ║ user_name ║ bio    ║ avatar_url ║
/// // ╠═════════╬═══════════╬════════╬════════════╣
/// // ║ 1       ║ Alice     ║ Hello! ║ NULL       ║
/// // ║ 2       ║ Bob       ║ NULL   ║ NULL       ║
/// // ╚═════════╩═══════════╩════════════════════════╝
/// ```
///
/// ## 3. Tuples with nested `Vec` — 1:N relations
///
/// When using SeaORM's `find_with_related()`, the result is typically
/// `Vec<(Parent, Vec<Child>)>`. The parent fields are flattened as columns
/// and the children are summarised in an extra column.
///
/// ```rust,ignore
/// // SeaORM example (requires sea-orm dependency)
/// let result: Vec<(User, Vec<Task>)> = user::Entity::find()
///     .find_with_related(task::Entity)
///     .all(&db).await?;
///
/// yaru::print_sql_result(&result);
/// // Parent columns + a "related_1" column listing each child
/// ```
///
/// ## 4. Boolean values
///
/// Booleans are rendered as emoji for quick visual scanning:
/// - `true`  → ✅
/// - `false` → ⬜
///
/// # Empty input
///
/// If the slice is empty, prints `(Aucune donnée)` and returns immediately.
///
/// ```rust
/// let empty: Vec<i32> = vec![];
/// yaru::print_sql_result(&empty);
/// // (Aucune donnée)
/// ```
pub fn print_sql_result<T: Serialize>(items: &[T]) {
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
                let elems: Vec<String> = arr.iter().map(|item| {
                    if let Value::Object(m) = item {
                        if let Some(Value::String(n)) = m.get("name").or(m.get("title")) {
                            return n.clone();
                        }
                    }
                    if let Value::String(s) = item { return s.clone(); }
                    item.to_string()
                }).collect();
                format!("[{}]", elems.join(", "))
            },
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
                                let key_name = if i > 0 { format!("{}_{}", k, i) } else { k.clone() };
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
