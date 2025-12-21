#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterProjectileAttack: LazyLock<Vec<MonsterProjectileAttackRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/monsterprojectileattack.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MonsterProjectileAttackRow {
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
                r#unknown20: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(20).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown21: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(21).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown22: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(22).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown23: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(23..23 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MonsterProjectileAttackRow {
    pub r#id: i32,
    pub r#projectile: ProjectilesRef,
    pub r#unknown20: bool,
    pub r#unknown21: bool,
    pub r#unknown22: bool,
    pub r#unknown23: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterProjectileAttackRef(pub usize);

impl Deref for MonsterProjectileAttackRef {
    type Target = MonsterProjectileAttackRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterProjectileAttack[self.0]
    }
}

impl MonsterProjectileAttackRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterProjectileAttackRow {
        &TABLE_MonsterProjectileAttack[self.0]
    }
    pub fn get(&self) -> &'static MonsterProjectileAttackRow {
        &TABLE_MonsterProjectileAttack[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterProjectileAttack
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterProjectileAttackRow)> {
        TABLE_MonsterProjectileAttack
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
        for row in TABLE_MonsterProjectileAttack.iter() {
            black_box(row);
        }
    }
}
