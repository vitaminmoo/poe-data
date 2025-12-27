use serde_json::Value;
use std::fs;
use std::path::PathBuf;

use poe_data::dat_parser::DatLoader;
use poe_data::types::Scalar;

#[test]
fn test_validation_does_not_reject_valid_types() {
    let schema_path = PathBuf::from("lib/schema.2.min.json");
    if !schema_path.exists() {
        eprintln!(
            "Schema not found at {:?}, skipping test. Run 'cargo run --bin getschema -- lib' to generate it.",
            schema_path
        );
        return;
    }

    let schema_json: Value = serde_json::from_str(&fs::read_to_string(&schema_path).unwrap()).unwrap();
    let tables = schema_json["tables"].as_array().unwrap();

    let target_tables = vec!["Mods", "UltimatumModifiers", "AlternatePassiveAdditions", "Stats", "PassiveSkills"];

    let mut loader = DatLoader::default();
    let file_list = loader.get_file_list();
    let mut sorted_files = file_list.clone();
    sorted_files.iter_mut().for_each(|f| *f = f.to_lowercase());
    sorted_files.sort();

    for table_name in target_tables {
        println!("Testing table: {}", table_name);
        let table_schema = tables
            .iter()
            .find(|t| t["name"].as_str() == Some(table_name))
            .expect(&format!("Table schema for {} not found", table_name));

        let file_path = format!("data/balance/{}.datc64", table_name);

        if loader.get_table(&file_path).is_none() {
            println!("Could not load table file {}, skipping", file_path);
            continue;
        }
        let dat_file = loader.get_table(&file_path).unwrap();

        // Run validation to get what types are ALLOWED (not rejected)
        let validations = dat_file.validate_types(Some(&sorted_files));

        let columns_schema = table_schema["columns"].as_array().unwrap();

        for col_schema in columns_schema {
            let offset = col_schema["offset"].as_u64().unwrap() as usize;
            let type_str = col_schema["type"].as_str().unwrap_or("");
            let is_array_schema = col_schema.get("array").and_then(|v| v.as_bool()).unwrap_or(false);

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
                        println!("Unknown type {} at offset {}, skipping", type_str, offset);
                        continue;
                    }
                }
            };

            // Find the validation entry for this offset AND length
            let validation = validations.iter().find(|v| v.offset == offset && v.length == expected_len);

            if let Some(validation) = validation {
                if is_array_schema {
                    assert!(
                        validation.is_array,
                        "{}: Offset {}: Schema says Array, but validation rejected it.",
                        table_name, offset
                    );
                } else {
                    match type_str {
                        "string" => {
                            let has_string = validation.allowed_types.contains(&Scalar::String)
                                || validation.allowed_types.contains(&Scalar::File)
                                || validation.allowed_types.contains(&Scalar::Directory)
                                || validation.allowed_types.contains(&Scalar::Color);

                            assert!(
                                has_string,
                                "{}: Offset {}: Schema says String, but validation rejected it. Allowed: {:?}",
                                table_name, offset, validation.allowed_types
                            );
                        }
                        "file" => {
                            let allowed = validation.allowed_types.contains(&Scalar::File);
                            assert!(
                                allowed,
                                "{}: Offset {}: Schema says File, but validation rejected it. Allowed: {:?}",
                                table_name, offset, validation.allowed_types
                            );
                        }
                        "directory" => {
                            let allowed = validation.allowed_types.contains(&Scalar::Directory);
                            assert!(
                                allowed,
                                "{}: Offset {}: Schema says Directory, but validation rejected it. Allowed: {:?}",
                                table_name, offset, validation.allowed_types
                            );
                        }
                        "bool" => {
                            assert!(
                                validation.allowed_types.contains(&Scalar::Bool),
                                "{}: Offset {}: Schema says Bool, but validation rejected it. Allowed: {:?}",
                                table_name,
                                offset,
                                validation.allowed_types
                            );
                        }
                        _ => {
                            // We don't have validators for other types yet (like interval being strictly checked), so we pass.
                        }
                    }
                }
            } else {
                // If we didn't get a validation entry, it might be because the column size wasn't one of the standard ones we check (1,2,4,8,16).
                // But for standard types, it should be there.
                if is_array_schema {
                    // Arrays are 16 bytes.
                    panic!(
                        "{}: Offset {}: Schema says Array (16 bytes), but no validation entry found.",
                        table_name, offset
                    );
                }
                match type_str {
                    "string" => panic!(
                        "{}: Offset {}: Schema says String (8 bytes), but no validation entry found.",
                        table_name, offset
                    ),
                    "bool" => panic!("{}: Offset {}: Schema says Bool (1 byte), but no validation entry found.", table_name, offset),
                    _ => {}
                }
            }
        }
    }
}
