use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use std::collections::HashMap;

pub fn scan(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    let rows_iter = dat_file.column_rows_iter(col_index, 1);

    // Check every row
    for row in rows_iter {
        // Since we are iterating with size 1, row is just 1 byte
        let val = row[0].to_le();
        if val != 0 && val != 1 {
            return None;
        }
    }

    Some(ColumnClaim {
        offset: col_index,
        bytes: 1,
        column_type: Cell::Scalar(Scalar::Bool),
        labels: HashMap::new(),
    })
}
