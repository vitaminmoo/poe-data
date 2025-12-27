use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use std::collections::HashMap;

pub fn scan(dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    if dat_file.row_count == 0 {
        return None;
    }

    // Validator has already confirmed basic structure (count <= 50, valid offsets).
    Some(ColumnClaim {
        offset: col_index,
        bytes: 16,
        column_type: Cell::Array(Scalar::Unknown),
        labels: HashMap::new(),
    })
}
