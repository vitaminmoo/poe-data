#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HarvestInfrastructure: LazyLock<Vec<HarvestInfrastructureRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/harvestinfrastructure.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| HarvestInfrastructureRow {
                r#object: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#client_strings_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct HarvestInfrastructureRow {
    pub r#object: String,
    pub r#unknown8: i32,
    pub r#client_strings_key: ClientStringsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HarvestInfrastructureRef(pub usize);

impl Deref for HarvestInfrastructureRef {
    type Target = HarvestInfrastructureRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HarvestInfrastructure[self.0]
    }
}

impl HarvestInfrastructureRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HarvestInfrastructureRow {
        &TABLE_HarvestInfrastructure[self.0]
    }
    pub fn get(&self) -> &'static HarvestInfrastructureRow {
        &TABLE_HarvestInfrastructure[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HarvestInfrastructure
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HarvestInfrastructureRow)> {
        TABLE_HarvestInfrastructure
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HarvestInfrastructure.iter() {
            black_box(row);
        }
    }
}
