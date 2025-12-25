use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use bytes::Buf;
use std::collections::HashMap;

pub fn scan_hash16(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let row_count = dat_file.table.len() / dat_file.bytes_per_row;
    if row_count == 0 {
        return None;
    }

    let mut counts = [0usize; 65536];
    let mut max_val = 0;
    let mut has_non_zero = false;
    let mut lsb_odd = 0;
    let mut msb_odd = 0;
    let mut has_fe_pattern = false;
    let mut unique_values = 0;

    let rows_iter = dat_file.column_rows_iter(col_index, 2);
    for mut row in rows_iter {
        let v = row.get_u16_le();
        if v > 0 {
            has_non_zero = true;
        }
        if v > max_val {
            max_val = v;
        }
        if counts[v as usize] == 0 {
            unique_values += 1;
        }
        counts[v as usize] += 1;

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

    if !has_non_zero || max_val <= 1000 || has_fe_pattern {
        return None;
    }
    if row_count > 10 && (lsb_odd == 0 || lsb_odd == row_count || msb_odd == 0 || msb_odd == row_count) {
        return None;
    }

    let len_f = row_count as f64;
    let mut entropy = 0.0;
    if unique_values > 1 {
        for &count in counts.iter() {
            if count > 0 {
                let p = count as f64 / len_f;
                entropy -= p * p.log2();
            }
        }
    }

    let max_possible = if row_count < 65536 { row_count as f64 } else { 65536.0 };
    if entropy <= max_possible.log2() * 0.8 {
        return None;
    }

    let mut likely_hash = true;
    if max_val < 40000 {
        likely_hash = false;
    }
    if max_val < (row_count * 2).min(60000) as u16 {
        likely_hash = false;
    }

    if likely_hash && row_count > 50 {
        let mut buckets = [0u8; 16];
        for (v, &count) in counts.iter().enumerate() {
            if count > 0 {
                buckets[v >> 12] = 1;
            }
        }
        if buckets.iter().sum::<u8>() < 8 {
            likely_hash = false;
        }
    }

    if likely_hash {
        let b0_stats = &dat_file.stats.per_byte_stats[col_index];
        let b1_stats = &dat_file.stats.per_byte_stats[col_index + 1];

        let limit = row_count / 2;
        if b0_stats.counts.iter().any(|&c| c > limit) || b1_stats.counts.iter().any(|&c| c > limit) {
            likely_hash = false;
        }

        let min_unique = (row_count / 10).clamp(3, 240).min(row_count);
        if b0_stats.unique_count < min_unique || b1_stats.unique_count < min_unique {
            likely_hash = false;
        }

        if (b0_stats.unique_count == 1 && b0_stats.min_value == 0) || (b1_stats.unique_count == 1 && b1_stats.min_value == 0) {
            likely_hash = false;
        }
    }

    if likely_hash {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 2,
            column_type: Cell::Scalar(Scalar::Hash16),
            labels: HashMap::new(),
        });
    }

    None
}

pub fn scan_hash32(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let row_count = dat_file.table.len() / dat_file.bytes_per_row;
    if row_count == 0 {
        return None;
    }

    let mut counts = HashMap::new();
    let mut max_val = 0;
    let mut has_non_zero = false;
    let mut lsb_odd = 0;
    let mut msb_odd = 0;
    let mut has_fe_pattern = false;

    let rows_iter = dat_file.column_rows_iter(col_index, 4);
    for mut row in rows_iter {
        let v = row.get_u32_le();
        if v > 0 {
            has_non_zero = true;
        }
        if v > max_val {
            max_val = v;
        }
        *counts.entry(v).or_insert(0) += 1;

        let b0 = (v & 0xFF) as u8;
        let b3 = (v >> 24) as u8;
        if !b0.is_multiple_of(2) {
            lsb_odd += 1;
        }
        if !b3.is_multiple_of(2) {
            msb_odd += 1;
        }

        let b1 = ((v >> 8) & 0xFF) as u8;
        let b2 = ((v >> 16) & 0xFF) as u8;
        if v != 0 && (b0 == 0 || b0 == 0xFE) && (b1 == 0 || b1 == 0xFE) && (b2 == 0 || b2 == 0xFE) && (b3 == 0 || b3 == 0xFE) {
            has_fe_pattern = true;
        }
    }

    if !has_non_zero || max_val <= 1_000_000 || has_fe_pattern {
        return None;
    }
    if row_count > 10 && (lsb_odd == 0 || lsb_odd == row_count || msb_odd == 0 || msb_odd == row_count) {
        return None;
    }

    let len_f = row_count as f64;
    let mut entropy = 0.0;
    for &count in counts.values() {
        let p = count as f64 / len_f;
        entropy -= p * p.log2();
    }

    let max_possible = row_count as f64;
    if entropy <= max_possible.log2() * 0.8 {
        return None;
    }

    let mut likely_hash = true;
    if max_val < 100_000_000 {
        likely_hash = false;
    }
    if max_val < (row_count as u32 * 2).min(100_000_000) {
        likely_hash = false;
    }

    if likely_hash && row_count > 50 {
        let mut buckets = [0u8; 16];
        for &v in counts.keys() {
            buckets[(v >> 28) as usize] = 1;
        }
        if buckets.iter().sum::<u8>() < 4 {
            likely_hash = false;
        }
    }

    if likely_hash {
        let b0_stats = &dat_file.stats.per_byte_stats[col_index];
        let b1_stats = &dat_file.stats.per_byte_stats[col_index + 1];
        let b2_stats = &dat_file.stats.per_byte_stats[col_index + 2];
        let b3_stats = &dat_file.stats.per_byte_stats[col_index + 3];

        let limit = row_count / 2;
        if b0_stats.counts.iter().any(|&c| c > limit)
            || b1_stats.counts.iter().any(|&c| c > limit)
            || b2_stats.counts.iter().any(|&c| c > limit)
            || b3_stats.counts.iter().any(|&c| c > limit)
        {
            likely_hash = false;
        }

        let min_unique = (row_count / 10).clamp(3, 240).min(row_count);
        if b0_stats.unique_count < min_unique || b1_stats.unique_count < min_unique || b2_stats.unique_count < min_unique || b3_stats.unique_count < min_unique
        {
            likely_hash = false;
        }

        if (b0_stats.unique_count == 1 && b0_stats.min_value == 0)
            || (b1_stats.unique_count == 1 && b1_stats.min_value == 0)
            || (b2_stats.unique_count == 1 && b2_stats.min_value == 0)
            || (b3_stats.unique_count == 1 && b3_stats.min_value == 0)
        {
            likely_hash = false;
        }
    }

    if likely_hash {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 4,
            column_type: Cell::Scalar(Scalar::Hash32),
            labels: HashMap::new(),
        });
    }

    None
}
