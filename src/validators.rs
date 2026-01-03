use crate::dat_parser::DatFile;
use crate::types::ValidationError;
use bytes::Buf;
use std::collections::HashMap;

fn format_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ")
}

/// Checks if a column is a valid boolean (0 or 1).
pub fn is_valid_bool(dat_file: &DatFile, col_index: usize) -> Result<(), ValidationError> {
    // Optimization: check pre-computed stats
    if dat_file.stats.per_byte_stats[col_index].max_value <= 1 {
        return Ok(());
    }

    for (i, mut row) in dat_file.column_rows_iter(col_index, 1).enumerate() {
        let val = row.get_u8();
        if val > 1 {
            return Err(ValidationError {
                check: "Bool(<=1)".to_string(),
                row: i,
                value_hex: format!("{:02X}", val),
            });
        }
    }
    Ok(())
}

/// Checks if a column contains valid string/file/path pointers.
pub fn is_valid_string_pointer(dat_file: &DatFile, col_index: usize) -> Result<(), ValidationError> {
    let mut seen_strings: HashMap<String, usize> = HashMap::new();

    for (i, mut row) in dat_file.column_rows_iter(col_index, 8).enumerate() {
        let raw_bytes = row.chunk(); // peek bytes for hex dump if needed
        let raw_hex = if raw_bytes.len() >= 8 {
            format_hex(&raw_bytes[..8])
        } else {
            format_hex(raw_bytes)
        };

        let offset = row.get_u64_le() as usize;
        // Check 1: Valid Reference
        if let Err(e) = dat_file.valid_data_ref(offset) {
            return Err(ValidationError {
                check: format!("String Pointer(valid ref): {}", e),
                row: i,
                value_hex: raw_hex, // format!("{:016X}", offset)
            });
        }
        // Check 2: Valid String Content
        let s = match dat_file.string_from_offset_if_valid(offset) {
            Ok(s) => s,
            Err(e) => {
                return Err(ValidationError {
                    check: format!("String Content: {}", e),
                    row: i,
                    value_hex: raw_hex,
                })
            }
        };

        // Check 3: Deduplication Consistency
        if let Some(&prev_offset) = seen_strings.get(&s) {
            if prev_offset != offset {
                return Err(ValidationError {
                    check: "String Deduplication".to_string(),
                    row: i,
                    value_hex: raw_hex,
                });
            }
        } else {
            seen_strings.insert(s, offset);
        }
    }
    Ok(())
}

/// Checks if a column structure matches the Array layout [Count, Offset].
pub fn is_valid_array(dat_file: &DatFile, col_index: usize) -> Result<(), ValidationError> {
    let mut seen_offsets: HashMap<usize, u64> = HashMap::new(); // Offset -> Count

    for (i, mut row) in dat_file.column_rows_iter(col_index, 16).enumerate() {
        let raw_bytes = row.chunk();
        let raw_hex = if raw_bytes.len() >= 16 {
            format_hex(&raw_bytes[..16])
        } else {
            format_hex(raw_bytes)
        };

        let count = row.get_u64_le();
        let offset = row.get_u64_le() as usize;

        // Arbitrary limit based on what's been seen in real data files.
        if count > 1000 {
            return Err(ValidationError {
                check: "Array Count(>1000)".to_string(),
                row: i,
                value_hex: raw_hex,
            });
        }

        if count > 0 {
            // Offset must point to valid vdata region if count > 0
            if dat_file.valid_data_ref(offset).is_err() {
                return Err(ValidationError {
                    check: "Array Pointer(invalid)".to_string(),
                    row: i,
                    value_hex: raw_hex,
                });
            }

            // Check Deduplication Consistency
            // If we see the same offset again, it must have the same count.
            if let Some(&prev_count) = seen_offsets.get(&offset) {
                if prev_count != count {
                    return Err(ValidationError {
                        check: "Array Deduplication".to_string(),
                        row: i,
                        value_hex: raw_hex,
                    });
                }
            } else {
                seen_offsets.insert(offset, count);
            }
        } else if offset != 0 && dat_file.valid_data_ref(offset).is_err() {
            return Err(ValidationError {
                check: "Array Empty Pointer(invalid)".to_string(),
                row: i,
                value_hex: raw_hex,
            });
        }
    }
    Ok(())
}

pub fn is_valid_file_path(dat_file: &DatFile, col_index: usize, known_files: Option<&[String]>) -> Result<(), ValidationError> {
    is_valid_string_pointer(dat_file, col_index)?;

    let files = match known_files {
        Some(f) => f,
        None => return Ok(()),
    };

    for (i, mut row) in dat_file.column_rows_iter(col_index, 8).enumerate() {
        let offset = row.get_u64_le() as usize;
        // Should be valid if is_valid_string_pointer passed
        let s = dat_file.string_from_offset_if_valid(offset).unwrap();

        if s.is_empty() {
            continue;
        }

        let s_lower = s.to_lowercase();
        if list_binary_search(files, &s_lower).is_err() {
            return Err(ValidationError {
                check: "File Path(not found)".to_string(),
                row: i,
                value_hex: format_hex(&offset.to_le_bytes()), // approximates hex
            });
        }
    }
    Ok(())
}

pub fn is_valid_directory_path(dat_file: &DatFile, col_index: usize, known_files: Option<&[String]>) -> Result<(), ValidationError> {
    is_valid_string_pointer(dat_file, col_index)?;

    let files = match known_files {
        Some(f) => f,
        None => return Ok(()),
    };

    for (i, mut row) in dat_file.column_rows_iter(col_index, 8).enumerate() {
        let offset = row.get_u64_le() as usize;
        let s = dat_file.string_from_offset_if_valid(offset).unwrap();

        if s.is_empty() {
            continue;
        }

        let s_lower = s.to_lowercase();

        // Check if s_lower is a prefix of any file in the sorted list.
        let idx = files.partition_point(|x| x.as_str() < s_lower.as_str());

        match files.get(idx) {
            Some(f) => {
                if !f.starts_with(&s_lower) {
                    return Err(ValidationError {
                        check: "Directory Path(not found)".to_string(),
                        row: i,
                        value_hex: format_hex(&offset.to_le_bytes()),
                    });
                }
            }
            None => {
                return Err(ValidationError {
                    check: "Directory Path(not found)".to_string(),
                    row: i,
                    value_hex: format_hex(&offset.to_le_bytes()),
                })
            }
        }
    }
    Ok(())
}

pub fn is_valid_color(dat_file: &DatFile, col_index: usize) -> Result<(), ValidationError> {
    is_valid_string_pointer(dat_file, col_index)?;

    let mut has_non_empty = false;
    for (i, mut row) in dat_file.column_rows_iter(col_index, 8).enumerate() {
        let offset = row.get_u64_le() as usize;
        let s = dat_file.string_from_offset_if_valid(offset).unwrap();

        if s.is_empty() {
            continue;
        }
        has_non_empty = true;

        let is_hex_code = (s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 9 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 8 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 10 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()));

        if !is_hex_code {
            return Err(ValidationError {
                check: "Color(format)".to_string(),
                row: i,
                value_hex: format_hex(&offset.to_le_bytes()),
            });
        }
    }
    if !has_non_empty {
        // A color column must have at least one color, otherwise it's just empty strings (ambiguous)
        // But maybe we return OK and let conflict resolution handle it?
        // Original returned false.
        return Err(ValidationError {
            check: "Color(all empty)".to_string(),
            row: 0,
            value_hex: "".to_string(),
        });
    }
    Ok(())
}

fn list_binary_search(list: &[String], target: &str) -> Result<usize, usize> {
    list.binary_search_by(|probe| probe.as_str().cmp(target))
}

pub fn is_valid_hash16(dat_file: &DatFile, col_index: usize) -> Result<(), ValidationError> {
    for (i, mut row) in dat_file.column_rows_iter(col_index, 2).enumerate() {
        let val = row.get_u16_le();
        if val == 0 || val == 0xFEFE {
            return Err(ValidationError {
                check: "Hash16(invalid)".to_string(),
                row: i,
                value_hex: format!("{:04X}", val),
            });
        }
    }
    Ok(())
}

pub fn is_valid_hash32(dat_file: &DatFile, col_index: usize) -> Result<(), ValidationError> {
    for (i, mut row) in dat_file.column_rows_iter(col_index, 4).enumerate() {
        let val = row.get_u32_le();
        if val == 0 || val == 0xFEFEFEFE {
            return Err(ValidationError {
                check: "Hash32(invalid)".to_string(),
                row: i,
                value_hex: format!("{:08X}", val),
            });
        }
    }
    Ok(())
}
