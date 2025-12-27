use crate::dat_parser::DatFile;
use crate::types::{Cell, ColumnClaim, Scalar, TypeSet};
use std::collections::HashMap;

pub fn scan(_dat_file: &DatFile, col_index: usize, candidates: &TypeSet) -> Vec<ColumnClaim> {
    let mut claims = Vec::new();

    // If validator confirmed it's a File, emit File claim
    if candidates.contains(Scalar::File) {
        claims.push(ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::File),
            labels: HashMap::new(),
        });
    }

    // If validator confirmed it's a Directory, emit Directory claim
    if candidates.contains(Scalar::Directory) {
        claims.push(ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::Directory),
            labels: HashMap::new(),
        });
    }

    // If validator confirmed it's a Color, emit Color claim
    if candidates.contains(Scalar::Color) {
        claims.push(ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::Color),
            labels: HashMap::new(),
        });
    }

    // If validator confirmed it's a String, emit String claim
    // This is the fallback if specific types aren't picked by resolve_conflicts,
    // or if only String is in candidates.
    if candidates.contains(Scalar::String) {
        claims.push(ColumnClaim {
            offset: col_index,
            bytes: 8,
            column_type: Cell::Scalar(Scalar::String),
            labels: HashMap::new(),
        });
    }

    claims
}
