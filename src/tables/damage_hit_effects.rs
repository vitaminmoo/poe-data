#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DamageHitEffects: LazyLock<Vec<DamageHitEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/damagehiteffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DamageHitEffectsRow {
            r#id: {
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
                // array_mutator column.array == true
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown28: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DamageHitEffectsRow {
    pub r#id: i32,
    pub r#unknown4: i32,
    pub r#unknown8: i32,
    pub r#unknown12: (usize, usize),
    pub r#unknown28: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DamageHitEffectsRef(pub usize);

impl Deref for DamageHitEffectsRef {
    type Target = DamageHitEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DamageHitEffects[self.0]
    }
}

impl DamageHitEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DamageHitEffectsRow {
        &TABLE_DamageHitEffects[self.0]
    }
    pub fn get(&self) -> &'static DamageHitEffectsRow {
        &TABLE_DamageHitEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DamageHitEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DamageHitEffectsRow)> {
        TABLE_DamageHitEffects
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
        for row in TABLE_DamageHitEffects.iter() {
            black_box(row);
        }
    }
}
