use bytes::Buf;
use std::collections::HashMap;

use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};

pub fn get_column_claims(dat_file: &DatFile, col_index: usize, cell_length: usize, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
    let cells = dat_file.column_rows(col_index, cell_length);
    if cells.is_empty() {
        return Vec::new();
    }

    match cell_length {
        1 => Vec::new(),
        2 => {
            // Zero/Constant Check using precalculated column stats
            if dat_file.stats.per_byte_stats[col_index].max_value == 0 || dat_file.stats.per_byte_stats[col_index + 1].max_value == 0 {
                return Vec::new();
            }

            let mut claims = Vec::new();

            let values: Vec<u16> = cells.iter().map(|c| c.clone().get_u16_le()).collect();
            if values.iter().any(|&x| x > 0) {
                let max_val = *values.iter().max().unwrap_or(&0);
                if max_val > 1000 {
                    let mut counts = HashMap::new();
                    for &val in &values {
                        *counts.entry(val).or_insert(0) += 1;
                    }
                    let len = values.len() as f64;
                    let entropy: f64 = counts.values().fold(0.0, |acc, &count| {
                        let p = count as f64 / len;
                        acc - p * p.log2()
                    });

                    let max_possible = if values.len() < 65536 { values.len() as f64 } else { 65536.0 };
                    if entropy > max_possible.log2() * 0.8 {
                        let mut likely_hash = true;
                        let row_count = values.len();

                        if max_val < 40000 {
                            likely_hash = false;
                        }
                        if max_val < (row_count * 2).min(60000) as u16 {
                            likely_hash = false;
                        }

                        if likely_hash && row_count > 50 {
                            let mut buckets = [0u8; 16];
                            for &v in &values {
                                buckets[(v >> 12) as usize] = 1;
                            }
                            let filled_buckets = buckets.iter().sum::<u8>();
                            if filled_buckets < 8 {
                                likely_hash = false;
                            }
                        }

                        if likely_hash {
                            let b0_stats = &dat_file.stats.per_byte_stats[col_index];
                            let b1_stats = &dat_file.stats.per_byte_stats[col_index + 1];

                            let mut lsb_odd = 0;
                            let mut msb_odd = 0;
                            let mut has_fe_pattern = false;

                            for &v in &values {
                                let b0 = (v & 0xFF) as u8;
                                let b1 = (v >> 8) as u8;
                                if !b0.is_multiple_of(2) {
                                    lsb_odd += 1;
                                }
                                if !b1.is_multiple_of(2) {
                                    msb_odd += 1;
                                }
                                if v != 0 && (b0 == 0 || b0 == 0xFE) && (b1 == 0 || b1 == 0xFE) {
                                    has_fe_pattern = true;
                                }
                            }

                            if has_fe_pattern {
                                likely_hash = false;
                            }

                            let limit = row_count / 2;
                            if b0_stats.counts.iter().any(|&c| c > limit) || b1_stats.counts.iter().any(|&c| c > limit) {
                                likely_hash = false;
                            }

                            let min_unique = (row_count / 10).clamp(3, 240).min(row_count);
                            if b0_stats.unique_count < min_unique || b1_stats.unique_count < min_unique {
                                likely_hash = false;
                            }

                            if b0_stats.unique_count == 1 && b0_stats.min_value == 0 {
                                likely_hash = false;
                            }
                            if b1_stats.unique_count == 1 && b1_stats.min_value == 0 {
                                likely_hash = false;
                            }

                            if likely_hash && row_count > 10 && (lsb_odd == 0 || lsb_odd == row_count || msb_odd == 0 || msb_odd == row_count) {
                                likely_hash = false;
                            }

                            if likely_hash {
                                claims.push(ColumnClaim {
                                    offset: col_index,
                                    bytes: 2,
                                    column_type: Cell::Scalar(Scalar::Hash16),
                                    labels: HashMap::new(),
                                });
                            }
                        }
                    }
                }
            }
            claims
        }
        4 => {
            let mut claims = Vec::new();
            if dat_file.stats.per_byte_stats[col_index].max_value == 0
                || dat_file.stats.per_byte_stats[col_index + 1].max_value == 0
                || dat_file.stats.per_byte_stats[col_index + 2].max_value == 0
                || dat_file.stats.per_byte_stats[col_index + 3].max_value == 0
            {
                return Vec::new();
            }

            let values: Vec<u32> = cells.iter().map(|c| c.clone().get_u32_le()).collect();
            if values.iter().any(|&x| x > 0) {
                let max_val = *values.iter().max().unwrap_or(&0);
                if max_val > 1_000_000 {
                    let mut counts = HashMap::new();
                    for &val in &values {
                        *counts.entry(val).or_insert(0) += 1;
                    }
                    let len = values.len() as f64;
                    let entropy: f64 = counts.values().fold(0.0, |acc, &count| {
                        let p = count as f64 / len;
                        acc - p * p.log2()
                    });

                    let max_possible = if values.len() < u32::MAX as usize {
                        values.len() as f64
                    } else {
                        u32::MAX as f64
                    };
                    if entropy > max_possible.log2() * 0.8 {
                        let mut likely_hash = true;
                        let row_count = values.len();

                        if max_val < 100_000_000 {
                            likely_hash = false;
                        }
                        if max_val < (row_count as u32 * 2).min(100_000_000) {
                            likely_hash = false;
                        }

                        if likely_hash && row_count > 50 {
                            let mut buckets = [0u8; 16];
                            for &v in &values {
                                buckets[(v >> 28) as usize] = 1;
                            }
                            let filled_buckets = buckets.iter().sum::<u8>();
                            if filled_buckets < 4 {
                                likely_hash = false;
                            }
                        }

                        if likely_hash {
                            let b0_stats = &dat_file.stats.per_byte_stats[col_index];
                            let b1_stats = &dat_file.stats.per_byte_stats[col_index + 1];
                            let b2_stats = &dat_file.stats.per_byte_stats[col_index + 2];
                            let b3_stats = &dat_file.stats.per_byte_stats[col_index + 3];

                            let mut lsb_odd = 0;
                            let mut msb_odd = 0;
                            let mut has_fe_pattern = false;

                            for &v in &values {
                                let b0 = (v & 0xFF) as u8;
                                let b1 = ((v >> 8) & 0xFF) as u8;
                                let b2 = ((v >> 16) & 0xFF) as u8;
                                let b3 = ((v >> 24) & 0xFF) as u8;

                                if !b0.is_multiple_of(2) {
                                    lsb_odd += 1;
                                }
                                if !b3.is_multiple_of(2) {
                                    msb_odd += 1;
                                }

                                if v != 0 && (b0 == 0 || b0 == 0xFE) && (b1 == 0 || b1 == 0xFE) && (b2 == 0 || b2 == 0xFE) && (b3 == 0 || b3 == 0xFE) {
                                    has_fe_pattern = true;
                                }
                            }

                            if has_fe_pattern {
                                likely_hash = false;
                            }

                            let limit = row_count / 2;
                            if b0_stats.counts.iter().any(|&c| c > limit)
                                || b1_stats.counts.iter().any(|&c| c > limit)
                                || b2_stats.counts.iter().any(|&c| c > limit)
                                || b3_stats.counts.iter().any(|&c| c > limit)
                            {
                                likely_hash = false;
                            }

                            let min_unique = (row_count / 10).clamp(3, 240).min(row_count);
                            if b0_stats.unique_count < min_unique
                                || b1_stats.unique_count < min_unique
                                || b2_stats.unique_count < min_unique
                                || b3_stats.unique_count < min_unique
                            {
                                likely_hash = false;
                            }

                            if likely_hash
                                && ((b0_stats.unique_count == 1 && b0_stats.min_value == 0)
                                    || (b1_stats.unique_count == 1 && b1_stats.min_value == 0)
                                    || (b2_stats.unique_count == 1 && b2_stats.min_value == 0)
                                    || (b3_stats.unique_count == 1 && b3_stats.min_value == 0))
                            {
                                likely_hash = false;
                            }

                            if likely_hash && row_count > 10 && (lsb_odd == 0 || lsb_odd == row_count || msb_odd == 0 || msb_odd == row_count) {
                                likely_hash = false;
                            }

                            if likely_hash {
                                claims.push(ColumnClaim {
                                    offset: col_index,
                                    bytes: 4,
                                    column_type: Cell::Scalar(Scalar::Hash32),
                                    labels: HashMap::new(),
                                });
                            }
                        }
                    }
                }
            }
            claims
        }
        8 => {
            let mut claims = Vec::new();
            let mut seen_strings = HashMap::new();
            let min_slice = dat_file.stats.per_byte_stats[col_index..col_index + 8]
                .iter()
                .map(|s| s.min_value)
                .collect::<Vec<_>>();
            let max_slice = dat_file.stats.per_byte_stats[col_index..col_index + 8]
                .iter()
                .map(|s| s.max_value)
                .collect::<Vec<_>>();

            let all_valid_strings = !cells.iter().any(|cell| {
                let offset = cell.clone().get_u64_le() as usize;
                if dat_file.valid_data_ref(offset).is_err() {
                    return true;
                }
                match dat_file.string_from_offset_if_valid(offset) {
                    Ok(s) => {
                        if s.len() < 2 && !(s.is_empty() || s == " ") {
                            return true;
                        }
                        if !s.is_ascii() {
                            return true;
                        }
                        if let Some(prev_offset) = seen_strings.get(&s) {
                            if offset != *prev_offset {
                                return true;
                            }
                        }
                        if min_slice == max_slice && !s.is_empty() {
                            return true;
                        }
                        if dat_file.stats.per_byte_stats[col_index].min_value == 0xfe && dat_file.stats.per_byte_stats[col_index].max_value == 0xfe {
                            return true;
                        }
                        if dat_file.stats.per_byte_stats[col_index].min_value == 0x00
                            && dat_file.stats.per_byte_stats[col_index].max_value == 0x00
                            && !s.is_empty()
                        {
                            return true;
                        }
                        seen_strings.insert(s.to_owned(), offset);
                    }
                    Err(_) => return true,
                }
                false
            });

            if all_valid_strings {
                let all_strings: Vec<&String> = seen_strings.keys().collect();

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
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 8,
                        column_type: Cell::Scalar(Scalar::Color),
                        labels: HashMap::new(),
                    });
                } else if let Some(files) = known_files {
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
                            claims.push(ColumnClaim {
                                offset: col_index,
                                bytes: 8,
                                column_type: Cell::Scalar(Scalar::File),
                                labels: HashMap::new(),
                            });
                        } else if is_dir {
                            claims.push(ColumnClaim {
                                offset: col_index,
                                bytes: 8,
                                column_type: Cell::Scalar(Scalar::Directory),
                                labels: HashMap::new(),
                            });
                        } else {
                            claims.push(ColumnClaim {
                                offset: col_index,
                                bytes: 8,
                                column_type: Cell::Scalar(Scalar::String),
                                labels: HashMap::new(),
                            });
                        }
                    } else {
                        claims.push(ColumnClaim {
                            offset: col_index,
                            bytes: 8,
                            column_type: Cell::Scalar(Scalar::String),
                            labels: HashMap::new(),
                        });
                    }
                } else {
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 8,
                        column_type: Cell::Scalar(Scalar::String),
                        labels: HashMap::new(),
                    });
                }
            }
            claims
        }
        16 => {
            let mut claims = Vec::new();
            let mut is_array = true;
            let mut max_count = 0;
            for cell in cells.iter().cloned() {
                let mut cell_buf = cell;
                let count = cell_buf.get_u64_le() as usize;
                let offset = cell_buf.get_u64_le() as usize;

                if count > 1000 {
                    is_array = false;
                    break;
                }
                if count > max_count {
                    max_count = count;
                }

                if dat_file.valid_data_ref(offset).is_err() {
                    is_array = false;
                    break;
                }
            }

            if is_array {
                claims.push(ColumnClaim {
                    offset: col_index,
                    bytes: 16,
                    column_type: Cell::Array(Scalar::Unknown),
                    labels: HashMap::new(),
                });
            }

            let null_val = u128::from_le_bytes([0xFE; 16]);
            let values: Vec<u128> = cells.iter().map(|c| c.clone().get_u128_le()).filter(|&v| v != null_val).collect();

            if !values.is_empty() {
                let col_max = *values.iter().max().unwrap();
                if col_max <= 100_000_000 {
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 16,
                        column_type: Cell::Scalar(Scalar::ForeignRow),
                        labels: HashMap::new(),
                    });
                }
            }
            claims
        }
        _ => Vec::new(),
    }
}

pub fn get_all_column_claims(dat_file: &DatFile, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
    let mut all_claims = Vec::new();
    for &size in &[16, 8, 4, 2] {
        if dat_file.bytes_per_row < size {
            continue;
        }
        for offset in 0..=(dat_file.bytes_per_row - size) {
            let claims = get_column_claims(dat_file, offset, size, known_files);
            all_claims.extend(claims);
        }
    }
    resolve_conflicts(dat_file, all_claims)
}

pub fn resolve_conflicts(dat_file: &DatFile, mut claims: Vec<ColumnClaim>) -> Vec<ColumnClaim> {
    fn get_score(c: &ColumnClaim) -> i32 {
        match c.column_type {
            Cell::Scalar(Scalar::File) | Cell::Scalar(Scalar::Directory) | Cell::Scalar(Scalar::Color) => 100,
            Cell::Array(_) => 90,
            Cell::Scalar(Scalar::ForeignRow) => 80,
            Cell::Scalar(Scalar::String) => 75,
            Cell::Scalar(Scalar::Hash32) => 50,
            Cell::Scalar(Scalar::Hash16) => 40,
            Cell::Scalar(Scalar::Bool) => 10,
            _ => 5,
        }
    }

    claims.sort_by(|a, b| {
        let score_a = get_score(a);
        let score_b = get_score(b);
        score_b.cmp(&score_a).then(a.offset.cmp(&b.offset)).then(b.bytes.cmp(&a.bytes))
    });

    let mut accepted = Vec::new();
    let mut occupied = vec![false; dat_file.bytes_per_row];

    for claim in claims {
        let start = claim.offset;
        let end = start + claim.bytes;
        let overlaps = occupied[start..end].iter().any(|&occupied| occupied);

        if !overlaps {
            occupied[start..end].iter_mut().for_each(|o| *o = true);
            accepted.push(claim);
        }
    }

    accepted.sort_by_key(|c| c.offset);
    accepted
}
