use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use bytes::Buf;
use std::collections::HashMap;

pub fn scan_foreign_row(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let null_val = u128::from_le_bytes([0xFE; 16]);
    let rows_iter = dat_file.column_rows_iter(col_index, 16);
    let mut has_non_null = false;
    let mut col_max = 0;

    for mut row in rows_iter {
        let v = row.get_u128_le();
        if v == null_val {
            continue;
        }
        has_non_null = true;
        if v > col_max {
            col_max = v;
        }
    }

    if has_non_null && col_max <= 100_000_000 {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 16,
            column_type: Cell::Scalar(Scalar::ForeignRow),
            labels: HashMap::new(),
        });
    }

    None
}
