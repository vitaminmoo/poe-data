#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterProjectileSpell: LazyLock<Vec<MonsterProjectileSpellRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/monsterprojectilespell.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MonsterProjectileSpellRow {
                r#id: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#projectile: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ProjectilesRef::new(value as usize)
                },
                r#animation: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    AnimationRef::new(value as usize)
                },
                r#unknown36: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(36).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown37: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(37..37 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown41: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(41).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MonsterProjectileSpellRow {
    pub r#id: i32,
    pub r#projectile: ProjectilesRef,
    pub r#animation: AnimationRef,
    pub r#unknown36: bool,
    pub r#unknown37: i32,
    pub r#unknown41: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterProjectileSpellRef(pub usize);

impl Deref for MonsterProjectileSpellRef {
    type Target = MonsterProjectileSpellRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterProjectileSpell[self.0]
    }
}

impl MonsterProjectileSpellRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterProjectileSpellRow {
        &TABLE_MonsterProjectileSpell[self.0]
    }
    pub fn get(&self) -> &'static MonsterProjectileSpellRow {
        &TABLE_MonsterProjectileSpell[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterProjectileSpell
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterProjectileSpellRow)> {
        TABLE_MonsterProjectileSpell
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
        for row in TABLE_MonsterProjectileSpell.iter() {
            black_box(row);
        }
    }
}
