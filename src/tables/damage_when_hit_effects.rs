#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DamageWhenHitEffects: LazyLock<Vec<DamageWhenHitEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/damagewhenhiteffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DamageWhenHitEffectsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DamageWhenHitEffectsRow {
    pub r#unknown0: i64,
    pub r#unknown16: i64,
    pub r#unknown32: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DamageWhenHitEffectsRef(pub usize);

impl Deref for DamageWhenHitEffectsRef {
    type Target = DamageWhenHitEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DamageWhenHitEffects[self.0]
    }
}

impl DamageWhenHitEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DamageWhenHitEffectsRow {
        &TABLE_DamageWhenHitEffects[self.0]
    }
    pub fn get(&self) -> &'static DamageWhenHitEffectsRow {
        &TABLE_DamageWhenHitEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DamageWhenHitEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DamageWhenHitEffectsRow)> {
        TABLE_DamageWhenHitEffects.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_DamageWhenHitEffects.iter() {
            black_box(row);
        }
    }
}
