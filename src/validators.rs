use crate::dat_parser::DatFile;
use bytes::Buf;
use std::collections::HashMap;

/// Checks if a column is a valid boolean (0 or 1).
pub fn is_valid_bool(dat_file: &DatFile, col_index: usize) -> bool {
    // Optimization: check pre-computed stats
    dat_file.stats.per_byte_stats[col_index].max_value <= 1
}

/// Checks if a column contains valid string/file/path pointers.
pub fn is_valid_string_pointer(dat_file: &DatFile, col_index: usize) -> bool {
    let mut seen_strings: HashMap<String, usize> = HashMap::new();

    for mut row in dat_file.column_rows_iter(col_index, 8) {
        let offset = row.get_u64_le() as usize;
        // Check 1: Valid Reference
        if dat_file.valid_data_ref(offset).is_err() {
            return false;
        }
        // Check 2: Valid String Content
        let s = match dat_file.string_from_offset_if_valid(offset) {
            Ok(s) => s,
            Err(_) => return false,
        };

        // Check 3: Deduplication Consistency
        if let Some(&prev_offset) = seen_strings.get(&s) {
            if prev_offset != offset {
                return false;
            }
        } else {
            seen_strings.insert(s, offset);
        }
    }
    true
}

/// Checks if a column structure matches the Array layout [Count, Offset].
pub fn is_valid_array(dat_file: &DatFile, col_index: usize) -> bool {
    let mut seen_offsets: HashMap<usize, u64> = HashMap::new(); // Offset -> Count

    for mut row in dat_file.column_rows_iter(col_index, 16) {
        let count = row.get_u64_le();
        let offset = row.get_u64_le() as usize;

        // Arbitrary limit based on what's been seen in real data files.
        if count > 50 {
            return false;
        }

        if count > 0 {
            // Offset must point to valid vdata region if count > 0
            if dat_file.valid_data_ref(offset).is_err() {
                return false;
            }

            // Check Deduplication Consistency
            // If we see the same offset again, it must have the same count.
            if let Some(&prev_count) = seen_offsets.get(&offset) {
                if prev_count != count {
                    return false;
                }
            } else {
                seen_offsets.insert(offset, count);
            }
        } else if offset != 0 && dat_file.valid_data_ref(offset).is_err() {
            return false;
        }
    }
    true
}

pub fn is_valid_file_path(dat_file: &DatFile, col_index: usize, known_files: Option<&[String]>) -> bool {
    if !is_valid_string_pointer(dat_file, col_index) {
        return false;
    }

    let files = match known_files {
        Some(f) => f,
        None => return true,
    };

    for mut row in dat_file.column_rows_iter(col_index, 8) {
        let offset = row.get_u64_le() as usize;
        let s = match dat_file.string_from_offset_if_valid(offset) {
            Ok(s) => s,
            Err(_) => return false,
        };

        if s.is_empty() {
            continue;
        }

        let s_lower = s.to_lowercase();
        if list_binary_search(files, &s_lower).is_err() {
            return false;
        }
    }
    true
}

pub fn is_valid_directory_path(dat_file: &DatFile, col_index: usize, known_files: Option<&[String]>) -> bool {
    if !is_valid_string_pointer(dat_file, col_index) {
        return false;
    }

    let files = match known_files {
        Some(f) => f,
        None => return true,
    };

    for mut row in dat_file.column_rows_iter(col_index, 8) {
        let offset = row.get_u64_le() as usize;
        let s = match dat_file.string_from_offset_if_valid(offset) {
            Ok(s) => s,
            Err(_) => return false,
        };

        if s.is_empty() {
            continue;
        }

        let s_lower = s.to_lowercase();

        // Check if s_lower is a prefix of any file in the sorted list.
        let idx = files.partition_point(|x| x.as_str() < s_lower.as_str());

        match files.get(idx) {
            Some(f) => {
                if !f.starts_with(&s_lower) {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

pub fn is_valid_color(dat_file: &DatFile, col_index: usize) -> bool {
    if !is_valid_string_pointer(dat_file, col_index) {
        return false;
    }

    let mut has_non_empty = false;
    for mut row in dat_file.column_rows_iter(col_index, 8) {
        let offset = row.get_u64_le() as usize;
        let s = match dat_file.string_from_offset_if_valid(offset) {
            Ok(s) => s,
            Err(_) => return false,
        };

        if s.is_empty() {
            continue;
        }
        has_non_empty = true;

        let is_hex_code = (s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 9 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 8 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 10 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()));

        if !is_hex_code {
            return false;
        }
    }
    has_non_empty
}

fn list_binary_search(list: &[String], target: &str) -> Result<usize, usize> {
    list.binary_search_by(|probe| probe.as_str().cmp(target))
}

pub fn is_valid_hash16(dat_file: &DatFile, col_index: usize) -> bool {
    for mut row in dat_file.column_rows_iter(col_index, 2) {
        let val = row.get_u16_le();
        if val == 0 || val == 0xFEFE {
            return false;
        }
    }
    true
}

pub fn is_valid_hash32(dat_file: &DatFile, col_index: usize) -> bool {
    for mut row in dat_file.column_rows_iter(col_index, 4) {
        let val = row.get_u32_le();
        if val == 0 || val == 0xFEFEFEFE {
            return false;
        }
    }
    true
}
