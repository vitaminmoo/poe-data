use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use bytes::Buf;
use std::collections::HashMap;

pub fn scan(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let rows_iter = dat_file.column_rows_iter(col_index, 8);
    let mut valid_count = 0;
    let mut has_non_zero = false;

    for mut row in rows_iter {
        let val = row.get_u64_le();
        if val == 0xFEFEFEFEFEFEFEFE {
            continue;
        }

        let min = val as u32 as i32;
        let max = (val >> 32) as u32 as i32;

        if min > max {
            return None;
        }

        if val != 0 {
            has_non_zero = true;
        }
        valid_count += 1;
    }

    if valid_count > 0 && has_non_zero {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::Interval),
            labels: HashMap::new(),
        });
    }

    None
}
