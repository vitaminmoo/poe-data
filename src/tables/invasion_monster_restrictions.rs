#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_InvasionMonsterRestrictions: LazyLock<Vec<InvasionMonsterRestrictionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/invasionmonsterrestrictions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| InvasionMonsterRestrictionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#world_areas_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#monster_varieties_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MonsterVarietiesRef::new(value as usize)).collect()
            },
            r#unknown40: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct InvasionMonsterRestrictionsRow {
    pub r#id: String,
    pub r#world_areas_key: WorldAreasRef,
    pub r#monster_varieties_keys: Vec<MonsterVarietiesRef>,
    pub r#unknown40: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct InvasionMonsterRestrictionsRef(pub usize);

impl Deref for InvasionMonsterRestrictionsRef {
    type Target = InvasionMonsterRestrictionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_InvasionMonsterRestrictions[self.0]
    }
}

impl InvasionMonsterRestrictionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static InvasionMonsterRestrictionsRow {
        &TABLE_InvasionMonsterRestrictions[self.0]
    }
    pub fn get(&self) -> &'static InvasionMonsterRestrictionsRow {
        &TABLE_InvasionMonsterRestrictions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_InvasionMonsterRestrictions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static InvasionMonsterRestrictionsRow)> {
        TABLE_InvasionMonsterRestrictions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_InvasionMonsterRestrictions.iter() {
            black_box(row);
        }
    }
}
