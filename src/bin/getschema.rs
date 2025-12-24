use anyhow::{Context, Result};
use poe_data::versions::get_versions;
use reqwest::header::{ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    bytes: usize,
    rows: usize,
    table_length_bytes: usize,
}

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
        if let Ok(map) = get_file_info(&cache_dir, version, is_poe2) {
            let schema = process_schema(&schema_json, &map, version_num)?;
            fs::write(output_dir.join(output_name), serde_json::to_string(&schema)?)?;
        }
        Ok(())
    };

    process_version(&current_versions.poe, false, "schema.1.min.json", 1)?;
    process_version(&current_versions.poe2, true, "schema.2.min.json", 2)?;

    Ok(())
}

fn process_schema(original_schema: &Value, files: &HashMap<String, FileInfo>, version_num: u64) -> Result<Value> {
    let mut new_schema = original_schema.as_object().context("Invalid schema format")?.clone();

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
    let mut new_tables = Vec::with_capacity(tables.len());
    for table in tables {
        let table_obj = table.as_object().context("Invalid table format")?;
        let valid_for = table_obj.get("validFor").and_then(|v| v.as_u64()).unwrap_or(0);
        if (valid_for & version_num) == 0 {
            continue;
        }

        let mut new_table = table_obj.clone();
        new_table.remove("validFor");

        let name = new_table.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let lookup_name = name.to_lowercase();
        let file_entry = files.get(&lookup_name);

        // Handle tags and row count
        if let Some(tags_array) = new_table.get_mut("tags").and_then(|t| t.as_array_mut()) {
            let has_missing = tags_array.iter().any(|v| v.as_str() == Some("missing"));
            if file_entry.is_none() && !has_missing {
                tags_array.push(Value::String("missing".to_string()));
            }
        }

        if let Some(entry) = file_entry {
            new_table.insert("num_rows".to_string(), entry.rows.into());
        }

        // Calculate offsets and sizes
        if let Some(columns) = new_table.get_mut("columns").and_then(|c| c.as_array_mut()) {
            let mut current_offset = 0;
            for col in columns.iter_mut() {
                if let Some(col_obj) = col.as_object_mut() {
                    let size = get_column_size(col_obj);
                    col_obj.insert("offset".to_string(), current_offset.into());
                    col_obj.insert("cell_bytes".to_string(), size.into());
                    current_offset += size;
                }
            }
        }

        new_tables.push(Value::Object(new_table));
    }

    new_schema.insert("tables".to_string(), Value::Array(new_tables));
    Ok(Value::Object(new_schema))
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

fn get_file_info(cache_dir: &Path, version: &str, is_poe2: bool) -> Result<HashMap<String, FileInfo>> {
    let base_url = poe_data_tools::bundle_loader::cdn_base_url(cache_dir, version)?;
    let fs = poe_data_tools::bundle_fs::FS::from_cdn(&base_url, cache_dir)?;

    let prefix = if is_poe2 { "data/balance/" } else { "data/" };
    let files_owned: Vec<String> = fs.list().filter(|x| x.starts_with(prefix) && x.ends_with(".datc64")).collect();
    let files_to_read: Vec<&str> = files_owned.iter().map(|s| s.as_str()).collect();

    let mut files = HashMap::with_capacity(files_to_read.len());

    for res in fs.batch_read(&files_to_read) {
        match res {
            Ok((path, bytes)) if bytes.len() >= 4 => {
                let rows = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;
                let name = Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or(path).to_string();
                let table_length_bytes = bytes.windows(8).position(|w| w == [0xBB; 8]).expect("magic index not found") - 4;

                files.insert(
                    name,
                    FileInfo {
                        bytes: bytes.len(),
                        rows,
                        table_length_bytes,
                    },
                );
            }
            Ok(_) => {}
            Err((path, e)) => eprintln!("Failed to read {}: {}", path, e),
        }
    }

    for (name, entry) in files.iter().take(10) {
        eprintln!("  {}: {}", name, entry.rows);
    }
    eprintln!("Found {} files for version {}", files.len(), version);
    Ok(files)
}
