#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillMineVariations: LazyLock<Vec<SkillMineVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/skillminevariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SkillMineVariationsRow {
            r#skill_mines_key: {
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
            r#misc_object: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscObjectsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SkillMineVariationsRow {
    pub r#skill_mines_key: i32,
    pub r#unknown4: i32,
    pub r#misc_object: MiscObjectsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillMineVariationsRef(pub usize);

impl Deref for SkillMineVariationsRef {
    type Target = SkillMineVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillMineVariations[self.0]
    }
}

impl SkillMineVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillMineVariationsRow {
        &TABLE_SkillMineVariations[self.0]
    }
    pub fn get(&self) -> &'static SkillMineVariationsRow {
        &TABLE_SkillMineVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillMineVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillMineVariationsRow)> {
        TABLE_SkillMineVariations
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
        for row in TABLE_SkillMineVariations.iter() {
            black_box(row);
        }
    }
}
