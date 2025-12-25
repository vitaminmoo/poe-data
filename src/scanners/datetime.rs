use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use bytes::Buf;
use std::collections::HashMap;

pub fn scan(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let rows_iter = dat_file.column_rows_iter(col_index, 8);
    let mut valid_count = 0;
    let mut date_count = 0;

    for mut row in rows_iter {
        let val = row.get_u64_le();
        if val == 0 || val == 0xFEFEFEFEFEFEFEFE {
            continue;
        }
        valid_count += 1;

        // Unix seconds: 2010 to 2030
        if (1_262_304_000..1_893_456_000).contains(&val) {
            date_count += 1;
            continue;
        }

        // Unix milliseconds: 2010 to 2030
        if (1_262_304_000_000..1_893_456_000_000).contains(&val) {
            date_count += 1;
            continue;
        }

        // Windows FILETIME: 2010 to 2030 (100ns units since 1601)
        if (129_067_200_000_000_000..135_392_160_000_000_000).contains(&val) {
            date_count += 1;
            continue;
        }

        // If we found a non-zero, non-null value that doesn't fit any range, it's not a DateTime column
        return None;
    }

    // Only claim if we saw at least one valid date and no invalid ones
    if valid_count > 0 && date_count == valid_count {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::DateTime),
            labels: HashMap::new(),
        });
    }

    None
}
