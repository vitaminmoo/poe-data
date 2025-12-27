use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use bytes::Buf;
use std::collections::HashMap;

pub fn scan_foreign_row(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let null_val = u128::from_le_bytes([0xFE; 16]);
    let rows_iter = dat_file.column_rows_iter(col_index, 16);
    let mut col_max = 0;

    for mut row in rows_iter {
        let v = row.get_u128_le();
        if v == null_val {
            continue;
        }
        if v > col_max {
            col_max = v;
        }
    }

    if col_max < 100_000 {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 16,
            column_type: Cell::Scalar(Scalar::ForeignRow),
            labels: HashMap::new(),
        });
    }

    None
}

pub fn scan_row(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let null_val = 0xFEFEFEFEFEFEFEFE_u64;
    let rows_iter = dat_file.column_rows_iter(col_index, 8);
    let mut has_non_null = false;

    for mut row in rows_iter {
        let v = row.get_u64_le();
        if v == null_val {
            continue;
        }
        if v >= dat_file.row_count as u64 {
            return None;
        }
        has_non_null = true;
    }

    if has_non_null {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::SelfRow),
            labels: HashMap::new(),
        });
    }

    None
}
