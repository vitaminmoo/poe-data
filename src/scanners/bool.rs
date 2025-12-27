use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar};
use std::collections::HashMap;

pub fn scan(_dat_file: &DatFile, col_index: usize) -> Option<ColumnClaim> {
    // If we are here, validators have already confirmed it's a valid bool column (0/1).
    Some(ColumnClaim {
        offset: col_index,
        bytes: 1,
        column_type: Cell::Scalar(Scalar::Bool),
        labels: HashMap::new(),
    })
}
