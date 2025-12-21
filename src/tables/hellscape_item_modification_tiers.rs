#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HellscapeItemModificationTiers: LazyLock<Vec<HellscapeItemModificationTiersRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/hellscapeitemmodificationtiers.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| HellscapeItemModificationTiersRow {
                r#tier: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#is_map: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(4).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown5: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(5..5 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#min_item_lvl: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(9..9 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct HellscapeItemModificationTiersRow {
    pub r#tier: i32,
    pub r#is_map: bool,
    pub r#unknown5: i32,
    pub r#min_item_lvl: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HellscapeItemModificationTiersRef(pub usize);

impl Deref for HellscapeItemModificationTiersRef {
    type Target = HellscapeItemModificationTiersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HellscapeItemModificationTiers[self.0]
    }
}

impl HellscapeItemModificationTiersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HellscapeItemModificationTiersRow {
        &TABLE_HellscapeItemModificationTiers[self.0]
    }
    pub fn get(&self) -> &'static HellscapeItemModificationTiersRow {
        &TABLE_HellscapeItemModificationTiers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HellscapeItemModificationTiers
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static HellscapeItemModificationTiersRow)> {
        TABLE_HellscapeItemModificationTiers
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
        for row in TABLE_HellscapeItemModificationTiers.iter() {
            black_box(row);
        }
    }
}
