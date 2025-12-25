use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use bytes::Buf;
use std::collections::HashMap;

pub fn scan(dat_file: &DatFile, col_index: usize, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
    let rows_iter = dat_file.column_rows_iter(col_index, 8);
    let mut seen_strings = HashMap::new();
    let mut all_valid_strings = true;
    let mut has_any_string = false;

    for mut row in rows_iter {
        let offset = row.get_u64_le() as usize;
        if offset == 0xFEFEFEFEFEFEFEFE {
            continue;
        }
        has_any_string = true;

        // Basic pointer validity is already checked by phase 1 absolute rejection in DatFile::get_column_claims if called through it,
        // but here we are doing a more detailed check.
        match dat_file.string_from_offset_if_valid(offset) {
            Ok(s) => {
                if s.len() < 2 && !(s.is_empty() || s == " ") {
                    all_valid_strings = false;
                    break;
                }
                if !s.is_ascii() {
                    all_valid_strings = false;
                    break;
                }
                if let Some(prev_offset) = seen_strings.get(&s) {
                    if offset != *prev_offset {
                        all_valid_strings = false;
                        break;
                    }
                }
                seen_strings.insert(s.to_owned(), offset);
            }
            Err(_) => {
                all_valid_strings = false;
                break;
            }
        }
    }

    if !all_valid_strings || !has_any_string {
        return Vec::new();
    }

    let all_strings: Vec<&String> = seen_strings.keys().collect();

    // Check for Color
    let mut is_color = true;
    for s in &all_strings {
        if s.is_empty() {
            continue;
        }
        let is_hex_code = (s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 9 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 8 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()))
            || (s.len() == 10 && s.starts_with("0x") && s[2..].chars().all(|c| c.is_ascii_hexdigit()));
        if !is_hex_code {
            is_color = false;
            break;
        }
    }

    if is_color && !all_strings.is_empty() && all_strings.iter().any(|s| !s.is_empty()) {
        return vec![ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::Color),
            labels: HashMap::new(),
        }];
    }

    // Check for File/Directory
    if let Some(files) = known_files {
        let mut is_file = true;
        let mut is_dir = true;
        let mut has_non_empty = false;

        for s in &all_strings {
            if s.is_empty() {
                continue;
            }
            has_non_empty = true;
            let s_lower = s.to_lowercase();
            if is_file && files.binary_search(&s_lower).is_err() {
                is_file = false;
            }
            if is_dir {
                let idx = files.partition_point(|f| f.as_str() < s_lower.as_str());
                if idx >= files.len() || !files[idx].starts_with(s_lower.as_str()) {
                    is_dir = false;
                }
            }
            if !is_file && !is_dir {
                break;
            }
        }

        if has_non_empty {
            if is_file {
                return vec![ColumnClaim {
                    offset: col_index,
                    bytes: 8,
                    column_type: Cell::Scalar(Scalar::File),
                    labels: HashMap::new(),
                }];
            } else if is_dir {
                return vec![ColumnClaim {
                    offset: col_index,
                    bytes: 8,
                    column_type: Cell::Scalar(Scalar::Directory),
                    labels: HashMap::new(),
                }];
            }
        }
    }

    // Default to String
    vec![ColumnClaim {
        offset: col_index,
        bytes: 8,
        column_type: Cell::Scalar(Scalar::String),
        labels: HashMap::new(),
    }]
}
