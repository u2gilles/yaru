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

    // Extraction dynamique des en-têtes à partir du premier élément
    let first_json = serde_json::to_value(&items[0]).unwrap_or_default();
    if let Value::Object(map) = first_json {
        let headers: Vec<String> = map.keys().cloned().collect();
        table.set_header(&headers);

        for item in items {
            let val_json = serde_json::to_value(item).unwrap_or_default();
            if let Value::Object(map) = val_json {
                let row: Vec<String> = headers
                    .iter()
                    .map(|h| {
                        let val = map.get(h).unwrap_or(&Value::Null);
                        match val {
                            Value::String(s) => s.clone(),
                            Value::Bool(true) => "✅".to_string(),
                            Value::Bool(false) => "⬜".to_string(),
                            Value::Null => "NULL".to_string(),
                            _ => val.to_string(),
                        }
                    })
                    .collect();
                table.add_row(row);
            }
        }
    }
    println!("\n{table}");
}
