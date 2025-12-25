use crate::dat_parser::DatFile;
use crate::scanners;
use crate::types::{Cell, ColumnClaim, Scalar, TypeSet};

pub fn check_phase_1_absolutes(dat_file: &DatFile, col_index: usize, cell_length: usize) -> (TypeSet, bool) {
    let mut candidates = TypeSet::from_size(cell_length);
    let mut is_array_candidate = cell_length == 16;
    let vdata_len = dat_file.vdata.len();

    // Zero check optimization
    let mut all_zeros = true;
    for i in 0..cell_length {
        if dat_file.stats.per_byte_stats[col_index + i].max_value != 0 {
            all_zeros = false;
            break;
        }
    }
    if all_zeros {
        return (candidates, is_array_candidate);
    }

    match cell_length {
        1 => {
            if dat_file.stats.per_byte_stats[col_index].max_value > 1 {
                candidates.remove(Scalar::Bool);
            }
        }
        8 => {
            let mut possible_ptr = true;
            let rows_iter = dat_file.column_rows_iter(col_index, 8);

            for mut row in rows_iter {
                let val = bytes::Buf::get_u64_le(&mut row);
                if val == 0xFEFEFEFEFEFEFEFE {
                    continue;
                }
                if val == 0 {
                    possible_ptr = false;
                    break;
                }
                if val < 8 || val as usize >= vdata_len {
                    possible_ptr = false;
                    break;
                }
                if dat_file.string_from_offset_if_valid(val as usize).is_err() {
                    possible_ptr = false;
                    break;
                }
            }

            if !possible_ptr {
                candidates.remove(Scalar::String);
                candidates.remove(Scalar::File);
                candidates.remove(Scalar::Directory);
                candidates.remove(Scalar::Color);
            }
        }
        16 => {
            let rows_iter = dat_file.column_rows_iter(col_index, 16);
            for mut row in rows_iter {
                let count = bytes::Buf::get_u64_le(&mut row);
                let offset = bytes::Buf::get_u64_le(&mut row);

                if count == 0xFEFEFEFEFEFEFEFE && offset == 0xFEFEFEFEFEFEFEFE {
                    is_array_candidate = false;
                    break;
                }

                if count > 100_000 {
                    is_array_candidate = false;
                    break;
                }

                if count > 0 {
                    if offset < 8 || offset as usize > vdata_len {
                        is_array_candidate = false;
                        break;
                    }
                } else if offset != 0 && (offset < 8 || offset as usize > vdata_len) {
                    is_array_candidate = false;
                    break;
                }
            }
        }
        _ => {}
    }

    (candidates, is_array_candidate)
}

pub fn get_column_claims(dat_file: &DatFile, col_index: usize, cell_length: usize, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
    let (candidates, is_array_candidate) = check_phase_1_absolutes(dat_file, col_index, cell_length);

    if candidates.is_empty() && !is_array_candidate {
        return Vec::new();
    }

    let mut claims = Vec::new();

    match cell_length {
        2 => {
            if candidates.contains(Scalar::Hash16) {
                if let Some(claim) = scanners::hashes::scan_hash16(dat_file, col_index) {
                    claims.push(claim);
                }
            }
        }
        4 => {
            if candidates.contains(Scalar::Hash32) {
                if let Some(claim) = scanners::hashes::scan_hash32(dat_file, col_index) {
                    claims.push(claim);
                }
            }
        }
        8 => {
            if candidates.contains(Scalar::String) {
                claims.extend(scanners::strings::scan(dat_file, col_index, known_files));
            }
            if candidates.contains(Scalar::Interval) {
                if let Some(claim) = scanners::interval::scan(dat_file, col_index) {
                    claims.push(claim);
                }
            }
            if candidates.contains(Scalar::DateTime) {
                if let Some(claim) = scanners::datetime::scan(dat_file, col_index) {
                    claims.push(claim);
                }
            }
        }
        16 => {
            if is_array_candidate {
                if let Some(claim) = scanners::arrays::scan(dat_file, col_index) {
                    claims.push(claim);
                }
            }
            if candidates.contains(Scalar::ForeignRow) {
                if let Some(claim) = scanners::references::scan_foreign_row(dat_file, col_index) {
                    claims.push(claim);
                }
            }
        }
        _ => {}
    }

    claims
}

pub fn get_all_column_claims(dat_file: &DatFile, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
    let mut all_claims = Vec::new();
    for &size in &[16, 8, 4, 2] {
        if dat_file.bytes_per_row < size {
            continue;
        }
        for offset in 0..=(dat_file.bytes_per_row - size) {
            let claims = get_column_claims(dat_file, offset, size, known_files);
            all_claims.extend(claims);
        }
    }
    resolve_conflicts(dat_file, all_claims)
}

pub fn resolve_conflicts(dat_file: &DatFile, mut claims: Vec<ColumnClaim>) -> Vec<ColumnClaim> {
    fn get_score(c: &ColumnClaim) -> i32 {
        match c.column_type {
            Cell::Scalar(Scalar::File) | Cell::Scalar(Scalar::Directory) | Cell::Scalar(Scalar::Color) => 100,
            Cell::Array(_) => 90,
            Cell::Scalar(Scalar::ForeignRow) => 80,
            Cell::Scalar(Scalar::String) => 75,
            Cell::Scalar(Scalar::Interval) | Cell::Scalar(Scalar::DateTime) => 60,
            Cell::Scalar(Scalar::Hash32) => 50,
            Cell::Scalar(Scalar::Hash16) => 40,
            Cell::Scalar(Scalar::Bool) => 10,
            _ => 5,
        }
    }

    claims.sort_by(|a, b| {
        let score_a = get_score(a);
        let score_b = get_score(b);
        score_b.cmp(&score_a).then(a.offset.cmp(&b.offset)).then(b.bytes.cmp(&a.bytes))
    });

    let mut accepted = Vec::new();
    let mut occupied = vec![false; dat_file.bytes_per_row];

    for claim in claims {
        let start = claim.offset;
        let end = start + claim.bytes;
        let overlaps = occupied[start..end].iter().any(|&occupied| occupied);

        if !overlaps {
            occupied[start..end].iter_mut().for_each(|o| *o = true);
            accepted.push(claim);
        }
    }

    accepted.sort_by_key(|c| c.offset);
    accepted
}
