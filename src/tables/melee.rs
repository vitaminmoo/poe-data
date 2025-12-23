#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Melee: LazyLock<Vec<MeleeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/melee.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MeleeRow {
            r#active_skill: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ActiveSkillsRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#misc_animated: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#melee_trails_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MeleeTrailsRef::new(value as usize)
            },
            r#melee_trails_key2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MeleeTrailsRef::new(value as usize)
            },
            r#melee_trails_key3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MeleeTrailsRef::new(value as usize)
            },
            r#melee_trails_key4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MeleeTrailsRef::new(value as usize)
            },
            r#melee_trails_key5: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MeleeTrailsRef::new(value as usize)
            },
            r#melee_trails_key6: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MeleeTrailsRef::new(value as usize)
            },
            r#melee_trails_key7: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MeleeTrailsRef::new(value as usize)
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(148).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#surge_effect_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(149..149 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown157: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(157..157 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown165: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(165..165 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MeleeRow {
    pub r#active_skill: ActiveSkillsRef,
    pub r#unknown16: i32,
    pub r#misc_animated: MiscAnimatedRef,
    pub r#melee_trails_key1: MeleeTrailsRef,
    pub r#melee_trails_key2: MeleeTrailsRef,
    pub r#melee_trails_key3: MeleeTrailsRef,
    pub r#melee_trails_key4: MeleeTrailsRef,
    pub r#melee_trails_key5: MeleeTrailsRef,
    pub r#melee_trails_key6: MeleeTrailsRef,
    pub r#melee_trails_key7: MeleeTrailsRef,
    pub r#unknown148: bool,
    pub r#surge_effect_epk_file: String,
    pub r#unknown157: String,
    pub r#unknown165: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MeleeRef(pub usize);

impl Deref for MeleeRef {
    type Target = MeleeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Melee[self.0]
    }
}

impl MeleeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MeleeRow {
        &TABLE_Melee[self.0]
    }
    pub fn get(&self) -> &'static MeleeRow {
        &TABLE_Melee[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Melee.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MeleeRow)> {
        TABLE_Melee.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Melee.iter() {
            black_box(row);
        }
    }
}
