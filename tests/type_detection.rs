use serde_json::Value;
use std::fs;
use std::path::PathBuf;

use poe_data::dat_parser::DatLoader;
use poe_data::types::Scalar;
use poe_data::versions::get_versions;

#[test]
fn test_validation_all_tables() {
    let versions = get_versions().expect("Failed to get versions");

    println!("Testing PoE 1: {}", versions.poe);
    run_test_for_version(&versions.poe, "lib/schema.1.min.json", false);

    println!("Testing PoE 2: {}", versions.poe2);
    run_test_for_version(&versions.poe2, "lib/schema.2.min.json", true);
}

fn run_test_for_version(version: &str, schema_path_str: &str, is_poe2: bool) {
    let schema_path = PathBuf::from(schema_path_str);
    if !schema_path.exists() {
        eprintln!(
            "Schema not found at {:?}, skipping test. Run 'cargo run --bin getschema -- lib' to generate it.",
            schema_path
        );
        return;
    }

    let schema_json: Value = serde_json::from_str(&fs::read_to_string(&schema_path).unwrap()).unwrap();
    let tables = schema_json["tables"].as_array().unwrap();

    let mut loader = DatLoader::new(version).expect("Failed to create loader");
    let file_list = loader.get_file_list();
    let mut sorted_files = file_list.clone();
    sorted_files.iter_mut().for_each(|f| *f = f.to_lowercase());
    sorted_files.sort();

    let prefix = if is_poe2 { "data/balance/" } else { "data/" };

    let mut errors = Vec::new();

    for table_schema in tables {
        let table_name = table_schema["name"].as_str().unwrap();

        // Skip tables marked as missing
        if let Some(tags) = table_schema.get("tags").and_then(|t| t.as_array()) {
            if tags.iter().any(|t| t.as_str() == Some("missing")) {
                continue;
            }
        }

        // Try to find the file
        let expected_path_end = format!("{}.datc64", table_name);
        let expected_path_full = format!("{}{}", prefix, expected_path_end);

        // Find matching file in file_list (case-insensitive check for robustness, though usually it matches prefix + name)
        let matching_file = file_list.iter().find(|f| {
            // Check if f ends with table_name.datc64 (case insensitive) AND starts with prefix
            // Actually, strict path construction is better if possible, but let's be flexible
            f.eq_ignore_ascii_case(&expected_path_full)
        });

        if matching_file.is_none() {
            // println!("Table {} file not found (expected {}), skipping", table_name, expected_path_full);
            continue;
        }
        let file_path = matching_file.unwrap();

        if loader.get_table(file_path).is_none() {
            println!("Could not load table file {}, skipping", file_path);
            continue;
        }
        let dat_file = loader.get_table(file_path).unwrap();

        // Run validation to get what types are ALLOWED (not rejected)
        let validations = dat_file.validate_types(Some(&sorted_files));

        let columns_schema = table_schema["columns"].as_array().unwrap();

        for col_schema in columns_schema {
            let offset = col_schema["offset"].as_u64().unwrap() as usize;
            let type_str = col_schema["type"].as_str().unwrap_or("");
            let is_array_schema = col_schema.get("array").and_then(|v| v.as_bool()).unwrap_or(false);

            // Skip columns with errors in schema (often OOB)
            if let Some(errors) = col_schema.get("errors").and_then(|e| e.as_array()) {
                if !errors.is_empty() {
                    continue;
                }
            }

            let expected_len = if is_array_schema {
                16
            } else {
                match type_str {
                    "bool" | "byte" | "ubyte" => 1,
                    "short" | "ushort" => 2,
                    "int" | "uint" | "float" | "enumrow" => 4,
                    "long" | "ulong" | "double" | "string" | "file" | "directory" | "color" | "ref" | "row" | "interval" | "datetime" => 8,
                    "foreignrow" => 16,
                    _ => {
                        //println!("Unknown type {} at offset {}, skipping", type_str, offset);
                        continue;
                    }
                }
            };

            // Find the validation entry for this offset AND length
            let validation = validations.iter().find(|v| v.offset == offset && v.length == expected_len);

            if let Some(validation) = validation {
                if is_array_schema {
                    if !validation.is_array {
                        // Check if it's a known schema error or truly invalid
                        // For now, report as Schema Mismatch
                        println!(
                            "SCHEMA MISMATCH: {}: Offset {}: Schema says Array, but data is NOT a valid array.",
                            table_name, offset
                        );
                    }
                } else {
                    match type_str {
                        "string" => {
                            let has_string = validation.allowed_types.contains(&Scalar::String)
                                || validation.allowed_types.contains(&Scalar::File)
                                || validation.allowed_types.contains(&Scalar::Directory)
                                || validation.allowed_types.contains(&Scalar::Color);

                            if !has_string {
                                let details = validation
                                    .failures
                                    .iter()
                                    .find(|(s, _)| *s == Scalar::String)
                                    .map(|(_, e)| format!("{} (Row {}, Val: {})", e.check, e.row, e.value_hex))
                                    .unwrap_or_else(|| "Unknown rejection".to_string());

                                if !validation.allowed_types.is_empty() {
                                    println!(
                                        "SCHEMA MISMATCH: {}: Offset {}: Schema says String, but data is valid as {:?}. Reason: {}",
                                        table_name, offset, validation.allowed_types, details
                                    );
                                } else {
                                    errors.push(format!(
                                        "{}: Offset {}: Schema says String, but validation rejected it entirely. Reason: {}",
                                        table_name, offset, details
                                    ));
                                }
                            }
                        }
                        "file" => {
                            if !validation.allowed_types.contains(&Scalar::File) {
                                let details = validation
                                    .failures
                                    .iter()
                                    .find(|(s, _)| *s == Scalar::File)
                                    .map(|(_, e)| format!("{} (Row {}, Val: {})", e.check, e.row, e.value_hex))
                                    .unwrap_or_else(|| "Unknown rejection".to_string());
                                println!(
                                    "SCHEMA MISMATCH: {}: Offset {}: Schema says File, but data is not a valid File path. Reason: {}",
                                    table_name, offset, details
                                );
                            }
                        }
                        "directory" => {
                            if !validation.allowed_types.contains(&Scalar::Directory) {
                                let details = validation
                                    .failures
                                    .iter()
                                    .find(|(s, _)| *s == Scalar::Directory)
                                    .map(|(_, e)| format!("{} (Row {}, Val: {})", e.check, e.row, e.value_hex))
                                    .unwrap_or_else(|| "Unknown rejection".to_string());
                                println!(
                                    "SCHEMA MISMATCH: {}: Offset {}: Schema says Directory, but data is not a valid Directory path. Reason: {}",
                                    table_name, offset, details
                                );
                            }
                        }
                        "bool" => {
                            if !validation.allowed_types.contains(&Scalar::Bool) {
                                let details = validation
                                    .failures
                                    .iter()
                                    .find(|(s, _)| *s == Scalar::Bool)
                                    .map(|(_, e)| format!("{} (Row {}, Val: {})", e.check, e.row, e.value_hex))
                                    .unwrap_or_else(|| "Unknown rejection".to_string());
                                println!(
                                    "SCHEMA MISMATCH: {}: Offset {}: Schema says Bool, but data is NOT a valid bool. Reason: {}",
                                    table_name, offset, details
                                );
                            }
                        }
                        _ => {}
                    }
                }
            } else {
                if dat_file.row_count == 0 {
                    continue;
                }
                // If no validation entry at all for a standard size, that's a validator issue
                errors.push(format!(
                    "{}: Offset {}: No validation entry found for expected length {}",
                    table_name, offset, expected_len
                ));
            }
        }
    }

    if !errors.is_empty() {
        for err in &errors {
            println!("VALIDATION BUG: {}", err);
        }
        panic!("Validation failed for {} columns due to suspected bugs", errors.len());
    }
}
