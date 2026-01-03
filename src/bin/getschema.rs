use anyhow::{Context, Result};
use poe_data::dat_parser::DatLoader;
use poe_data::types::ValidationError;
use poe_data::versions::get_versions;
use reqwest::header::{ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let output_dir = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    let cache_dir = dirs::cache_dir().context("no cache dir")?.join("poe_data_tools");
    fs::create_dir_all(&cache_dir)?;

    let schema_path = cache_dir.join("schema.min.json");
    let headers_path = cache_dir.join("schema.min.json.headers");

    let client = reqwest::blocking::Client::new();
    let schema_url = "https://github.com/poe-tool-dev/dat-schema/releases/download/latest/schema.min.json";

    let mut headers = reqwest::header::HeaderMap::new();
    if schema_path.exists() {
        if let Ok(h_str) = fs::read_to_string(&headers_path) {
            let h: HashMap<String, String> = serde_json::from_str(&h_str).unwrap_or_default();
            if let Some(etag) = h.get("etag").and_then(|v| v.parse().ok()) {
                headers.insert(IF_NONE_MATCH, etag);
            }
            if let Some(lm) = h.get("last-modified").and_then(|v| v.parse().ok()) {
                headers.insert(IF_MODIFIED_SINCE, lm);
            }
        }
    }

    let res = client.get(schema_url).headers(headers).send().context("Failed to request schema")?;

    if res.status().is_success() {
        let mut h_map = HashMap::with_capacity(2);
        if let Some(etag) = res.headers().get(ETAG).and_then(|v| v.to_str().ok()) {
            h_map.insert("etag", etag);
        }
        if let Some(lm) = res.headers().get(LAST_MODIFIED).and_then(|v| v.to_str().ok()) {
            h_map.insert("last-modified", lm);
        }
        fs::write(&headers_path, serde_json::to_string(&h_map)?)?;

        let bytes = res.bytes()?;
        fs::write(&schema_path, bytes)?;
        eprintln!("Downloaded new schema.");
    } else if res.status() == reqwest::StatusCode::NOT_MODIFIED {
        eprintln!("Schema up to date.");
    } else {
        eprintln!("Schema download failed: {}", res.status());
    }

    if !schema_path.exists() {
        anyhow::bail!("Schema file missing");
    }

    let schema_json: Value = serde_json::from_slice(&fs::read(&schema_path)?)?;
    let current_versions = get_versions()?;

    let process_version = |version: &str, is_poe2: bool, output_name: &str, version_num: u64| -> Result<()> {
        if version == "error" {
            return Ok(());
        }
        eprintln!("Checking info for PoE {}: {}", if is_poe2 { "2" } else { "1" }, version);
        let mut loader = DatLoader::new(version)?;
        let schema = process_schema(&schema_json, &mut loader, version_num)?;
        fs::write(output_dir.join(output_name), serde_json::to_string(&schema)?)?;
        Ok(())
    };

    process_version(&current_versions.poe, false, "schema.1.min.json", 1)?;
    process_version(&current_versions.poe2, true, "schema.2.min.json", 2)?;

    Ok(())
}

fn process_schema(original_schema: &Value, loader: &mut DatLoader, version_num: u64) -> Result<Value> {
    let mut new_schema = original_schema.as_object().context("Invalid schema format")?.clone();

    // Prepare file list for validation
    let mut file_list = loader.get_file_list();
    for f in file_list.iter_mut() {
        *f = f.to_lowercase();
    }
    file_list.sort();

    let enumerations = original_schema
        .get("enumerations")
        .and_then(|t| t.as_array())
        .context("Missing enumerations array")?;
    let mut new_enumerations = Vec::with_capacity(enumerations.len());
    for enumerator in enumerations {
        let enumerator_obj = enumerator.as_object().context("Invalid enumerator format")?;
        let valid_for = enumerator_obj.get("validFor").and_then(|v| v.as_u64()).unwrap_or(0);
        if (valid_for & version_num) == 0 {
            continue;
        }

        let mut new_enumerator = enumerator_obj.clone();
        new_enumerator.remove("validFor");
        new_enumerations.push(Value::Object(new_enumerator));
    }
    new_schema.insert("enumerations".to_string(), Value::Array(new_enumerations));

    let tables = original_schema.get("tables").and_then(|t| t.as_array()).context("Missing tables array")?;

    // First pass: Collect relevant tables and batch load them
    let mut tables_to_load = Vec::new();
    for table in tables {
        let table_obj = table.as_object().context("Invalid table format")?;
        let valid_for = table_obj.get("validFor").and_then(|v| v.as_u64()).unwrap_or(0);
        if (valid_for & version_num) == 0 {
            continue;
        }
        if let Some(name) = table_obj.get("name").and_then(|n| n.as_str()) {
            // Handle poe2 pathing
            if version_num == 2 {
                tables_to_load.push(format!("data/balance/{}.datc64", name.to_lowercase()));
            } else {
                tables_to_load.push(format!("data/{}.datc64", name.to_lowercase()));
            }
        }
    }

    let tables_ref: Vec<&str> = tables_to_load.iter().map(|s| s.as_str()).collect();
    loader.load_tables(&tables_ref);

    let mut new_tables = Vec::with_capacity(tables.len());
    for table in tables {
        let table_obj = table.as_object().context("Invalid table format")?;
        let valid_for = table_obj.get("validFor").and_then(|v| v.as_u64()).unwrap_or(0);
        if (valid_for & version_num) == 0 {
            continue;
        }

        let mut new_table = table_obj.clone();

        // Add inherited tag
        if valid_for == 3 && version_num == 2 {
            if let Some(tags_array) = new_table.get_mut("tags").and_then(|t| t.as_array_mut()) {
                if !tags_array.iter().any(|v| v.as_str() == Some("inherited")) {
                    tags_array.push(Value::String("inherited".to_string()));
                }
            } else {
                new_table.insert("tags".to_string(), Value::Array(vec![Value::String("inherited".to_string())]));
            }
        }

        new_table.remove("validFor");

        let name = new_table.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let file_path = if version_num == 2 {
            format!("data/balance/{}.datc64", name.to_lowercase())
        } else {
            format!("data/{}.datc64", name.to_lowercase())
        };

        let dat_file = loader.dat_files.get(&file_path);

        // Handle tags and row count
        if let Some(tags_array) = new_table.get_mut("tags").and_then(|t| t.as_array_mut()) {
            let has_missing = tags_array.iter().any(|v| v.as_str() == Some("missing"));
            if dat_file.is_none() && !has_missing {
                tags_array.push(Value::String("missing".to_string()));
            }
        }

        if let Some(df) = dat_file {
            // Replicate FileInfo logic
            // num_bytes_tdata (fixed part) = table.len() ?
            // DatLoader parses: table is data.split_to(magic_index).
            // But table includes the row count (4 bytes) at start?
            // DatLoader: `let table_len_rows = table.get_u32_le() as usize;`
            // This advances the buffer. `df.table` is the remaining.
            // But we want the size on disk?
            // `df.table.len()` is fixed data *minus 4 bytes*.
            // `df.vdata.len()` is variable data *including magic (8 bytes)*?
            // No, `data.split_to(magic)` -> `data` remains. `data` is `vdata`.
            // `df.vdata` has magic bytes at start (checked by position).
            // So `df.table` + `df.vdata` + 4 bytes (row count) = Total File Size?
            // Let's approximate or just use what we have.
            // schema.min.json expects num_bytes, num_bytes_tdata, etc.

            let tdata_len = df.table.len() + 4; // Add back the row count bytes
            let vdata_len = df.vdata.len();
            let total_len = tdata_len + vdata_len;

            new_table.insert("num_bytes".to_string(), total_len.into());
            new_table.insert("num_bytes_tdata".to_string(), tdata_len.into());
            new_table.insert("num_bytes_vdata".to_string(), vdata_len.into());
            new_table.insert("num_rows".to_string(), df.row_count.into());
            new_table.insert("num_bytes_row".to_string(), df.bytes_per_row.into());
        }

        // Calculate offsets and sizes
        if let Some(columns) = new_table.get_mut("columns").and_then(|c| c.as_array_mut()) {
            let mut current_offset = 0;
            for col in columns.iter_mut() {
                if let Some(col_obj) = col.as_object_mut() {
                    let size = get_column_size(col_obj);
                    col_obj.insert("offset".to_string(), current_offset.into());
                    col_obj.insert("cell_bytes".to_string(), size.into());

                    if let Some(df) = dat_file {
                        if (current_offset + size) as usize > df.bytes_per_row {
                            let reason = Value::String("column end > row length".to_string());
                            add_error(col_obj, reason);
                        } else {
                            // Validate column type
                            if let Some(err) = validate_column_type(df, col_obj, current_offset as usize, size, &file_list) {
                                let err_json = serde_json::to_value(&err).unwrap_or(Value::String("Validation failed".to_string()));
                                add_error(col_obj, err_json);
                            }
                        }
                    }

                    current_offset += size;
                }
            }
        }

        new_tables.push(Value::Object(new_table));
    }

    new_schema.insert("tables".to_string(), Value::Array(new_tables));
    Ok(Value::Object(new_schema))
}

fn add_error(col_obj: &mut serde_json::Map<String, Value>, error: Value) {
    if let Some(arr) = col_obj.get_mut("errors").and_then(|v| v.as_array_mut()) {
        arr.push(error);
    } else {
        col_obj.insert("errors".to_string(), Value::Array(vec![error]));
    }
}

fn validate_column_type(
    dat_file: &poe_data::dat_parser::DatFile,
    col_obj: &serde_json::Map<String, Value>,
    offset: usize,
    _size: u32,
    known_files: &[String],
) -> Option<ValidationError> {
    let type_str = col_obj.get("type").and_then(|v| v.as_str()).unwrap_or("");
    let is_array = col_obj.get("array").and_then(|v| v.as_bool()).unwrap_or(false);

    if is_array {
        return poe_data::validators::is_valid_array(dat_file, offset).err();
    }

    match type_str {
        "bool" => poe_data::validators::is_valid_bool(dat_file, offset).err(),
        "string" => poe_data::validators::is_valid_string_pointer(dat_file, offset).err(),
        "file" => poe_data::validators::is_valid_file_path(dat_file, offset, Some(known_files)).err(),
        // "directory" type isn't standard in schema usually but if it appears:
        "directory" => poe_data::validators::is_valid_directory_path(dat_file, offset, Some(known_files)).err(),
        "color" => poe_data::validators::is_valid_color(dat_file, offset).err(),
        _ => None,
    }
}

fn get_column_size(col: &serde_json::Map<String, Value>) -> u32 {
    if col.get("array").and_then(|v| v.as_bool()) == Some(true) {
        return 16;
    }
    if col.get("interval").and_then(|v| v.as_bool()) == Some(true) {
        return 8;
    }
    match col.get("type").and_then(|v| v.as_str()).unwrap_or("") {
        "bool" => 1,
        "string" => 8,
        "enumrow" | "i32" | "u32" | "f32" => 4,
        "row" | "i64" | "u64" | "f64" => 8,
        "foreignrow" => 16,
        "i16" | "u16" => 2,
        type_str => {
            if !type_str.is_empty() {
                eprintln!("Unknown type: {}", type_str);
            }
            0
        }
    }
}
