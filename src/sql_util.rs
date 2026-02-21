use comfy_table::Table;
use serde::Serialize;
use serde_json::Value;

/// Affiche n'importe quel slice de structures qui dérive Serialize sous forme de tableau SQL.
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
            Value::Bool(true) => "✅".to_string(),
            Value::Bool(false) => "⬜".to_string(),
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
