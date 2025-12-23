use anyhow::{Context, Result};
use poe_data::versions::get_versions;
use reqwest::header::{ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct CachedFile {
    rows: u32,
    bundle_name: String,
    offset: u32,
    size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct RowCountsCache {
    poe_version: String,
    poe2_version: String,
    files_1: HashMap<String, CachedFile>,
    files_2: HashMap<String, CachedFile>,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let output_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from(".")
    };

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    // 0. Setup
    let cache_dir = dirs::cache_dir()
        .ok_or(anyhow::anyhow!("no cache dir"))?
        .join("poe_data_tools");
    fs::create_dir_all(&cache_dir)?;

    let schema_path = cache_dir.join("schema.min.json");
    let headers_path = cache_dir.join("schema.min.json.headers");
    let row_counts_cache_path = cache_dir.join("row_counts.json");

    // 1. Download Schema (Conditional)
    let client = reqwest::blocking::Client::new();
    let schema_url =
        "https://github.com/poe-tool-dev/dat-schema/releases/download/latest/schema.min.json";

    let mut headers = reqwest::header::HeaderMap::new();
    if schema_path.exists() && headers_path.exists() {
        if let Ok(saved_headers) = fs::read_to_string(&headers_path) {
            let h: HashMap<String, String> =
                serde_json::from_str(&saved_headers).unwrap_or_default();
            if let Some(etag) = h.get("etag") {
                if let Ok(val) = etag.parse() {
                    headers.insert(IF_NONE_MATCH, val);
                }
            }
            if let Some(lm) = h.get("last-modified") {
                if let Ok(val) = lm.parse() {
                    headers.insert(IF_MODIFIED_SINCE, val);
                }
            }
        }
    }

    // Handle redirects automatically by reqwest
    let res = client
        .get(schema_url)
        .headers(headers)
        .send()
        .context("Failed to request schema")?;

    let schema_changed = if res.status().is_success() {
        let resp_headers = res.headers().clone();
        let mut h_map = HashMap::new();
        if let Some(etag) = resp_headers.get(ETAG) {
            if let Ok(s) = etag.to_str() {
                h_map.insert("etag".to_string(), s.to_string());
            }
        }
        if let Some(lm) = resp_headers.get(LAST_MODIFIED) {
            if let Ok(s) = lm.to_str() {
                h_map.insert("last-modified".to_string(), s.to_string());
            }
        }
        fs::write(&headers_path, serde_json::to_string(&h_map)?)?;

        let bytes = res.bytes()?;
        fs::write(&schema_path, bytes)?;
        eprintln!("Downloaded new schema.");
        true
    } else if res.status() == reqwest::StatusCode::NOT_MODIFIED {
        eprintln!("Schema up to date.");
        false
    } else {
        eprintln!("Schema download failed: {}", res.status());
        false
    };

    if !schema_path.exists() {
        anyhow::bail!("Schema file missing");
    }

    let schema_json: Value = serde_json::from_str(&fs::read_to_string(&schema_path)?)?;

    // 2. Check Versions
    let current_versions = get_versions()?;

    // Load row count cache
    let mut cache = if row_counts_cache_path.exists() {
        fs::read_to_string(&row_counts_cache_path)
            .ok()
            .and_then(|f| serde_json::from_str(&f).ok())
            .unwrap_or_else(|| RowCountsCache {
                poe_version: String::new(),
                poe2_version: String::new(),
                files_1: HashMap::new(),
                files_2: HashMap::new(),
            })
    } else {
        RowCountsCache {
            poe_version: String::new(),
            poe2_version: String::new(),
            files_1: HashMap::new(),
            files_2: HashMap::new(),
        }
    };

    let mut cache_changed = false;
    let mut poe1_updated = false;
    let mut poe2_updated = false;

    // Check PoE 1
    if current_versions.poe != "error" {
        eprintln!("Checking info for PoE 1: {}", current_versions.poe);
        if let Ok(map) = get_file_info(&cache_dir, &current_versions.poe, false, &cache.files_1) {
            if cache.poe_version != current_versions.poe {
                cache.poe_version = current_versions.poe.clone();
                cache_changed = true;
                poe1_updated = true;
            }
            if map.len() != cache.files_1.len() {
                cache_changed = true;
                poe1_updated = true;
            } else {
                for (k, v) in &map {
                    if let Some(old) = cache.files_1.get(k) {
                        if old.rows != v.rows {
                            cache_changed = true;
                            poe1_updated = true;
                            break;
                        }
                    } else {
                        cache_changed = true;
                        poe1_updated = true;
                        break;
                    }
                }
            }
            cache.files_1 = map;
        }
    }

    // Check PoE 2
    if current_versions.poe2 != "error" {
        eprintln!("Checking info for PoE 2: {}", current_versions.poe2);
        if let Ok(map) = get_file_info(&cache_dir, &current_versions.poe2, true, &cache.files_2) {
            if cache.poe2_version != current_versions.poe2 {
                cache.poe2_version = current_versions.poe2.clone();
                cache_changed = true;
                poe2_updated = true;
            }
            if map.len() != cache.files_2.len() {
                cache_changed = true;
                poe2_updated = true;
            } else {
                for (k, v) in &map {
                    if let Some(old) = cache.files_2.get(k) {
                        if old.rows != v.rows {
                            cache_changed = true;
                            poe2_updated = true;
                            break;
                        }
                    } else {
                        cache_changed = true;
                        poe2_updated = true;
                        break;
                    }
                }
            }
            cache.files_2 = map;
        }
    }

    if cache_changed {
        fs::write(&row_counts_cache_path, serde_json::to_string(&cache)?)?;
    }

    let output_1 = output_dir.join("schema.1.min.json");
    if schema_changed || poe1_updated || !output_1.exists() {
        eprintln!("Generating output schema 1...");
        let schema1 = process_schema(&schema_json, &cache.files_1, 1)?;
        fs::write(&output_1, serde_json::to_string(&schema1)?)?;
    }

    let output_2 = output_dir.join("schema.2.min.json");
    if schema_changed || poe2_updated || !output_2.exists() {
        eprintln!("Generating output schema 2...");
        let schema2 = process_schema(&schema_json, &cache.files_2, 2)?;
        fs::write(&output_2, serde_json::to_string(&schema2)?)?;
    }

    if !schema_changed && !poe1_updated && !poe2_updated && output_1.exists() && output_2.exists() {
        eprintln!("No changes detected. Schemas are up to date.");
    }

    Ok(())
}

fn process_schema(
    original_schema: &Value,
    files: &HashMap<String, CachedFile>,
    version_num: u64,
) -> Result<Value> {
    let mut schema = original_schema.clone();

    if let Some(tables) = schema.get_mut("tables").and_then(|t| t.as_array_mut()) {
        let mut new_tables = Vec::new();

        for table in tables.iter() {
            let valid_for = table.get("validFor").and_then(|v| v.as_u64()).unwrap_or(0);
            if (valid_for & version_num) == 0 {
                continue;
            }

            let mut table_obj = table.as_object().unwrap().clone();
            // Clone the name string to avoid borrowing from table_obj while we mutate it
            let name = table_obj
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string();

            // Remove validFor as it is implicit in the split file
            //table_obj.remove("validFor");

            let tags = table_obj
                .get("tags")
                .and_then(|t| t.as_array())
                .cloned()
                .unwrap_or_default();

            let mut tags_set: HashSet<String> = tags
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();

            let lookup_name = name.to_lowercase();

            if !files.contains_key(&lookup_name) && !tags_set.contains("missing") {
                tags_set.insert("missing".to_string());
            }

            // Add row count
            if let Some(entry) = files.get(&lookup_name) {
                table_obj.insert(
                    "num_rows".to_string(),
                    Value::Number(serde_json::Number::from(entry.rows)),
                );
            }

            // Clean tags: remove missing:1 / missing:2 if present, ensure only relevant 'missing' is there?
            // Reconstruct tags array from our set (or cleaner logic)
            // But we want to preserve order of other tags maybe?
            // Let's filter original tags and append 'missing' if needed.
            let mut new_tags: Vec<Value> = tags
                .into_iter()
                .filter(|v| {
                    let s = v.as_str().unwrap_or("");
                    !s.starts_with("missing")
                })
                .collect();

            if !files.contains_key(&lookup_name) {
                new_tags.push(Value::String("missing".to_string()));
            }

            table_obj.insert("tags".to_string(), Value::Array(new_tags));

            // Calculate offsets and sizes
            if let Some(columns) = table_obj.get_mut("columns").and_then(|c| c.as_array_mut()) {
                let mut current_offset = 0;
                for col in columns.iter_mut() {
                    if let Some(col_obj) = col.as_object_mut() {
                        let size = get_column_size(col_obj);
                        col_obj.insert(
                            "offset".to_string(),
                            Value::Number(serde_json::Number::from(current_offset)),
                        );
                        col_obj.insert(
                            "cell_bytes".to_string(),
                            Value::Number(serde_json::Number::from(size)),
                        );
                        current_offset += size;
                    }
                }
            }

            new_tables.push(Value::Object(table_obj));
        }

        // Update tables in schema
        if let Some(obj) = schema.as_object_mut() {
            obj.insert("tables".to_string(), Value::Array(new_tables));
        }
    }

    Ok(schema)
}

fn get_column_size(col: &serde_json::Map<String, Value>) -> u32 {
    if col.get("array").and_then(|v| v.as_bool()).unwrap_or(false) {
        return 16;
    }
    if col
        .get("interval")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return 8;
    }
    let type_str = col.get("type").and_then(|v| v.as_str()).unwrap_or("");
    match type_str {
        "bool" => 1,
        "string" => 8,
        "enumrow" | "i32" | "u32" | "f32" => 4,
        "row" | "i64" | "u64" | "f64" => 8,
        "foreignrow" => 16,
        "i16" | "u16" => 2,
        _ => {
            eprintln!("Unknown type: {}", type_str);
            0
        }
    }
}

fn get_file_info(
    cache_dir: &Path,
    version: &str,
    is_poe2: bool,
    previous_cache: &HashMap<String, CachedFile>,
) -> Result<HashMap<String, CachedFile>> {
    let base_url = poe_data_tools::bundle_loader::cdn_base_url(cache_dir, version)?;
    let fs = poe_data_tools::bundle_fs::FS::from_cdn(&base_url, cache_dir)?;
    let mut files = HashMap::new();

    let mut files_to_read = Vec::new();
    let mut cached_count = 0;

    for metadata in fs.list_files() {
        let path = &metadata.path;
        let is_interesting = if is_poe2 {
            path.starts_with("data/balance/") && path.ends_with(".datc64")
        } else {
            path.starts_with("data/") && path.ends_with(".datc64")
        };

        if is_interesting {
            let name = std::path::Path::new(path)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            // Check cache
            let mut used_cache = false;
            if let Some(cached_entry) = previous_cache.get(&name) {
                if cached_entry.bundle_name == metadata.bundle_name
                    && cached_entry.offset == metadata.offset
                    && cached_entry.size == metadata.size
                {
                    files.insert(
                        name.clone(),
                        CachedFile {
                            rows: cached_entry.rows,
                            bundle_name: metadata.bundle_name.clone(),
                            offset: metadata.offset,
                            size: metadata.size,
                        },
                    );
                    used_cache = true;
                    cached_count += 1;
                }
            }

            if !used_cache {
                files_to_read.push(metadata);
            }
        }
    }

    if !files_to_read.is_empty() {
        eprintln!(
            "{} files changed or new, reading (cached {})...",
            files_to_read.len(),
            cached_count
        );
        let paths: Vec<&str> = files_to_read.iter().map(|m| m.path.as_str()).collect();

        // Process in batches
        for chunk in paths.chunks(100) {
            for res in fs.batch_read(chunk) {
                match res {
                    Ok((path, bytes)) => {
                        if bytes.len() >= 4 {
                            let rows = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
                            let name = std::path::Path::new(path)
                                .file_stem()
                                .unwrap()
                                .to_string_lossy()
                                .to_string();

                            // Find metadata for this file to cache it
                            if let Some(meta) = files_to_read.iter().find(|m| m.path == path) {
                                files.insert(
                                    name,
                                    CachedFile {
                                        rows,
                                        bundle_name: meta.bundle_name.clone(),
                                        offset: meta.offset,
                                        size: meta.size,
                                    },
                                );
                            }
                        }
                    }
                    Err((path, e)) => {
                        eprintln!("Failed to read {}: {}", path, e);
                    }
                }
            }
        }
    } else {
        eprintln!("All {} files up to date in cache.", cached_count);
    }

    eprintln!("Found {} files for version {}", files.len(), version);
    for (name, entry) in files.iter().take(10) {
        eprintln!("  {}: {}", name, entry.rows);
    }
    Ok(files)
}
