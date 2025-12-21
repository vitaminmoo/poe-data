#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ClassPassiveSkillOverrides: LazyLock<Vec<ClassPassiveSkillOverridesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/classpassiveskilloverrides.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| ClassPassiveSkillOverridesRow {
                r#character_to_override_for: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    CharactersRef::new(value as usize)
                },
                r#skill_to_override: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    PassiveSkillsRef::new(value as usize)
                },
                r#override: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    PassiveSkillsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct ClassPassiveSkillOverridesRow {
    pub r#character_to_override_for: CharactersRef,
    pub r#skill_to_override: PassiveSkillsRef,
    pub r#override: PassiveSkillsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ClassPassiveSkillOverridesRef(pub usize);

impl Deref for ClassPassiveSkillOverridesRef {
    type Target = ClassPassiveSkillOverridesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ClassPassiveSkillOverrides[self.0]
    }
}

impl ClassPassiveSkillOverridesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ClassPassiveSkillOverridesRow {
        &TABLE_ClassPassiveSkillOverrides[self.0]
    }
    pub fn get(&self) -> &'static ClassPassiveSkillOverridesRow {
        &TABLE_ClassPassiveSkillOverrides[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ClassPassiveSkillOverrides
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ClassPassiveSkillOverridesRow)>
    {
        TABLE_ClassPassiveSkillOverrides
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
        for row in TABLE_ClassPassiveSkillOverrides.iter() {
            black_box(row);
        }
    }
}
