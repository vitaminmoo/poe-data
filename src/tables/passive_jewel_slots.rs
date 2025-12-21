#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveJewelSlots: LazyLock<Vec<PassiveJewelSlotsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passivejewelslots.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveJewelSlotsRow {
            r#slot: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillsRef::new(value as usize)
            },
            r#cluster_jewel_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveTreeExpansionJewelSizesRef::new(value as usize)
            },
            r#cluster_index: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#replaces_slot: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveJewelSlotsRef::new(value as usize)
            },
            r#proxy_slot: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillsRef::new(value as usize)
            },
            r#start_indices: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveJewelSlotsRow {
    pub r#slot: PassiveSkillsRef,
    pub r#cluster_jewel_size: PassiveTreeExpansionJewelSizesRef,
    pub r#cluster_index: i32,
    pub r#replaces_slot: PassiveJewelSlotsRef,
    pub r#proxy_slot: PassiveSkillsRef,
    pub r#start_indices: Vec<i32>,
    pub r#unknown76: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveJewelSlotsRef(pub usize);

impl Deref for PassiveJewelSlotsRef {
    type Target = PassiveJewelSlotsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveJewelSlots[self.0]
    }
}

impl PassiveJewelSlotsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveJewelSlotsRow {
        &TABLE_PassiveJewelSlots[self.0]
    }
    pub fn get(&self) -> &'static PassiveJewelSlotsRow {
        &TABLE_PassiveJewelSlots[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveJewelSlots
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveJewelSlotsRow)> {
        TABLE_PassiveJewelSlots
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
        for row in TABLE_PassiveJewelSlots.iter() {
            black_box(row);
        }
    }
}
