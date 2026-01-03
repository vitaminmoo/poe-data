use crate::dat_parser::DatFile;
use crate::scanners;
use crate::types::{Cell, ColumnClaim, Scalar, TypeSet, ValidationError};

pub fn check_phase_1_absolutes(
    dat_file: &DatFile,
    col_index: usize,
    cell_length: usize,
    known_files: Option<&[String]>,
) -> (TypeSet, bool, Vec<(Scalar, ValidationError)>) {
    let mut candidates = TypeSet::from_size(cell_length);
    let mut is_array_candidate = cell_length == 16;
    let mut failures = Vec::new();

    match cell_length {
        1 => {
            if let Err(e) = crate::validators::is_valid_bool(dat_file, col_index) {
                candidates.remove(Scalar::Bool);
                failures.push((Scalar::Bool, e));
            }
        }
        2 => {
            if let Err(e) = crate::validators::is_valid_hash16(dat_file, col_index) {
                candidates.remove(Scalar::Hash16);
                failures.push((Scalar::Hash16, e));
            }
        }
        4 => {
            if let Err(e) = crate::validators::is_valid_hash32(dat_file, col_index) {
                candidates.remove(Scalar::Hash32);
                failures.push((Scalar::Hash32, e));
            }
        }
        8 => {
            if let Err(e) = crate::validators::is_valid_string_pointer(dat_file, col_index) {
                candidates.remove(Scalar::String);
                candidates.remove(Scalar::File);
                candidates.remove(Scalar::Directory);
                candidates.remove(Scalar::Color);
                failures.push((Scalar::String, e.clone()));
                failures.push((Scalar::File, e.clone()));
                failures.push((Scalar::Directory, e.clone()));
                failures.push((Scalar::Color, e));
            } else {
                if let Err(e) = crate::validators::is_valid_file_path(dat_file, col_index, known_files) {
                    candidates.remove(Scalar::File);
                    failures.push((Scalar::File, e));
                }
                if let Err(e) = crate::validators::is_valid_directory_path(dat_file, col_index, known_files) {
                    candidates.remove(Scalar::Directory);
                    failures.push((Scalar::Directory, e));
                }
                if let Err(e) = crate::validators::is_valid_color(dat_file, col_index) {
                    candidates.remove(Scalar::Color);
                    failures.push((Scalar::Color, e));
                }
            }
        }
        16 => {
            if let Err(_e) = crate::validators::is_valid_array(dat_file, col_index) {
                is_array_candidate = false;
                // Since 'Array' isn't a Scalar type in our TypeSet usually (it's a Cell type),
                // we might need to represent this error differently or just ignore if we can't map to Scalar.
                // But for now, let's assume we don't track Array errors in 'failures' mapped to Scalar.
                // Wait, users might want to know WHY it's not an array.
                // We can use a dummy scalar or just add it to failures with a special key if we had one.
                // But ColumnValidation has failures: Vec<(Scalar, ValidationError)>.
                // We can hack it or add Scalar::Array? Scalar has no Array.
                // Let's just log it if possible or ignore for now as 'is_array' bool tells us status.
            }
        }
        _ => {}
    }

    (candidates, is_array_candidate, failures)
}

pub fn get_column_claims(dat_file: &DatFile, col_index: usize, cell_length: usize, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
    let (candidates, is_array_candidate, _) = check_phase_1_absolutes(dat_file, col_index, cell_length, known_files);

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
                claims.extend(scanners::strings::scan(dat_file, col_index, &candidates));
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
            if candidates.contains(Scalar::SelfRow) {
                if let Some(claim) = scanners::references::scan_row(dat_file, col_index) {
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
        1 => {
            if candidates.contains(Scalar::Bool) {
                if let Some(claim) = scanners::bool::scan(dat_file, col_index) {
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
    for &size in &[16, 8, 4, 2, 1] {
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
            Cell::Scalar(Scalar::String) => 85,
            Cell::Scalar(Scalar::ForeignRow) | Cell::Scalar(Scalar::SelfRow) => 80,
            Cell::Scalar(Scalar::DateTime) => 60,
            Cell::Scalar(Scalar::Hash32) => 50,
            Cell::Scalar(Scalar::Hash16) => 40,
            Cell::Scalar(Scalar::Bool) => 10,
            Cell::Scalar(Scalar::Interval) => 5,
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

pub fn validate_file_types(dat_file: &DatFile, known_files: Option<&[String]>) -> Vec<crate::types::ColumnValidation> {
    let mut validations = Vec::new();
    let row_len = dat_file.bytes_per_row;

    // Check for standard sizes: 1, 2, 4, 8, 16.
    for &length in &[1, 2, 4, 8, 16] {
        if row_len < length {
            continue;
        }
        for offset in 0..=(row_len - length) {
            let (candidates, is_array, failures) = check_phase_1_absolutes(dat_file, offset, length, known_files);
            let allowed_types: Vec<Scalar> = candidates.iter().collect();

            validations.push(crate::types::ColumnValidation {
                offset,
                length,
                allowed_types,
                is_array,
                failures,
            });
        }
    }
    validations
}
