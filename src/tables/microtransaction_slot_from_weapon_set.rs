#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionSlotFromWeaponSet: LazyLock<Vec<MicrotransactionSlotFromWeaponSetRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionslotfromweaponset.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionSlotFromWeaponSetRow {
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
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionSlotFromWeaponSetRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionSlotFromWeaponSetRef(pub usize);

impl Deref for MicrotransactionSlotFromWeaponSetRef {
    type Target = MicrotransactionSlotFromWeaponSetRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionSlotFromWeaponSet[self.0]
    }
}

impl MicrotransactionSlotFromWeaponSetRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionSlotFromWeaponSetRow {
        &TABLE_MicrotransactionSlotFromWeaponSet[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionSlotFromWeaponSetRow {
        &TABLE_MicrotransactionSlotFromWeaponSet[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionSlotFromWeaponSet.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionSlotFromWeaponSetRow)> {
        TABLE_MicrotransactionSlotFromWeaponSet.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionSlotFromWeaponSet.iter() {
            black_box(row);
        }
    }
}
