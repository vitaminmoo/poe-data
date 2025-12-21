#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ModGrantedSkills: LazyLock<Vec<ModGrantedSkillsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/modgrantedskills.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ModGrantedSkillsRow {
            r#mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#skill: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SkillGemsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ModGrantedSkillsRow {
    pub r#mod: ModsRef,
    pub r#skill: SkillGemsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModGrantedSkillsRef(pub usize);

impl Deref for ModGrantedSkillsRef {
    type Target = ModGrantedSkillsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ModGrantedSkills[self.0]
    }
}

impl ModGrantedSkillsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModGrantedSkillsRow {
        &TABLE_ModGrantedSkills[self.0]
    }
    pub fn get(&self) -> &'static ModGrantedSkillsRow {
        &TABLE_ModGrantedSkills[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ModGrantedSkills
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModGrantedSkillsRow)> {
        TABLE_ModGrantedSkills
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
        for row in TABLE_ModGrantedSkills.iter() {
            black_box(row);
        }
    }
}
