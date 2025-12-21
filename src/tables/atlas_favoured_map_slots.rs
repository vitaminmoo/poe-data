#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasFavouredMapSlots: LazyLock<Vec<AtlasFavouredMapSlotsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/atlasfavouredmapslots.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AtlasFavouredMapSlotsRow {
                r#unknown0: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown4: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#requirement: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(8..8 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AtlasFavouredMapSlotsRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#requirement: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasFavouredMapSlotsRef(pub usize);

impl Deref for AtlasFavouredMapSlotsRef {
    type Target = AtlasFavouredMapSlotsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasFavouredMapSlots[self.0]
    }
}

impl AtlasFavouredMapSlotsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasFavouredMapSlotsRow {
        &TABLE_AtlasFavouredMapSlots[self.0]
    }
    pub fn get(&self) -> &'static AtlasFavouredMapSlotsRow {
        &TABLE_AtlasFavouredMapSlots[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasFavouredMapSlots
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasFavouredMapSlotsRow)> {
        TABLE_AtlasFavouredMapSlots
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
        for row in TABLE_AtlasFavouredMapSlots.iter() {
            black_box(row);
        }
    }
}
