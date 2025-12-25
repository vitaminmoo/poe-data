use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use bytes::Buf;
use std::collections::HashMap;

pub fn scan(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let rows_iter = dat_file.column_rows_iter(col_index, 16);
    let vdata_len = dat_file.vdata.len();
    let mut has_any = false;

    for mut row in rows_iter {
        let count = row.get_u64_le();
        let offset = row.get_u64_le();

        // Check for Null ForeignRow pattern
        if count == 0xFEFEFEFEFEFEFEFE && offset == 0xFEFEFEFEFEFEFEFE {
            return None;
        }

        // Array Validity Checks
        if count > 100_000 {
            return None;
        }

        if count > 0 {
            if offset < 8 || offset as usize > vdata_len {
                return None;
            }
        } else if offset != 0 && (offset < 8 || offset as usize > vdata_len) {
            return None;
        }
        has_any = true;
    }

    if has_any {
        return Some(ColumnClaim {
            offset: col_index,
            bytes: 16,
            column_type: Cell::Array(Scalar::Unknown),
            labels: HashMap::new(),
        });
    }

    None
}
